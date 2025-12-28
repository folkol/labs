use memchr::memchr;
use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
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
    let n = total_lines_from_reader(BufReader::new(file))?;

    println!("{n}");
    io::stdout().flush().expect("expected flush to work");
    std::process::exit(0);
}

fn count_lines(data: &[u8]) -> i64 {
    memchr::memchr_iter(b'\n', data).count() as i64
}

/// Reads the file as newline-aligned chunks and counts '\n' in parallel.
///
/// Note: this is still newline-counting, so it assumes the file ends with '\n'
/// if you want "number of lines" == "number of '\n'".
fn total_lines_from_reader<R: Read + Send>(mut r: BufReader<R>) -> io::Result<i64> {
    let chunks = iter::from_fn(move || read_newline_aligned_chunk(&mut r, CHUNK_SIZE).transpose());

    let total = chunks
        .par_bridge()
        .map(|chunk_res| chunk_res.map(|chunk| count_lines(&chunk)))
        .try_reduce(|| 0i64, |a, b| Ok(a + b))?;

    Ok(total)
}

/// Read up to `chunk_size` bytes, then extend to the next '\n' (inclusive),
/// unless EOF. Returns `Ok(None)` at EOF.
///
/// Guarantees: if it returns `Some(buf)`, then either:
/// - buf ends with '\n', or
/// - it hit EOF and returned the last partial chunk.
///
/// If you require trailing '\n' exactly, check after the count.
fn read_newline_aligned_chunk<R: Read>(
    r: &mut BufReader<R>,
    chunk_size: usize,
) -> io::Result<Option<Vec<u8>>> {
    let mut buf = vec![0u8; chunk_size];
    let n = r.read(&mut buf)?;
    if n == 0 {
        return Ok(None);
    }
    buf.truncate(n);

    // If we filled the chunk, extend to newline so we don't split a line.
    if n == chunk_size {
        // BufRead::read_until appends (including the delimiter).
        let mut tail = Vec::new();
        let _ = r.read_until(b'\n', &mut tail)?;
        buf.extend_from_slice(&tail);
    }

    Ok(Some(buf))
}