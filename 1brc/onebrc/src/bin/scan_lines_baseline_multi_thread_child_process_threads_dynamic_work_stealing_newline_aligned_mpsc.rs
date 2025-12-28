use memchr::memchr;
use memmap2::Mmap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

const CHUNK_SIZE: usize = 1 << 20;

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

fn claim_chunk(data: &[u8], next: &AtomicUsize) -> Option<(usize, usize)> {
    let len = data.len();
    loop {
        let start = next.load(Ordering::Relaxed);
        if start >= len {
            return None;
        }

        let end = usize::min(start + CHUNK_SIZE, len);
        let end = snap_to_newline(data, end);

        if next
            .compare_exchange_weak(start, end, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            return Some((start, end));
        }
    }
}

fn total_lines(data: &[u8]) -> i64 {
    let num_threads = thread::available_parallelism().map_or(1, |n| n.get());
    eprintln!("total threads: {}", num_threads);

    let next = AtomicUsize::new(0);
    let next = &next;

    thread::scope(|s| {
        let (tx, rx) = mpsc::channel();

        let mut handles = Vec::with_capacity(num_threads);

        for _ in 0..num_threads {
            let tx = tx.clone();
            handles.push(s.spawn(move || {
                let mut sum: i64 = 0;
                while let Some((start, end)) = claim_chunk(&data, &next) {
                    sum += count_lines(&data[start..end]);
                }
                tx.send(sum).unwrap();
            }));
        }

        drop(tx); // close channel when all workers spawned

        rx.iter().sum()
    })
}
