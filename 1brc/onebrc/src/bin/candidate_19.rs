use memchr::memchr;
use memmap2::Mmap;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{env, thread};

const CHUNK_SIZE: usize = 1 << 20;

// Java-style mask table (avoids variable shifts in mask_low_bytes)
const MASK1: [u64; 9] = [
    0x00,
    0xFF,
    0xFFFF,
    0xFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFFFF,
    0xFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
];

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
    Ok(())
}

fn run_worker() -> io::Result<()> {
    let file = File::open("../java-orig/measurements.txt")?;
    let data = unsafe { Mmap::map(&file)? };

    println!("{}", total_lines(&data));
    io::stdout().flush().expect("expected flush to work");
    assert_eq!(b'\n', data[data.len() - 1]);
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

#[inline(always)]
unsafe fn load_u64_unaligned(p: *const u8) -> u64 { unsafe {
    (p as *const u64).read_unaligned()
}}

#[inline(always)]
unsafe fn load_prefix2(p: *const u8, len: usize) -> (u64, u64) { unsafe {
    let w1 = load_u64_unaligned(p);
    let w2 = load_u64_unaligned(p.add(8));

    if len >= 16 {
        (w1, w2)
    } else if len <= 8 {
        // len ∈ [0,8] → safe
        let m = *MASK1.get_unchecked(len);
        (w1 & m, 0)
    } else {
        // len ∈ [9,15] → (len - 8) ∈ [1,7] → safe
        let m = *MASK1.get_unchecked(len - 8);
        (w1, w2 & m)
    }
}}

#[inline(always)]
fn hash16(w1: u64, w2: u64, len: u16) -> u64 {
    let mut x = w1 ^ w2 ^ ((len as u64) << 48);
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x
}

pub struct NameTable<'a> {
    data: &'a [u8],
    slots: Vec<u32>, // 0 = empty, else entry_index + 1
    entries: Vec<Entry>,
    mask: usize,
}

impl<'a> NameTable<'a> {
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

    /// Lookup or insert a key given as (offset, len) into `self.data`.
    /// Returns a mutable reference to the entry's Stats.
    #[inline(always)]
    pub fn get_or_insert_stats(&mut self, name_off: u64, name_len: u16) -> &mut StationStats {
        let len = name_len as usize;

        let base = self.data.as_ptr();
        let key_ptr = unsafe { base.add(name_off as usize) };

        let (w1, w2) = unsafe { load_prefix2(key_ptr, len) };
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
                return unsafe { &mut self.entries.get_unchecked_mut(idx).stats };
            }

            let entry_idx = (slot - 1) as usize;
            let e = unsafe { self.entries.get_unchecked(entry_idx) };

            if e.hash == h && e.name_len == name_len && e.w1 == w1 && e.w2 == w2 {
                if len <= 16 {
                    return unsafe { &mut self.entries.get_unchecked_mut(entry_idx).stats };
                }

                let cand_ptr = unsafe { base.add(e.name_off as usize) };
                if unsafe { Self::bytes_eq_u64_ptr(cand_ptr, key_ptr, len) } {
                    return unsafe { &mut self.entries.get_unchecked_mut(entry_idx).stats };
                }
            }

            i = (i + step) & self.mask;
        }
    }

    #[inline(always)]
    unsafe fn bytes_eq_u64_ptr(mut a: *const u8, mut b: *const u8, mut len: usize) -> bool { unsafe {
        while len >= 8 {
            if (a as *const u64).read_unaligned() != (b as *const u64).read_unaligned() {
                return false;
            }
            a = a.add(8);
            b = b.add(8);
            len -= 8;
        }
        if len != 0 {
            // NOTE: may read past slice end if key ends at the very end of mmap.
            // For strict safety replace with a byte loop for the tail.
            let mask = MASK1[len];
            let wa = (a as *const u64).read_unaligned() & mask;
            let wb = (b as *const u64).read_unaligned() & mask;
            wa == wb
        } else {
            true
        }
    }}

    #[inline(always)]
    fn index_for(&self, hash: u64) -> usize {
        let x = hash ^ (hash >> 33);
        (x as usize) & self.mask
    }

    pub fn iter_entries(&self) -> impl Iterator<Item = (&[u8], StationStats)> + '_ {
        self.entries.iter().map(|e| {
            let off = e.name_off as usize;
            let len = e.name_len as usize;
            (&self.data[off..off + len], e.stats)
        })
    }
}

