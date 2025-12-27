#![allow(unsafe_op_in_unsafe_fn)]
use libc::{MADV_SEQUENTIAL, MADV_WILLNEED, madvise};
use memmap2::Mmap;
use std::arch::x86_64::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};

// #[target_feature(enable = "avx2")]
// unsafe fn count_nl_avx2_accum(data: &[u8]) -> u64 {
//     let nl = _mm256_set1_epi8(b'\n' as i8);
//     let zero = _mm256_setzero_si256();
//
//     let mut p = data.as_ptr();
//     let mut n = data.len();
//     let mut total: u64 = 0;
//
//     // Count in 64-byte steps using two accumulators.
//     // Drain every 255 iterations to avoid u8 wraparound.
//     while n >= 64 {
//         let mut acc0 = _mm256_setzero_si256();
//         // let mut acc1 = _mm256_setzero_si256();
//
//         // Number of 64-byte iterations we can do without overflowing u8 lanes.
//         // Each iteration increments each lane by at most 1.
//         let iters = (n / 32).min(255);
//         assert!(n >= iters * 32);
//         let mut i = 0usize;
//
//         while i < iters {
//             let v0 = _mm256_loadu_si256(p as *const __m256i);
//             // let v1 = _mm256_loadu_si256(p.add(32) as *const __m256i);
//
//             let m0 = _mm256_cmpeq_epi8(v0, nl);
//             // let m1 = _mm256_cmpeq_epi8(v1, nl);
//
//             acc0 = _mm256_sub_epi8(acc0, m0);
//             // acc1 = _mm256_sub_epi8(acc1, m1);
//
//             p = p.add(32);
//             i += 1;
//         }
//
//         // Reduce acc0, acc1: vpsadbw gives 4x u64 sums per ymm
//         let s0 = _mm256_sad_epu8(acc0, zero);
//         // let s1 = _mm256_sad_epu8(acc1, zero);
//
//         // Sum the 4 lanes of u64 in each ymm.
//         // Store is fine here; it's once per ~16KB, not per 32B.
//         let mut tmp0: [u64; 4] = [0; 4];
//         // let mut tmp1: [u64; 4] = [0; 4];
//         _mm256_storeu_si256(tmp0.as_mut_ptr() as *mut __m256i, s0);
//         // _mm256_storeu_si256(tmp1.as_mut_ptr() as *mut __m256i, s1);
//
//         total += tmp0[0] + tmp0[1] + tmp0[2] + tmp0[3];
//         // total += tmp1[0] + tmp1[1] + tmp1[2] + tmp1[3];
//
//         n -= iters * 32;
//     }
//
//     // Scalar tail for remaining < 64 bytes
//     let tail = &data[data.len() - n..];
//     for &b in tail {
//         total += (b == b'\n') as u64;
//     }
//
//     total
// }

#[target_feature(enable = "avx2")]
unsafe fn wc_lines_avx2(data: &[u8]) -> i64 {
    let mut lines: i64 = 0;

    let newlines = _mm256_set1_epi8(b'\n' as i8);

    let mut datap = data.as_ptr();
    let mut rem = data.len();

    while rem >= 64 {
        let v0 = _mm256_loadu_si256(datap as *const __m256i);
        let m0 = _mm256_cmpeq_epi8(v0, newlines);
        lines += (_mm256_movemask_epi8(m0).count_ones()) as i64;
        datap = datap.add(32);
        rem -= 32;
    }

    // scalar tail
    for i in 0..rem {
        if *datap.add(i) == b'\n' {
            lines += 1;
        }
    }

    lines
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

    let child = Command::new(exe)
        .arg("--worker")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let out = child.stdout.unwrap();
    let mut reader = BufReader::new(out);

    let mut line = String::new();
    reader.read_line(&mut line)?;
    // reader.read_to_string(&mut line)?;

    print!("{line}");
    // let _ = io::stdout().flush();

    // std::process::exit(0);
    Ok(())
}

fn run_worker() -> io::Result<()> {
    // mmap + compute + print result (your existing code)
    // println!("{}", total_lines);
    doit().expect("wc failed");
    let _ = io::stdout().flush();
    // unsafe {
    //     libc::close(libc::STDOUT_FILENO);
    // }
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
                    local += unsafe { wc_lines_avx2(slice) as i64 };
                }

                local
            }));
        }

        total_lines = handles.into_iter().map(|h| h.join().unwrap()).sum();
    });

    println!("{}", total_lines);
    Ok(())
}
