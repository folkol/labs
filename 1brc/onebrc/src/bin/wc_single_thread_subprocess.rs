use memmap2::Mmap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};

/// 1. Work in child so that we can print result and exit without waiting for munmap of the large thing

fn main() {
    let is_worker = std::env::args().any(|a| a == "--worker");
    if is_worker {
        run_worker().expect("run_worker failed");
    } else {
        run_parent().expect("run_parent failed")
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

    Ok(())
}

fn run_worker() -> io::Result<()> {
    let file = File::open("../java-orig/measurements.txt")?;
    let data = unsafe { Mmap::map(&file)? };

    println!("{}", count_lines(&data));
    io::stdout().flush()?;

    Ok(())
}

fn count_lines(data: &[u8]) -> usize {
    memchr::memchr_iter(b'\n', data).count()
}