#[inline(always)]
unsafe fn load_u64(p: *const u8) -> u64 { unsafe {
    (p as *const u64).read_unaligned()
}}

#[inline(always)]
fn find_byte_mask(word: u64, byte: u8) -> u64 {
    let x = word ^ u64::from_le_bytes([byte; 8]);
    x.wrapping_sub(0x0101_0101_0101_0101) & !x & 0x8080_8080_8080_8080 
}

#[inline(always)]
fn first_hit_byte_index(mask: u64) -> usize {
    // mask has 0x80 set in the byte lane that matched
    (mask.trailing_zeros() >> 3) as usize
}

/// Branchless temp parse (tenths) ported from the Java winner.
/// Input: pointer at first char after ';' (digit or '-')
#[inline(always)]
unsafe fn parse_temp_branchless(semi_plus_1: *const u8) -> i16 { unsafe {
    let number_word = (semi_plus_1 as *const u64).read_unaligned();

    // Java: trailingZeros(~numberWord & 0x10101000L)
    let decimal_sep_pos = ((!number_word) & 0x0000_0000_1010_1000u64).trailing_zeros() as i32;

    let shift = 28 - decimal_sep_pos;

    // signed is -1 if negative, 0 otherwise
    let signed = ((!number_word) << 59) as i64 >> 63 ;
    let design_mask = !((signed as u64) & 0xFF);

    let digits = ((number_word & design_mask) << shift) & 0x0000_0F00_0F0F_00u64;
    let abs_value = (((digits.wrapping_mul(0x640a_0001)) >> 32) & 0x3FF) as i64;

    ((abs_value ^ signed) - signed) as i16
}}

/// Single-pass record parser:
/// - scans forward in 8-byte steps
/// - finds ';' once
/// - continues scanning (without restarting) until '\n' found
/// This avoids calling `scan_to_byte` twice per record and handles the
/// "semicolon and newline in the same 8-byte word" case efficiently.
fn chunk_statistics(data: &[u8], chunk_start: usize, chunk_end: usize, statistics: &mut NameTable) {
    assert_eq!(data[chunk_end - 1], b'\n');

    unsafe {
        let base = data.as_ptr();
        let mut p = base.add(chunk_start);
        let end = base.add(chunk_end);

        while p < end {
            let line_start = p;
            let mut semi: *const u8 = std::ptr::null();

            // Scan forward until we find ';' and then '\n' (continuing the same scan).
            loop {
                let w = load_u64(p);
                let semi_m = if semi.is_null() {
                    find_byte_mask(w, b';')
                } else {
                    0
                };
                let nl_m = find_byte_mask(w, b'\n');

                if !semi_m.eq(&0) && semi.is_null() {
                    let si = first_hit_byte_index(semi_m);
                    semi = p.add(si);

                    // If '\n' is also in this word after ';', finish immediately.
                    if nl_m != 0 {
                        let ni = first_hit_byte_index(nl_m);
                        if ni > si {
                            let nl = p.add(ni);

                            let name_off = (line_start as usize - base as usize) as u64;
                            let name_len = (semi as usize - line_start as usize) as u16;

                            let temp = parse_temp_branchless(semi.add(1));

                            let entry = statistics.get_or_insert_stats(name_off, name_len);
                            entry.count += 1;
                            entry.min = entry.min.min(temp);
                            entry.max = entry.max.max(temp);
                            entry.total += temp as i64;

                            p = nl.add(1);
                            break;
                        }
                    }
                }

                if !semi.is_null() && nl_m != 0 {
                    let ni = first_hit_byte_index(nl_m);
                    let nl = p.add(ni);

                    let name_off = (line_start as usize - base as usize) as u64;
                    let name_len = (semi as usize - line_start as usize) as u16;

                    let temp = parse_temp_branchless(semi.add(1));

                    let entry = statistics.get_or_insert_stats(name_off, name_len);
                    entry.count += 1;
                    entry.min = entry.min.min(temp);
                    entry.max = entry.max.max(temp);
                    entry.total += temp as i64;

                    p = nl.add(1);
                    break;
                }

                p = p.add(8);
            }
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
        output.push('{');
        let mut sep = "";
        for (k, v) in total_statistics {
            output.push_str(sep);
            output.push_str(&format!(
                "{k}={:.1}/{:.1}/{:.1}",
                v.min as f64 / 10.0,
                v.total as f64 / 10.0 / v.count as f64,
                v.max as f64 / 10.0
            ));
            sep = ", ";
        }
        output.push('}');
        output
    })
}
