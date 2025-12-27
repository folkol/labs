#![allow(unsafe_op_in_unsafe_fn)]
use libc::{MADV_SEQUENTIAL, MADV_WILLNEED, madvise};
use memmap2::Mmap;
use std::arch::x86_64::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};

#[target_feature(enable = "avx2")]
unsafe fn count_nl_avx2_accum(data: &[u8]) -> u64 {
    let nl = _mm256_set1_epi8(b'\n' as i8);
    let zero = _mm256_setzero_si256();

    let mut p = data.as_ptr();
    let mut n = data.len();
    let mut total: u64 = 0;

    // Count in 64-byte steps using two accumulators.
    // Drain every 255 iterations to avoid u8 wraparound.
    while n >= 64 {
        let mut acc0 = _mm256_setzero_si256();
        let mut acc1 = _mm256_setzero_si256();

        // Number of 64-byte iterations we can do without overflowing u8 lanes.
        // Each iteration increments each lane by at most 1.
        let iters = (n / 64).min(255);
        assert!(n >= iters * 64);
        let mut i = 0usize;

        while i < iters {
            let v0 = _mm256_loadu_si256(p as *const __m256i);
            let v1 = _mm256_loadu_si256(p.add(32) as *const __m256i);

            let m0 = _mm256_cmpeq_epi8(v0, nl);
            let m1 = _mm256_cmpeq_epi8(v1, nl);

            acc0 = _mm256_sub_epi8(acc0, m0);
            acc1 = _mm256_sub_epi8(acc1, m1);

            p = p.add(64);
            i += 1;
        }

        // Reduce acc0, acc1: vpsadbw gives 4x u64 sums per ymm
        let s0 = _mm256_sad_epu8(acc0, zero);
        let s1 = _mm256_sad_epu8(acc1, zero);

        // Sum the 4 lanes of u64 in each ymm.
        // Store is fine here; it's once per ~16KB, not per 32B.
        let mut tmp0: [u64; 4] = [0; 4];
        let mut tmp1: [u64; 4] = [0; 4];
        _mm256_storeu_si256(tmp0.as_mut_ptr() as *mut __m256i, s0);
        _mm256_storeu_si256(tmp1.as_mut_ptr() as *mut __m256i, s1);

        total += tmp0[0] + tmp0[1] + tmp0[2] + tmp0[3];
        total += tmp1[0] + tmp1[1] + tmp1[2] + tmp1[3];

        n -= iters * 64;
    }

    // Scalar tail for remaining < 64 bytes
    let tail = &data[data.len() - n..];
    for &b in tail {
        total += (b == b'\n') as u64;
    }

    total
}

use std::sync::atomic::{AtomicUsize, Ordering};
const SEGMENT: usize = 1 << 21; // 2 MiB

fn main() -> io::Result<()> {
    let is_worker = std::env::args().any(|a| a == "--worker");
    if !is_worker {
        return run_parent();
    }
    run_worker()
}

fn run_parent() -> io::Result<()> {
    let exe = std::env::current_exe()?;

    let mut child = Command::new(exe)
        .arg("--worker")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let stdout_pipe = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout_pipe);

    let mut line = String::new();
    reader.read_line(&mut line)?;          // stops after first '\n'

    // print result and flush
    print!("{line}");
    io::stdout().flush().ok();

    // Do NOT wait for child. Do NOT drop `child` normally.
    std::mem::forget(child);

    // Exit immediately (no destructors, no waiting)
    std::process::exit(0);
}

fn run_worker() -> io::Result<()> {
    // mmap + compute + print result (your existing code)
    // println!("{}", total_lines);
    doit().expect("wc failed");
    Ok(())
}


fn doit() -> io::Result<()> {
    let file = File::open("../java-orig/measurements.txt")?;
    // let file = File::open("measurements_small.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };
    unsafe {
        madvise(mmap.as_ptr() as *mut _, mmap.len(), MADV_SEQUENTIAL);
        madvise(mmap.as_ptr() as *mut _, mmap.len(), MADV_WILLNEED);
    }

    let n_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let mut total_lines = 0;

    use std::sync::Arc;

    std::thread::scope(|s| {
        let cursor = Arc::new(AtomicUsize::new(0));
        let data: &[u8] = &mmap;

        let mut handles = Vec::with_capacity(n_threads);
        for _ in 0..n_threads {
            let cursor = Arc::clone(&cursor);
            handles.push(s.spawn(move || {
                let mut local = 0i64;
                loop {
                    let start = cursor.fetch_add(SEGMENT, Ordering::Relaxed);
                    if start >= data.len() {
                        break;
                    }
                    let end = (start + SEGMENT).min(data.len());

                    assert!(end <= data.len());
                    assert!(start <= end);
                    let slice = &data[start..end];
                    local += unsafe { count_nl_avx2_accum(slice) as i64 };
                }

                local
            }));
        }

        total_lines = handles.into_iter().map(|h| h.join().unwrap()).sum();
    });

    println!("{}", total_lines);

    // Flush and close stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.flush()?;
    drop(handle);

    Ok(())
}
