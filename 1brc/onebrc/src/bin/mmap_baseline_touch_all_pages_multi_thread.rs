use memmap2::Mmap;
use std::fs::File;
use std::io::{self, BufRead};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::{env, thread};

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
    eprintln!("In parent process");
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
    // std::process::exit(0);
    Ok(())
}

unsafe fn madvise(ptr: *const u8, len: usize, advice: i32) {
    unsafe {
        libc::madvise(ptr as *mut libc::c_void, len, advice);
    }
}

fn run_worker() -> io::Result<()> {
    let results: Vec<usize> = thread::scope(|s| {
        let file = File::open("../java-orig/measurements.txt").expect("failed to open file");
        let mmap = unsafe { Mmap::map(&file).expect("failed to map") };
        unsafe {
            madvise(mmap.as_ptr(), mmap.len(), libc::MADV_HUGEPAGE);
            madvise(mmap.as_ptr(), mmap.len(), libc::MADV_SEQUENTIAL); // optional
            // If you want to force population up front:
            // madvise(mmap.as_ptr(), mmap.len(), libc::MADV_WILLNEED);
        }
        eprintln!("pid={}", std::process::id());
        let page_size = 4096usize;
        let num_pages = mmap.len() / page_size;

        let num_threads = env::var("NUM_THREADS")
            .unwrap_or("1".to_string())
            .parse::<usize>()
            .unwrap();
        eprintln!("Processing threads: {num_threads}");
        let pages_per_thread = (num_pages + num_threads - 1) / num_threads; // ceil div

        let mut handles = Vec::with_capacity(num_threads);

        let data = Arc::new(mmap);
        for i in 0..num_threads {
            let my_data = data.clone();
            let start = i * pages_per_thread;
            let end = ((i + 1) * pages_per_thread).min(num_pages);

            handles.push(s.spawn(move || {
                let mut partial = 0usize;
                for n in start..end {
                    partial += my_data[n * page_size] as usize;
                }
                partial
            }));
        }

        let vec = handles
            .into_iter()
            .map(|h| h.join().expect("thread panicked"))
            .collect();
        // std::thread::sleep(std::time::Duration::from_secs(30));
        vec
    });

    println!("{}", results.into_iter().count());
    // std::process::exit(0);
    Ok(())
}
