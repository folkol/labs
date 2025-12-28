use memchr::memchr;
use memmap2::Mmap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::{env, iter, thread};
use std::process::{Command, Stdio};

const CHUNK_SIZE: usize = 1 << 19;

/// 1. Work in child so that we can print result and exit without waiting for munmap of the large thing
/// 2. Split work into reasonable chunks (at newline-boundaries) and parallelize
/// 3. Use explicit threads instead of rayon

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
    assert_eq!(b'\n', data[data.len() - 1]);
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
    let num_threads = env::var("NUM_THREADS").unwrap_or("24".to_string()).parse().unwrap_or(1);
    eprintln!("total threads: {}", num_threads);

    let len = data.len();
    if len == 0 {
        return 0;
    }

    // Split into contiguous newline-aligned ranges.
    let approx = (len + num_threads - 1) / num_threads; // ceil div
    let mut ranges = Vec::with_capacity(num_threads);

    let mut start = 0usize;
    for i in 0..num_threads {
        if start >= len {
            break;
        }
        let mut end = ((i + 1) * approx).min(len);

        // Snap end to newline (inclusive) so we don't split lines,
        // except for the last range which just ends at EOF.
        end = snap_to_newline(data, end);

        // Avoid zero-length ranges if snap didn't move and we're not at EOF.
        if end <= start {
            end = len;
        }

        ranges.push((start, end));
        start = end;
    }

    // Spawn threads to count lines in each range.
    thread::scope(|s| {
        let mut handles = Vec::with_capacity(ranges.len());

        for (start, end) in ranges {
            handles.push(s.spawn(move || count_lines(&data[start..end])));
        }

        handles
            .into_iter()
            .map(|h| h.join().expect("thread panicked"))
            .sum()
    })
}