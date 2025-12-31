use memchr::memchr;
use memmap2::Mmap;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{env, thread};
use xxhash_rust::xxh3::xxh3_64;

const CHUNK_SIZE: usize = 1 << 20;

/// 1. use scan_lines_baseline_multi_thread_child_process_threads_dynamic_work_stealing_newline_aligned.rs
/// 2. add naive hash map that collect statistics
/// 3. HashMap + xxhash
/// 4. get rid of the complicated string comparison
/// 5. get rid of the single-line scanner

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

#[derive(Clone, Copy)]
struct StationStats {
    min: i16,
    max: i16,
    count: usize,
    total: i64,
}

impl Default for StationStats {
    fn default() -> Self {
        Self {
            min: i16::MAX,
            max: i16::MIN,
            count: 0,
            total: 0,
        }
    }
}

// #[derive(Clone, Copy, Default)]
// pub struct Stats {
//     pub count: u32,
//     pub sum: i64, // example
// }

#[derive(Clone, Copy)]
struct Entry {
    name_off: u64,
    name_len: u16,
    hash: u64,
    stats: StationStats,
}

pub struct NameTable<'a> {
    data: &'a [u8],  // backing buffer (mmap or Vec<u8>)
    slots: Vec<u32>, // 0 = empty, else entry_index + 1
    entries: Vec<Entry>,
    mask: usize, // slots.len() - 1
}

impl<'a> NameTable<'a> {
    /// `capacity` = expected number of distinct keys (stations).
    /// Load factor target is ~0.7; we round up to a power-of-two slots size.
    pub fn with_capacity(data: &'a [u8], capacity: usize) -> Self {
        let desired = ((capacity as f64) / 0.70).ceil() as usize;
        let slots_len = desired.next_power_of_two().max(8);

        Self {
            data,
            slots: vec![0; slots_len],
            entries: Vec::with_capacity(capacity),
            mask: slots_len - 1,
        }
    }

    #[inline]
    fn key_bytes(&self, off: u64, len: u16) -> &'a [u8] {
        let off = off as usize;
        let len = len as usize;
        &self.data[off..off + len]
    }

    /// Lookup or insert a key given as (offset, len) into `self.data`.
    /// Returns a mutable reference to the entry's Stats.
    pub fn get_or_insert_stats(&mut self, name_off: u64, name_len: u16) -> &mut StationStats {
        let key = self.key_bytes(name_off, name_len);
        let h = xxh3_64(key);

        // Open addressing with a fixed odd step (31) like the Java code.
        let mut i = self.index_for(h);
        let step = 31usize;

        let mut probes = 0;
        loop {
            let slot = self.slots[i];
            if slot == 0 {
                // eprintln!("inserting new entry for: {}", str::from_utf8(key).unwrap());
                // Insert new entry.
                let idx = self.entries.len();
                self.entries.push(Entry {
                    name_off,
                    name_len,
                    hash: h,
                    stats: Default::default(),
                });
                self.slots[i] = (idx as u32) + 1;
                return &mut self.entries[idx].stats;
            }

            let entry_idx = (slot - 1) as usize;
            let e = self.entries[entry_idx];

            // Fast reject: hash and length first
            if e.hash == h && e.name_len == name_len {
                // Full compare: bytes in backing buffer
                let candidate = self.key_bytes(e.name_off, e.name_len);
                if bytes_eq_u64(candidate, key) {
                    return &mut self.entries[entry_idx].stats;
                }
            }

            i = (i + step) & self.mask;

            probes += 1;
            if probes > self.slots.len() {
                panic!("hash table full (need resize) {}", self.slots.len());
            }
        }
    }

    #[inline]
    fn index_for(&self, hash: u64) -> usize {
        // Fold to usize and mask (slots len is power-of-two).
        // A little extra mixing is fine but not strictly necessary with XXH3.
        let x = hash ^ (hash >> 33);
        (x as usize) & self.mask
    }

    /// Optional: iterate all entries after processing
    pub fn iter_entries(&self) -> impl Iterator<Item = (&[u8], StationStats)> + '_ {
        self.entries
            .iter()
            .map(|e| (self.key_bytes(e.name_off, e.name_len), e.stats))
    }
}

/// Compare two byte slices quickly using 8-byte loads.
/// Safe wrapper; uses `read_unaligned` internally.
#[inline]
fn bytes_eq_u64(a: &[u8], b: &[u8]) -> bool {
    a == b
    // if a.len() != b.len() {
    //     return false;
    // }
    // let len = a.len();
    // let mut i = 0usize;
    //
    // unsafe {
    //     // Compare 8 bytes at a time
    //     while i + 8 <= len {
    //         let va = ptr::read_unaligned(a.as_ptr().add(i) as *const u64);
    //         let vb = ptr::read_unaligned(b.as_ptr().add(i) as *const u64);
    //         if va != vb {
    //             return false;
    //         }
    //         i += 8;
    //     }
    //
    //     // Tail compare (0..7 bytes) with a mask
    //     let rem = len - i;
    //     if rem == 0 {
    //         return true;
    //     }
    //
    //     let mut ta = 0u64;
    //     let mut tb = 0u64;
    //     ptr::copy_nonoverlapping(a.as_ptr().add(i), &mut ta as *mut u64 as *mut u8, rem);
    //     ptr::copy_nonoverlapping(b.as_ptr().add(i), &mut tb as *mut u64 as *mut u8, rem);
    //     ta == tb
    // }
}

