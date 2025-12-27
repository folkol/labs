#![allow(unsafe_op_in_unsafe_fn)]
use libc::{c_void, read};
use std::arch::x86_64::*;
use std::io;
use std::os::unix::io::AsRawFd;

#[derive(Debug)]
struct WcLines {
    #[allow(dead_code)]
    err: i32,
    lines: i64,
    bytes: i64,
}

const BUFSIZE: usize = 1024*1024;

#[target_feature(enable = "avx2")]
unsafe fn wc_lines_avx2(fd: i32) -> WcLines {
    let mut lines: i64 = 0;
    let mut bytes: i64 = 0;

    let endlines = _mm256_set1_epi8(b'\n' as i8);

    let mut buf: Vec<u8> = Vec::with_capacity(BUFSIZE);

    unsafe {
        buf.set_len(BUFSIZE);
    }

    loop {
        let n = unsafe { read(fd, buf.as_mut_ptr() as *mut c_void, BUFSIZE) };
        if n <= 0 {
            return WcLines {
                err: if n == 0 { 0 } else { *libc::__errno_location() },
                lines,
                bytes,
            };
        }

        let bytes = n as usize;
        let mut datap = buf.as_ptr();
        let mut rem = bytes;

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
            if unsafe { *datap.add(i) } == b'\n' {
                lines += 1;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();
    unsafe {
        let res = wc_lines_avx2(fd);
        println!("{} {}", res.lines, res.bytes);
    }
}
