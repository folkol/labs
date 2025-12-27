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

    let endlines = _mm256_set1_epi8(b'\n' as i8);

    let mut datap = data.as_ptr();
    let mut rem = data.len();

    while rem >= 64 {
        let v0 = _mm256_loadu_si256(datap as *const __m256i);
        let v1 = _mm256_loadu_si256(datap.add(32) as *const __m256i);

        let m0 = _mm256_cmpeq_epi8(v0, endlines);
        let m1 = _mm256_cmpeq_epi8(v1, endlines);

        lines += (_mm256_movemask_epi8(m0).count_ones() + _mm256_movemask_epi8(m1).count_ones())
            as i64;

        datap = datap.add(64);
        rem -= 64;
    }

    // scalar tail
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

    unsafe {
        let res = wc_lines_avx2(&mmap);
        println!("{} {}", res.lines, res.bytes);
    }
    Ok(())
}
