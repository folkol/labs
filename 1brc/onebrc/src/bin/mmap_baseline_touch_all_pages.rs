use memmap2::Mmap;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::{Command, Stdio};


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

    let mut result = 0;
    let page_size = 4096;
    for n in (0..data.len()).step_by(page_size) {
        result += &data[n]
    }

    println!("{result}");
    std::process::exit(0);
}
