#![allow(unsafe_op_in_unsafe_fn)]
use std::arch::x86_64::*;
use std::io;
use std::os::unix::io::AsRawFd;
use libc::{read, c_void};

#[derive(Debug)]
struct WcLines {
    #[allow(dead_code)]
    err: i32,
    lines: i64,
    bytes: i64,
}

const BUFSIZE: usize = 16320;

#[target_feature(enable = "avx2")]
unsafe fn wc_lines_avx2(fd: i32) -> WcLines {
    let mut lines: i64 = 0;
    let mut bytes: i64 = 0;

    let zeroes = _mm256_setzero_si256();
    let endlines = _mm256_set1_epi8(b'\n' as i8);

    // Ensure the buffer is aligned for SIMD loads
    #[repr(align(32))]
    struct AlignedBuf([u8; BUFSIZE]);
    let mut avx_buf = AlignedBuf([0u8; BUFSIZE]);

    loop {
        let mut accumulator = _mm256_setzero_si256();
        let mut accumulator2 = _mm256_setzero_si256();

        let bytes_read = read(fd, avx_buf.0.as_mut_ptr() as *mut c_void, BUFSIZE);
        if bytes_read <= 0 {
            return WcLines {
                err: if bytes_read == 0 { 0 } else { *libc::__errno_location() },
                lines,
                bytes,
            };
        }

        bytes += bytes_read as i64;
        let mut bytes_rem = bytes_read as usize;
        let mut datap = avx_buf.0.as_ptr();

        while bytes_rem >= 64 {
            let to_match = _mm256_load_si256(datap as *const __m256i);
            let to_match2 = _mm256_load_si256(datap.add(32) as *const __m256i);
            let matches = _mm256_cmpeq_epi8(to_match, endlines);
            let matches2 = _mm256_cmpeq_epi8(to_match2, endlines);

            accumulator = _mm256_sub_epi8(accumulator, matches);
            accumulator2 = _mm256_sub_epi8(accumulator2, matches2);

            datap = datap.add(64);
            bytes_rem -= 64;
        }

        accumulator = _mm256_sad_epu8(accumulator, zeroes);
        // lines += (_mm256_extract_epi16(accumulator, 0) as i64)
        //     + (_mm256_extract_epi16(accumulator, 4) as i64)
        //     + (_mm256_extract_epi16(accumulator, 8) as i64)
        //     + (_mm256_extract_epi16(accumulator, 12) as i64);

        // vpsadbw (sad_epu8) yields 4x u64 sums in the YMM register; sum them as scalars.
        let mut sums: [u64; 4] = [0; 4];
        _mm256_storeu_si256(sums.as_mut_ptr() as *mut __m256i, accumulator);
        lines += (sums[0] + sums[1] + sums[2] + sums[3]) as i64;

        accumulator2 = _mm256_sad_epu8(accumulator2, zeroes);
        // lines += (_mm256_extract_epi16(accumulator2, 0) as i64)
        //     + (_mm256_extract_epi16(accumulator2, 4) as i64)
        //     + (_mm256_extract_epi16(accumulator2, 8) as i64)
        //     + (_mm256_extract_epi16(accumulator2, 12) as i64);

        let mut sums2: [u64; 4] = [0; 4];
        _mm256_storeu_si256(sums2.as_mut_ptr() as *mut __m256i, accumulator2);
        lines += (sums2[0] + sums2[1] + sums2[2] + sums2[3]) as i64;

        for i in 0..bytes_rem {
            if *datap.add(i) == b'\n' {
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
