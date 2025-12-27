#![allow(unsafe_op_in_unsafe_fn)]
use std::arch::x86_64::*;
use std::fs::File;
use std::io;
use memmap2::Mmap;

#[derive(Debug)]
struct WcLines {
    lines: i64,
    bytes: i64,
}

#[target_feature(enable = "avx2")]
unsafe fn wc_lines_avx2(data: &[u8]) -> WcLines {
    let mut lines: i64 = 0;
    let bytes: i64 = data.len() as i64;

    let newlines = _mm256_set1_epi8(b'\n' as i8);

    let mut datap = data.as_ptr();
    let mut rem = data.len();

    while rem >= 64 {
        let v0 = _mm256_loadu_si256(datap as *const __m256i);
        let m0 = _mm256_cmpeq_epi8(v0, newlines);
        lines += _mm256_movemask_epi8(m0).count_ones() as i64;
        datap = datap.add(32);
        rem -= 32;
    }

    for i in 0..rem {
        if *datap.add(i) == b'\n' {
            lines += 1;
        }
    }

    WcLines {
        lines,
        bytes,
    }
}

fn main() -> io::Result<()> {
    let file = File::open("../java-orig/measurements.txt")?;
    let mmap = unsafe { Mmap::map(&file)? };

    let n_threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    let chunk_size = mmap.len() / n_threads;

    let mut total_lines = 0;
    let mut total_bytes = 0;

    std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(n_threads);
        for i in 0..n_threads {
            let start = i * chunk_size;
            let end = if i == n_threads - 1 {
                mmap.len()
            } else {
                (i + 1) * chunk_size
            };

            let chunk = &mmap[start..end];
            handles.push(s.spawn(move || unsafe {
                wc_lines_avx2(chunk)
            }));
        }

        for handle in handles {
            let res = handle.join().unwrap();
            total_lines += res.lines;
            total_bytes += res.bytes;
        }
    });

    println!("{} {}", total_lines, total_bytes);

    Ok(())
}
