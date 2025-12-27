use memchr::memchr;
use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::iter;
use std::process::{Command, Stdio};

const CHUNK_SIZE: usize = 1 << 19;

/// 1. Work in child so that we can print result and exit without waiting for munmap of the large thing
/// 2. Split work into reasonable chunks (at newline-boundaries) and parallelize
/// 3.

fn main() -> io::Result<()> {
    let is_worker = std::env::args().any(|a| a == "--worker");
    if is_worker {
        run_worker()
    } else {
        run_parent()
    }
}

fn run_parent() -> io::Result<()> {
    let exe = std::env::current_exe()?;
    let child = Command::new(exe)
        .arg("--worker")
        .stdout(Stdio::piped())
        .spawn()?;

    let out = child.stdout.expect("expected stdout to be present");
    let mut line = String::new();
    let mut reader = io::BufReader::new(out);
    reader.read_line(&mut line)?;
    print!("{line}");
    std::process::exit(0);
}

fn run_worker() -> io::Result<()> {
    let file = File::open("../java-orig/measurements.txt")?;
    let data = unsafe { Mmap::map(&file)? };

    println!("{}", total_lines(&data));
    io::stdout().flush().expect("expected flush to work");
    std::process::exit(0);
}

fn count_lines(data: &[u8]) -> i64 {
    memchr::memchr_iter(b'\n', data).count() as i64
}

#[inline]
fn snap_to_newline(data: &[u8], end: usize) -> usize {
    if end < data.len() {
        end + memchr(b'\n', &data[end..]).unwrap()
    } else {
        end
    }
}

fn total_lines(data: &[u8]) -> i64 {
    let mut pos = 0;
    let len = data.len();

    iter::from_fn(move || {
        if pos >= len {
            return None;
        }

        let end = usize::min(pos + CHUNK_SIZE, len);
        let (start, end) = (pos, snap_to_newline(data, end));
        pos = end;
        Some(&data[start..end])
    })
    .par_bridge()
    .map(count_lines)
    .sum()
}
