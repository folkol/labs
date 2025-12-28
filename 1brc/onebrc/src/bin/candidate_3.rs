use memchr::memchr;
use memmap2::Mmap;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{env, thread};
use std::hash::{BuildHasher, Hasher};
use ahash::AHashMap;

const CHUNK_SIZE: usize = 1 << 20;

/// 1. use scan_lines_baseline_multi_thread_child_process_threads_dynamic_work_stealing_newline_aligned.rs
/// 2. add naive hash map that collect statistics
/// 3. HashMap + xxhash

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
    // std::process::exit(0);
    Ok(())
}

fn run_worker() -> io::Result<()> {
    let file = File::open("../java-orig/measurements.txt")?;
    let data = unsafe { Mmap::map(&file)? };

    println!("{}", total_lines(&data));
    io::stdout().flush().expect("expected flush to work");
    assert_eq!(b'\n', data[data.len() - 1]);
    // std::process::exit(0);
    Ok(())
}

struct StationStats {
    min: f64,
    max: f64,
    count: usize,
    total: f64,
}

use xxhash_rust::xxh3::Xxh3;

#[derive(Clone, Default)]
struct XxHashBuilder;

impl BuildHasher for XxHashBuilder {
    type Hasher = Xxh3;

    fn build_hasher(&self) -> Self::Hasher {
        Xxh3::new()
    }
}


fn chunk_statistics(data: &[u8], statistics: &mut HashMap<String, StationStats, XxHashBuilder>) {
    // eprintln!("data.len {}", data.len());
    assert_eq!(data[data.len() - 1], b'\n');
    // eprintln!("head: {:?}", str::from_utf8(&data[0..100]).unwrap());
    let mut start = 0;
    for end in memchr::memchr_iter(b'\n', data) {
        if start >= end {
            break
        }
        // eprintln!("start: end -> {start}: {end}");
        let semicolon = start + memchr::memrchr(b';', &data[start..end]).unwrap();
        // eprintln!("semicolon: {}", semicolon);
        let temperature_1 = &data[semicolon + 1..end];
        // eprintln!("temperature_1: {:?}", temperature_1);
        let temperature_2 = str::from_utf8(&temperature_1).unwrap();
        // eprintln!("temperature_2: {:?}", temperature_2);
        let temperature = match temperature_2.parse::<f64>() {
            Ok(f) => f,
            Err(e) => {
                panic!(
                    "failed to parse temperature: {e:?}\n\
                    - temperature_1={temperature_1:?}\n\
                    - temperature_2={temperature_2:?}\n\
                    - start={start}\n\
                    - end={end}\n\
                    - semicolon={semicolon}"
                );
            }
        };

        let station = &data[start..semicolon];
        let station = String::from_utf8(station.to_vec()).unwrap();
        let entry = statistics.entry(station).or_insert(StationStats {
            min: f64::MAX,
            max: f64::MIN,
            count: 0,
            total: 0.0,
        });
        entry.count += 1;
        entry.min = f64::min(entry.min, temperature);
        entry.max = f64::max(entry.max, temperature);
        entry.total = entry.total + temperature;

        start = end + 1;
        // eprintln!("...");
    }
}

#[inline]
fn snap_to_newline(data: &[u8], end: usize) -> usize {
    if end < data.len() {
        end + 1 + memchr(b'\n', &data[end..]).unwrap()
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

        assert_eq!(data[end - 1], b'\n');

        if next
            .compare_exchange_weak(start, end, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
        {
            return Some((start, end));
        }
    }
}


fn total_lines(data: &[u8]) -> String {
    let num_threads = match env::var("NUM_THREADS") {
        Ok(v) => v.parse::<usize>().unwrap(),
        Err(_) => thread::available_parallelism().map_or(1, |n| n.get()),
    };
    eprintln!("total threads: {}", num_threads);

    let next = AtomicUsize::new(0);

    thread::scope(|s| {
        let mut handles = Vec::with_capacity(num_threads);

        for _ in 0..num_threads {
            handles.push(s.spawn(|| {
                let mut statistics = HashMap::with_capacity_and_hasher(1000, XxHashBuilder);
                while let Some((start, end)) = claim_chunk(&data, &next) {
                    assert_eq!(data[end - 1], b'\n');
                    // eprintln!("{} - {}", start, end);
                    chunk_statistics(&data[start..end], &mut statistics);
                }
                statistics
            }));
        }

        let mut total_statistics: BTreeMap<String, StationStats> = BTreeMap::new();
        for handle in handles {
            let statistics = handle.join().unwrap();
            for (k, v) in statistics {
                total_statistics
                    .entry(k)
                    .and_modify(|entry| {
                        entry.min = entry.min.min(v.min);
                        entry.max = entry.max.max(v.max);
                        entry.count += v.count;
                        entry.total += v.total;
                    })
                    .or_insert(v);
            }
        }

        let mut output = String::new();
        output += "{";
        let mut sep = "";
        for (k, v) in total_statistics {
            output += sep;
            output += &format!(
                "{k}={:.1}/{:.1}/{:.1}",
                v.min,
                v.total / v.count as f64,
                v.max
            );
            sep = ", ";
        }
        output += "}";

        output
    })
}
