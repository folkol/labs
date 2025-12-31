use memchr::memchr;
use memmap2::Mmap;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{env, thread};

const CHUNK_SIZE: usize = 1 << 20;

/// 1. use scan_lines_baseline_multi_thread_child_process_threads_dynamic_work_stealing_newline_aligned.rs
/// 2. add naive hash map that collect statistics
/// 3. HashMap + xxhash
/// 4. get rid of the complicated string comparison

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

#[derive(Clone, Copy)]
struct Entry {
    name_off: u64,
    name_len: u16,
    hash: u64,
    w1: u64,
    w2: u64,
    stats: StationStats,
}

#[inline]
unsafe fn load_u64_unaligned(p: *const u8) -> u64 { unsafe {
    (p as *const u64).read_unaligned()
}}

#[inline]
fn mask_low_bytes(n: usize) -> u64 {
    // keep lowest n bytes
    if n >= 8 { !0u64 } else { (1u64 << (n * 8)) - 1 }
}

#[inline]
unsafe fn load_prefix2(p: *const u8, len: usize) -> (u64, u64) { unsafe {
    // Loads first up-to-16 bytes, masking bytes beyond len to 0.
    // Caller guarantees p points into the mmap; len >= 0.
    let w1 = load_u64_unaligned(p);
    if len <= 8 {
        (w1 & mask_low_bytes(len), 0)
    } else {
        let w2 = load_u64_unaligned(p.add(8));
        (w1, w2 & mask_low_bytes(len - 8))
    }
}}

#[inline]
fn hash16(w1: u64, w2: u64, len: u16) -> u64 {
    // cheap-ish mix; faster than xxh3 for short keys
    let mut x = w1 ^ w2 ^ ((len as u64) << 48);
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x
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
        let len = name_len as usize;

        // Load prefix directly from backing buffer (no copies).
        let p = unsafe { self.data.as_ptr().add(name_off as usize) };
        let (w1, w2) = unsafe { load_prefix2(p, len) };
        let h = hash16(w1, w2, name_len);

        let mut i = self.index_for(h);
        let step = 31usize;

        loop {
            let slot = unsafe { *self.slots.get_unchecked(i) };
            if slot == 0 {
                let idx = self.entries.len();
                self.entries.push(Entry {
                    name_off,
                    name_len,
                    hash: h,
                    w1,
                    w2,
                    stats: Default::default(),
                });
                unsafe {
                    *self.slots.get_unchecked_mut(i) = (idx as u32) + 1;
                }
                return &mut self.entries[idx].stats;
            }

            let entry_idx = (slot - 1) as usize;
            let e = unsafe { *self.entries.get_unchecked(entry_idx) };

            if e.hash == h && e.name_len == name_len && e.w1 == w1 && e.w2 == w2 {
                if len <= 16 {
                    return &mut self.entries[entry_idx].stats;
                }
                // Rare slow path: keys longer than 16 bytes must compare full bytes
                let key = self.key_bytes(name_off, name_len);
                let candidate = self.key_bytes(e.name_off, e.name_len);
                if bytes_eq_u64(candidate, key) {
                    return &mut self.entries[entry_idx].stats;
                }
            }

            i = (i + step) & self.mask;
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
unsafe fn parse_temp_from_semi(semi_plus_1: *const u8) -> i16 { unsafe {
    // formats:  "-dd.d" | "-d.d" | "dd.d" | "d.d"
    let mut p = semi_plus_1;

    let mut neg = false;
    let mut c = *p;
    if c == b'-' {
        neg = true;
        p = p.add(1);
        c = *p;
    }

    // first digit
    let d0 = (c - b'0') as i16;
    p = p.add(1);

    // either digit or '.'
    let c1 = *p;
    let (int_part, p_after_dot) = if c1 == b'.' {
        (d0, p.add(1))
    } else {
        // two-digit integer part
        let d1 = (c1 - b'0') as i16;
        // next must be '.'
        (d0 * 10 + d1, p.add(2))
    };

    let tenths = (*p_after_dot - b'0') as i16;
    let v = int_part * 10 + tenths;
    if neg { -v } else { v }
}}

#[inline]
unsafe fn load_u64(p: *const u8) -> u64 { unsafe {
    (p as *const u64).read_unaligned()
}}

#[inline]
fn find_byte_mask(word: u64, byte: u8) -> u64 {
    // classic "haszero" trick after XOR with repeated byte
    let x = word ^ u64::from_le_bytes([byte; 8]);
    x.wrapping_sub(0x0101_0101_0101_0101) & !x & 0x8080_8080_8080_8080 
}

#[inline]
unsafe fn scan_to_byte(mut p: *const u8, byte: u8) -> *const u8 { unsafe {
    loop {
        let w = load_u64(p);
        let m = find_byte_mask(w, byte);
        if m != 0 {
            let off = (m.trailing_zeros() >> 3) as usize;
            return p.add(off);
        }
        p = p.add(8);
    }
}}

fn chunk_statistics(data: &[u8], chunk_start: usize, chunk_end: usize, statistics: &mut NameTable) {
    assert_eq!(data[chunk_end - 1], b'\n');

    unsafe {
        let base = data.as_ptr();

        let mut p = base.add(chunk_start);
        let end = base.add(chunk_end);

        while p < end {
            // find ';' (forward, wide)
            let semi = scan_to_byte(p, b';');

            // find '\n' (forward, wide) starting from after ';'
            let nl = scan_to_byte(semi.add(1), b'\n');

            let name_off = (p as usize - base as usize) as u64;
            let name_len = (semi as usize - p as usize) as u16;

            // fast temp parse from semi+1
            // (always safe inside the mmap; but if you want to be strict:
            //  add a bounds check and fallback to a safe parse)
            let temp = parse_temp_from_semi(semi.add(1));

            let entry = statistics.get_or_insert_stats(name_off, name_len);
            entry.count += 1;
            entry.min = entry.min.min(temp);
            entry.max = entry.max.max(temp);
            entry.total += temp as i64;

            // next line starts after '\n'
            p = nl.add(1);
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