#[inline]
fn parse_temp_tenths(s: &[u8]) -> i16 {
    // formats like: b"-12.3" or b"7.8" or b"0.0"
    // range fits i16 easily
    let mut i = 0usize;
    let neg = s.first() == Some(&b'-');
    if neg {
        i += 1;
    }

    // read int part (1 or 2 digits for your range)
    let mut val: i16 = (s[i] - b'0') as i16;
    i += 1;
    if s[i] != b'.' {
        val = val * 10 + (s[i] - b'0') as i16;
        i += 1;
    }
    // s[i] == b'.'
    i += 1;
    val = val * 10 + (s[i] - b'0') as i16;

    if neg { -val } else { val }
}

#[inline]
fn parse_temp_tenths_fixed_dot(data: &[u8], end: usize) -> i16 {
    // end points just past '\n' (i.e. data[end-1] == b'\n')
    debug_assert_eq!(data[end - 1], b'\n');
    debug_assert_eq!(data[end - 3], b'.'); // '.' is always 2 bytes before '\n'

    // Indices relative to end:
    // end-2: tenths digit
    // end-4: ones digit
    // end-5: tens digit OR '-'/';'/something
    // end-6: '-' if -dd.d
    let tenths = (data[end - 2] - b'0') as i16;
    let ones = (data[end - 4] - b'0') as i16;

    let b5 = data[end - 5]; // could be digit (tens) or '-'/';'
    let (tens, neg) = if b5.is_ascii_digit() {
        let tens = (b5 - b'0') as i16;
        let neg = data[end - 6] == b'-';
        (tens, neg)
    } else {
        // one-digit integer part; sign (if any) is exactly at end-5
        (0i16, b5 == b'-')
    };

    let abs = (tens * 10 + ones) * 10 + tenths;
    if neg { -abs } else { abs }
}

#[inline]
unsafe fn parse_temp_tenths_fixed_dot_ptr(line_end_nl: *const u8) -> i16 { unsafe {
    // line_end_nl points at '\n'
    debug_assert!(*line_end_nl == b'\n');
    debug_assert!(*line_end_nl.sub(2) == b'.'); // '.' is 2 bytes before '\n'

    let tenths = (*line_end_nl.sub(1) - b'0') as i16;
    let ones = (*line_end_nl.sub(3) - b'0') as i16;

    let b5 = *line_end_nl.sub(4); // could be digit (tens) or '-'/';'
    let (tens, neg) = if b5.is_ascii_digit() {
        let tens = (b5 - b'0') as i16;
        let neg = *line_end_nl.sub(5) == b'-';
        (tens, neg)
    } else {
        (0i16, b5 == b'-')
    };

    let abs = (tens * 10 + ones) * 10 + tenths;
    if neg { -abs } else { abs }
}}

#[inline]
fn find_byte_mask(word: u64, byte: u8) -> u64 {
    let x = word ^ u64::from_ne_bytes([byte; 8]);
    x.wrapping_sub(0x0101_0101_0101_0101) & !x & 0x8080_8080_8080_8080
}

#[inline]
unsafe fn find_delim(mut p: *const u8, end: *const u8, byte: u8) -> *const u8 { unsafe {
    // Scan 8 bytes at a time; end is one-past-last valid byte.
    while p.add(8) <= end {
        let w = (p as *const u64).read_unaligned();
        let m = find_byte_mask(w, byte);
        if m != 0 {
            return p.add((m.trailing_zeros() >> 3) as usize);
        }
        p = p.add(8);
    }
    // Tail
    while p < end {
        if *p == byte {
            return p;
        }
        p = p.add(1);
    }
    end
}}

fn chunk_statistics(data: &[u8], chunk_start: usize, chunk_end: usize, statistics: &mut NameTable) {
    let chunk = &data[chunk_start..chunk_end];
    assert_eq!(chunk[chunk.len() - 1], b'\n');

    let base = chunk.as_ptr();
    let endp = unsafe { base.add(chunk.len()) };

    unsafe {
        let mut line_start = base;

        while line_start < endp {
            // Find ';' starting from line_start (forward, not backward).
            let semi = find_delim(line_start, endp, b';');
            debug_assert!(semi < endp && *semi == b';');

            // Find '\n' starting from semi (or from line_start; semi is fine).
            let nl = find_delim(semi, endp, b'\n');
            debug_assert!(nl < endp && *nl == b'\n');

            // Parse temperature from the tail (uses your fixed-dot invariant).
            let temp = parse_temp_tenths_fixed_dot_ptr(nl);

            // Off/len for the station name = [line_start .. semi)
            let name_off = (chunk_start as isize + (line_start as isize - base as isize)) as u64;
            let name_len = (semi as usize - line_start as usize) as u16;

            let entry = statistics.get_or_insert_stats(name_off, name_len);
            entry.count += 1;
            entry.min = entry.min.min(temp);
            entry.max = entry.max.max(temp);
            entry.total += temp as i64;

            line_start = nl.add(1);
        }
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

// fn fmt_tenths(x: i16) -> String {
//     let neg = x < 0;
//     let a = (x as i32).abs();
//     let whole = a / 10;
//     let frac = a % 10;
//     if neg { format!("-{whole}.{frac}") } else { format!("{whole}.{frac}") }
// }

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
                let mut statistics = NameTable::with_capacity(data, 10000);
                while let Some((start, end)) = claim_chunk(data, &next) {
                    assert_eq!(data[end - 1], b'\n');
                    // eprintln!("{} - {}", start, end);
                    chunk_statistics(data, start, end, &mut statistics);
                }
                statistics
            }));
        }

        let mut total_statistics: BTreeMap<String, StationStats> = BTreeMap::new();
        for handle in handles {
            let statistics = handle.join().unwrap();
            for (k, v) in statistics.iter_entries() {
                total_statistics
                    .entry(str::from_utf8(k).unwrap().to_string())
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
                v.min as f64 / 10.,
                v.total as f64 / 10. / v.count as f64,
                v.max as f64 / 10.
            );
            sep = ", ";
        }
        output += "}";

        output
    })
}
