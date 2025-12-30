use memmap2::Mmap;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{env, io};
use std::{sync::Arc, thread};

const FILE: &str = "./measurements.txt";
const MIN_TEMP: i32 = -999;
const MAX_TEMP: i32 = 999;
const MAX_NAME_LENGTH: i32 = 100;
const MAX_CITIES: i32 = 10000;
const SEGMENT_SIZE: i32 = 1 << 21;
const HASH_TABLE_SIZE: i32 = 1 << 17;

unsafe fn madvise(ptr: *const u8, len: usize, advice: i32) {
    unsafe {
        libc::madvise(ptr as *mut libc::c_void, len, advice);
    }
}

fn main() -> io::Result<()> {
    let is_worker = std::env::args().any(|a| a == "--worker");
    if !is_worker {
        return spawn_worker();
    }

    let number_of_workers = env::var("NUM_THREADS").map_or(
        thread::available_parallelism().map_or(1, |n| n.get()),
        |n| n.parse::<usize>().unwrap(),
    );

    let file = File::open(FILE)?;
    let mmap = unsafe { Mmap::map(&file)? };
    unsafe {
        madvise(mmap.as_ptr(), mmap.len(), libc::MADV_HUGEPAGE);
        madvise(mmap.as_ptr(), mmap.len(), libc::MADV_SEQUENTIAL);
        madvise(mmap.as_ptr(), mmap.len(), libc::MADV_WILLNEED);
    }

    let data = &mmap;

    let file_start = 0;
    let file_end = file_start + mmap.len();
    let cursor = Arc::new(AtomicUsize::new(file_start));

    thread::scope(|s| {
        let mut handles = Vec::with_capacity(number_of_workers);
        for _ in 0..number_of_workers {
            let cursor = Arc::clone(&cursor);
            let file_start = file_start;
            let file_end = file_end;

            handles.push(s.spawn(move || parse_loop(data, &cursor, file_end, file_start)));
        }
        let mut all_results = Vec::with_capacity(number_of_workers);
        for h in handles {
            all_results.push(h.join().expect("worker panicked"));
        }
        let final_result = accumulate_results(data, all_results);
        let total_rows: usize = final_result.iter().map(|(_, v)| v.count).sum();
        let mut report = String::new();
        report.push('{');
        let mut prefix = "";
        for (station, stats) in final_result {
            report += prefix;
            prefix = ", ";
            report += &format!(
                "{station}={:.1}/{:.1}/{:.1}",
                stats.min as f32 / 10.0,
                (stats.sum as f32 / 10.0) / stats.count as f32,
                stats.max as f32 / 10.0
            );
        }
        report.push('}');

        println!("{report}");
        eprintln!("({total_rows} rows processed)");
    });
    Ok(())
}

fn spawn_worker() -> io::Result<()> {
    let exe = env::current_exe()?;
    let mut cmd = Command::new(exe);
    let mut child = cmd.arg("--worker").stdout(Stdio::piped()).spawn()?;
    let pipe = child.stdout.take().expect("stdout was piped");
    let mut result = String::new();
    BufReader::new(pipe).read_line(&mut result)?;
    println!("{result}");
    Ok(())
}

struct ProbeCounters {
    lookups: u64,
    probes: u64,
    fast_hits: u64,
    full_compares: u64,
}

fn parse_loop(
    data: &[u8],
    _cursor: &AtomicUsize,
    _file_end: usize,
    _file_start: usize,
) -> Vec<Result> {
    // Hash table: 0 = empty, else (idx + 1) into `out`
    let mut table: Vec<u32> = vec![0; HASH_TABLE_SIZE as usize];
    // Collected results (like Java's collectedResults)
    let mut out: Vec<Result> = Vec::with_capacity(MAX_CITIES as usize);

    // let mut probe_counter = ProbeCounters {
    //     lookups: 0,
    //     probes: 0,
    //     fast_hits: 0,
    //     full_compares: 0,
    // };

    // let mut seg_nl = 0;
    //
    // let mut rows: u64 = 0;
    // let mut last_segment_start = 0;
    // let mut last_segment_end = 0;
    // let mut last_scanner_pos = 0;
    loop {
        let current = _cursor.fetch_add(SEGMENT_SIZE as usize, Ordering::SeqCst);
        if current >= _file_end {
            // eprintln!("rows={rows} unique={}", out.len());
            //
            // eprintln!(
            //     "lookups={} probes={} probes/lookup={:.3} fast_hits={} full_compares={}",
            //     probe_counter.lookups,
            //     probe_counter.probes,
            //     (probe_counter.probes as f64)/(probe_counter.lookups as f64),
            //     probe_counter.fast_hits,
            //     probe_counter.full_compares,
            // );

            // if rows != seg_nl {
            //     eprintln!(
            //         "MISMATCH previous segment: current={} rows={rows} seg_nl={seg_nl} last_segment_start={last_segment_start} last_segment_end={last_segment_end} last_scanner_pos={last_scanner_pos}",
            //         current - SEGMENT_SIZE as usize
            //     );
            // }

            return out;
        }

        let segment_end = next_newline(
            data,
            usize::min(_file_end - 1, current + SEGMENT_SIZE as usize),
        );

        let segment_start = if current == _file_start {
            current
        } else {
            next_newline(data, current) + 1
        };

        if current == 0 {
            assert_eq!(segment_start, 0);
        }

        // last_segment_end = segment_end;
        // last_segment_start = segment_start;
        // rows = 0;
        // seg_nl = memchr::memchr_iter(b'\n', &data[segment_start..=segment_end]).count() as u64;

        let mut scanner_1 = Scanner::new(data, segment_start, segment_end);
        // verify_after_newline(data, current, _file_start, &scanner_1);

        // if segment_end >= 13795437781 {
        // eprintln!(
        //     "current={current} segment_start={segment_start} segment_end={segment_end} _file_end={_file_end} scanner_1.pos={}",
        //     scanner_1.pos
        // );
        // }

        while scanner_1.has_next_safe() {
            // verify_after_newline(data, current, _file_start, &scanner_1);

            let word_1 = scanner_1.get_u64_unsafe();
            let delim_1 = find_delimiter(word_1);

            let word_1b = scanner_1.get_u64_at_unsafe(scanner_1.pos + 8);
            let delim_1b = find_delimiter(word_1b);

            let r = find_result_unsafe_idx(
                word_1,
                delim_1,
                word_1b,
                delim_1b,
                &mut scanner_1,
                &mut table,
                &mut out,
                _file_end,
                // &mut probe_counter
            );

            let number_1 = scan_number_unsafe(&mut scanner_1);
            r.record(number_1);
            // rows += 1;
        }

        if segment_end < _file_end - 1 {
            while scanner_1.has_next() {
                // verify_after_newline(data, current, _file_start, &scanner_1);

                let word_1 = scanner_1.get_u64_unsafe();
                let delim_1 = find_delimiter(word_1);
                let word_1b = scanner_1.get_u64_at_unsafe(scanner_1.pos + 8);
                let delim_1b = find_delimiter(word_1b);

                let r = find_result_unsafe_idx(
                    word_1,
                    delim_1,
                    word_1b,
                    delim_1b,
                    &mut scanner_1,
                    &mut table,
                    &mut out,
                    _file_end,
                    // &mut probe_counter
                );

                let number_1 = scan_number_unsafe(&mut scanner_1);
                r.record(number_1);
                // rows += 1;
            }
        } else {
            while scanner_1.has_next_safe() {
                // eprintln!(
                //     "last segment A: segment_start={segment_start} segment_end={segment_end} _file_end={_file_end} scanner.pos={}",
                //     scanner_1.pos
                // );
                // verify_after_newline(data, current, _file_start, &scanner_1);

                let word_1 = scanner_1.get_u64_unsafe();
                let delim_1 = find_delimiter(word_1);
                let word_1b = scanner_1.get_u64_at_unsafe(scanner_1.pos + 8);
                let delim_1b = find_delimiter(word_1b);

                let r = find_result_unsafe_idx(
                    word_1,
                    delim_1,
                    word_1b,
                    delim_1b,
                    &mut scanner_1,
                    &mut table,
                    &mut out,
                    _file_end,
                    // &mut probe_counter
                );

                let number_1 = scan_number_unsafe(&mut scanner_1);
                r.record(number_1);
                // rows += 1;
            }

            while scanner_1.has_next() {
                // eprintln!(
                //     "last segment B: segment_start={segment_start} segment_end={segment_end} _file_end={_file_end} scanner.pos={} context={:?}",
                //     scanner_1.pos,
                //     &data[scanner_1.pos - 2..scanner_1.pos + 3]
                // );
                //
                // verify_after_newline(data, current, _file_start, &scanner_1);
                let word_1 = scanner_1.get_u64();
                let delim_1 = find_delimiter(word_1);
                let word_1b = scanner_1.get_u64_at(scanner_1.pos + 8);
                let delim_1b = find_delimiter(word_1b);

                // let before = scanner_1.pos;

                let r = find_result_idx(
                    word_1,
                    delim_1,
                    word_1b,
                    delim_1b,
                    &mut scanner_1,
                    &mut table,
                    &mut out,
                    _file_end,
                );

                let number_1 = scan_number(&mut scanner_1);
                r.record(number_1);
                // rows += 1;

                // let after = scanner_1.pos;
                //
                // if before >= _file_end - 2000 {
                //     let nls = memchr::memchr_iter(b'\n', &data[before..after]).count();
                //     eprintln!(
                //         "row: before={before} after={after} crossed_newlines={nls} scanned_row={:?}",
                //         String::from_utf8_lossy(&data[before..after])
                //     );
                //     if nls != 1 {
                //         let lo = before.saturating_sub(10);
                //         let hi = (after + 10).min(data.len());
                //         eprintln!("context:\n{:?}", String::from_utf8_lossy(&data[lo..hi]));
                //         // std::process::abort();
                //     }
                // }
            }

            // eprintln!("giving up this segment at {}", scanner_1.pos);
        }
        // last_scanner_pos = scanner_1.pos;
    }
}

fn verify_after_newline(data: &[u8], current: usize, file_start: usize, scanner: &Scanner) {
    assert_eq!(file_start, 0);
    if current > 0 {
        assert_eq!(data[scanner.pos - 1], b'\n');
    }
}

#[inline(always)]
fn scan_number(scanner: &mut Scanner) -> i16 {
    // Java: scanPtr.getLongAt(scanPtr.pos() + 1)
    let number_word: u64 = scanner.get_u64_at(scanner.pos() + 1);

    // Java: Long.numberOfTrailingZeros(~numberWord & 0x10101000L)
    let decimal_sep_pos: u32 = ((!number_word) & 0x1010_1000u64).trailing_zeros();

    // Java: convertIntoNumber(decimalSepPos, numberWord)
    let number: i16 = convert_into_number_i16(decimal_sep_pos, number_word);

    // Java: scanPtr.add((decimalSepPos >>> 3) + 4)
    let adv: usize = ((decimal_sep_pos >> 3) as usize) + 4;
    // if scanner.pos > 13795437694 {
    //     eprintln!(
    //         "scan_number. number_word={number_word} decimal_sep_pos={decimal_sep_pos} number={number} adv={adv}"
    //     )
    // }
    scanner.add(adv);

    number
}

/// Port of the Java `scanNumber`, returning `i16`.
///
/// Assumptions (same as the Java trick):
/// - The number starts at `scanner.pos + 1` (i.e. right after ';').
/// - The textual format is fixed-width like `[-]dd.d`,
///   and you want to advance past the number (matches `+ 4`).
#[inline(always)]
fn scan_number_unsafe(scanner: &mut Scanner) -> i16 {
    // Java: scanPtr.getLongAt(scanPtr.pos() + 1)
    let number_word: u64 = scanner.get_u64_at_unsafe(scanner.pos() + 1);

    // Java: Long.numberOfTrailingZeros(~numberWord & 0x10101000L)
    let decimal_sep_pos: u32 = ((!number_word) & 0x1010_1000u64).trailing_zeros();

    // Java: convertIntoNumber(decimalSepPos, numberWord)
    let number: i16 = convert_into_number_i16(decimal_sep_pos, number_word);

    // Java: scanPtr.add((decimalSepPos >>> 3) + 4)
    let adv: usize = ((decimal_sep_pos >> 3) as usize) + 4;
    scanner.add(adv);

    number
}

/// Branchless ASCII-to-i16 conversion (ported from Java by Quan Anh Mai).
///
/// `decimal_sep_pos` is the *bit index* of the decimal separator (from trailing_zeros).
#[inline(always)]
fn convert_into_number_i16(decimal_sep_pos: u32, number_word: u64) -> i16 {
    let shift: i32 = 28 - decimal_sep_pos as i32;
    let signed: i64 = (((!number_word << 59) as i64) >> 63);
    let design_mask: u64 = !((signed as u64) & 0xFF);
    let sh = (shift as u32) & 63;
    let digits: u64 = ((number_word & design_mask) << sh) & 0x0F00_0F0F_00u64;
    let abs_value: i64 = (((digits.wrapping_mul(0x640A_0001u64)) >> 32) & 0x3FF) as i64;
    ((abs_value ^ signed) - signed) as i16
}

struct Scanner<'a> {
    data: &'a [u8],
    start: usize,
    end: usize,
    pos: usize,
}

impl<'a> Scanner<'a> {
    fn new(data: &'a [u8], start: usize, end: usize) -> Scanner<'a> {
        Self {
            data,
            start,
            end,
            pos: start,
        }
    }

    fn has_next(&self) -> bool {
        // if self.pos > 13795437780 - 1000 {
        //     eprintln!("checking scanner.has_next near end pos={}", self.pos);
        // }
        self.pos < self.end
    }

    fn has_next_safe(&self) -> bool {
        // if self.pos > 13795437780 - 1000 {
        //     eprintln!("checking scanner.has_next_safe near end pos={}", self.pos);
        // }
        self.pos < self.end - (MAX_NAME_LENGTH as usize + 7)
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn add(&mut self, n: usize) {
        self.pos += n
    }

    #[inline]
    pub fn get_u64(&mut self) -> u64 {
        self.get_u64_at(self.pos)
    }

    #[inline]
    pub fn get_u64_unsafe(&mut self) -> u64 {
        self.get_u64_at_unsafe(self.pos)
    }

    #[inline]
    pub fn get_u64_at(&self, off: usize) -> u64 {
        let mut buf = [0u8; 8];

        let len = self.data.len();
        if off < len {
            let n = (len - off).min(8);
            buf[..n].copy_from_slice(&self.data[off..off + n]);
        }

        u64::from_le_bytes(buf)
    }

    #[inline(always)]
    fn get_u64_at_unsafe(&self, pos: usize) -> u64 {
        unsafe { u64::from_le((self.data.as_ptr().add(pos) as *const u64).read_unaligned()) }
    }
}

#[inline(always)]
fn find_delimiter(word: u64) -> u64 {
    let input = word ^ 0x3B3B3B3B3B3B3B3Bu64; // ';' = 0x3B
    (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
}

#[inline]
pub fn next_newline_safe(bytes: &[u8], mut prev: usize) -> usize {
    loop {
        let word = u64::from_le_bytes(bytes[prev..prev + 8].try_into().unwrap());

        let input = word ^ 0x0A0A0A0A0A0A0A0Au64;
        let pos = (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64;

        if pos != 0 {
            prev += (pos.trailing_zeros() as usize) >> 3;
            return prev;
        } else {
            prev += 8;
        }
    }
}

#[inline]
pub fn next_newline(data: &[u8], prev: usize) -> usize {
    prev + memchr::memchr(b'\n', &data[prev..]).expect("expected newline")
}

#[inline]
pub unsafe fn next_newline_ptr(mut p: *const u8) -> *const u8 {
    loop {
        let word = (p as *const u64).read_unaligned(); // native endian load
        let word = u64::from_le(word); // match little-endian files / x86 behavior

        let input = word ^ 0x0A0A0A0A0A0A0A0Au64;
        let pos = (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64;

        if pos != 0 {
            p = p.add((pos.trailing_zeros() as usize) >> 3);
            return p;
        } else {
            p = p.add(8);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Result {
    pub first_name_word: u64,
    pub second_name_word: u64,
    pub name_address: usize, // offset into scanner.data
    // ... other fields
    min: i16,
    max: i16,
    sum: i64,
    count: usize,
}

impl Result {
    pub(crate) fn calc_name(&self, data: &[u8]) -> String {
        let start = self.name_address.min(data.len());
        let bytes = &data[start..];

        // Prefer ';' as terminator (works even if you don't NUL-terminate).
        let end = bytes
            .iter()
            .position(|&b| b == b';' || b == 0)
            .unwrap_or(bytes.len());

        // Station names are typically ASCII/UTF-8; fall back lossily if needed.
        String::from_utf8_lossy(&bytes[..end]).into_owned()
    }

    /// Merge another `Result` into `self` (min/max/sum/count).
    #[inline(always)]
    fn accumulate(&mut self, other: &Self) {
        if other.count == 0 {
            return;
        }
        if self.count == 0 {
            self.min = other.min;
            self.max = other.max;
        } else {
            if other.min < self.min {
                self.min = other.min;
            }
            if other.max > self.max {
                self.max = other.max;
            }
        }

        self.sum += other.sum;
        self.count += other.count;
    }

    #[inline(always)]
    fn record(&mut self, number: i16) {
        // min / max update
        if number < self.min {
            self.min = number;
        }
        if number > self.max {
            self.max = number;
        }

        // sum / count
        self.sum += number as i64;
        self.count += 1;
    }
}

#[inline(always)]
fn hash_to_index(hash: u64, table_len: usize) -> usize {
    debug_assert!(table_len.is_power_of_two());
    let hash_as_int = hash ^ (hash >> 33) ^ (hash >> 15);
    (hash_as_int as usize) & (table_len - 1)
}

fn new_entry(name_address: usize, name_length: usize, scanner: &mut Scanner) -> Result {
    let mut result = Result {
        first_name_word: scanner.get_u64_at(name_address),
        second_name_word: scanner.get_u64_at(name_address + 8),
        name_address,
        min: i16::MAX,
        max: i16::MIN,
        sum: 0,
        count: 0,
    };
    let total_length = name_length + 1;
    if total_length <= 8 {
        result.first_name_word &= MASK1[total_length - 1];
        result.second_name_word = 0;
    } else if total_length < 16 {
        result.second_name_word &= MASK1[total_length - 9];
    }
    result.name_address = name_address;
    result
}

fn new_entry_unsafe(name_address: usize, name_length: usize, scanner: &Scanner) -> Result {
    let mut r = Result {
        first_name_word: scanner.get_u64_at_unsafe(name_address),
        second_name_word: scanner.get_u64_at_unsafe(name_address + 8),
        name_address,
        min: i16::MAX,
        max: i16::MIN,
        sum: 0,
        count: 0,
    };
    let total_length = name_length + 1;
    if total_length <= 8 {
        r.first_name_word &= MASK1[total_length - 1];
        r.second_name_word = 0;
    } else if total_length < 16 {
        r.second_name_word &= MASK1[total_length - 9];
    }
    r
}

const MASK1: [u64; 9] = [
    0x00000000000000FF,
    0x000000000000FFFF,
    0x0000000000FFFFFF,
    0x00000000FFFFFFFF,
    0x000000FFFFFFFFFF,
    0x0000FFFFFFFFFFFF,
    0x00FFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
];

const MASK2: [u64; 9] = [
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xFFFFFFFFFFFFFFFF,
];

#[inline(always)]
fn find_result_idx<'a, 'b>(
    initial_word: u64,
    initial_delim_mask: u64,
    word_b: u64,
    delim_mask_b: u64,
    scanner: &mut Scanner<'a>,
    table: &mut [u32],
    out: &'b mut Vec<Result>,
    file_end: usize,
) -> &'b mut Result {
    let mut word = initial_word;
    let mut delimiter_mask = initial_delim_mask;
    let mut hash: u64;
    let name_address = scanner.pos;
    let mut word2 = word_b;
    let delimiter_mask2 = delim_mask_b;

    if scanner.pos > file_end - 120 {
        // Tail-end safe path: DO NOT advance blindly. Scan from current position.
        hash = word ^ word2;
        //
        // // Scan 8 bytes at a time as long as it's safe.
        // while scanner.pos + 8 <= scanner.end {
        //     word = scanner.get_u64_at(scanner.pos);
        //     delimiter_mask = find_delimiter(word);
        //
        //     if delimiter_mask != 0 {
        //         let tz = delimiter_mask.trailing_zeros() as usize;
        //         word <<= 63 - tz;
        //         scanner.add(tz >> 3);
        //         hash ^= word;
        //         break;
        //     } else {
        //         scanner.add(8);
        //         hash ^= word;
        //     }
        // }

        // Tail: fewer than 8 bytes left; scan bytewise for ';'
        // if delimiter_mask == 0 {
        while scanner.pos < scanner.end {
            let b = scanner.data[scanner.pos];
            if b == b';' {
                break;
            }
            hash ^= b as u64; // keep if this matches your intended hash semantics
            scanner.add(1);
        }
        // }
    } else {
        if (delimiter_mask | delimiter_mask2) != 0 {
            let letter_count1 = (delimiter_mask.trailing_zeros() as usize) >> 3;
            let letter_count2 = (delimiter_mask2.trailing_zeros() as usize) >> 3;

            let mask = MASK2[letter_count1];
            word &= MASK1[letter_count1];
            word2 = mask & word2 & MASK1[letter_count2];

            hash = word ^ word2;

            let i1 = letter_count1 + ((letter_count2 as u64 & mask) as usize);
            scanner.add(i1);

            let idx = hash_to_index(hash, table.len());

            // Fast hit: existing entry and both name words match
            let entry = table[idx];
            if entry != 0 {
                let r = &out[(entry - 1) as usize];
                if r.first_name_word == word && r.second_name_word == word2 {
                    return &mut out[(entry - 1) as usize];
                }
            }
        } else {
            hash = word ^ word2;
            scanner.add(16);

            loop {
                word = scanner.get_u64();
                delimiter_mask = find_delimiter(word);

                if delimiter_mask != 0 {
                    let tz = delimiter_mask.trailing_zeros() as usize;
                    word <<= 63 - tz;
                    scanner.add(tz >> 3);
                    hash ^= word;
                    break;
                } else {
                    scanner.add(8);
                    hash ^= word;
                }
            }
        }
    }

    let name_length = scanner.pos() - name_address;

    let mut table_index = hash_to_index(hash, table.len());
    let mask = table.len() - 1; // table len is power-of-two

    'outer: loop {
        let entry = table[table_index];
        if entry == 0 {
            // Insert new Result
            let r = new_entry(name_address, name_length, scanner);
            out.push(r);
            table[table_index] = out.len() as u32; // idx+1
        }

        let idx = (table[table_index] - 1) as usize;
        let existing_addr = out[idx].name_address;

        // Collision check (8-byte chunks)
        let end = name_length + 1;
        let mut i: usize = 0;
        while i + 8 < end {
            if scanner.get_u64_at(existing_addr + i) != scanner.get_u64_at(name_address + i) {
                table_index = (table_index + 31) & mask;
                continue 'outer;
            }
            i += 8;
        }

        let remaining_shift = 64 - ((end - i) << 3);

        let a = scanner.get_u64_at(existing_addr + i);
        let b = scanner.get_u64_at(name_address + i);

        if ((a ^ b) << remaining_shift) == 0 {
            return &mut out[idx];
        } else {
            table_index = (table_index + 31) & mask;
        }
    }
}

#[inline(always)]
unsafe fn table_get(table: &[u32], i: usize) -> u32 {
    *table.get_unchecked(i)
}
#[inline(always)]
unsafe fn table_set(table: &mut [u32], i: usize, v: u32) {
    *table.get_unchecked_mut(i) = v;
}
#[inline(always)]
unsafe fn out_get(out: &[Result], i: usize) -> &Result {
    out.get_unchecked(i)
}
#[inline(always)]
unsafe fn out_get_mut(out: &mut [Result], i: usize) -> &mut Result {
    out.get_unchecked_mut(i)
}

#[inline(always)]
fn find_result_unsafe_idx<'a, 'b>(
    initial_word: u64,
    initial_delim_mask: u64,
    word_b: u64,
    delim_mask_b: u64,
    scanner: &mut Scanner<'a>,
    table: &mut [u32],
    out: &'b mut Vec<Result>,
    file_end: usize,
    // probe_counters: &mut ProbeCounters,
) -> &'b mut Result {
    let mut word = initial_word;
    let mut delimiter_mask = initial_delim_mask;
    let mut hash: u64;
    let name_address = scanner.pos;
    let mut word2 = word_b;
    let delimiter_mask2 = delim_mask_b;

    if scanner.pos > file_end - 120 {
        hash = word ^ word2;

        while scanner.pos + 8 <= scanner.end {
            word = scanner.get_u64_at_unsafe(scanner.pos);
            delimiter_mask = find_delimiter(word);

            if delimiter_mask != 0 {
                let tz = delimiter_mask.trailing_zeros() as usize;
                word <<= 63 - tz;
                scanner.add(tz >> 3);
                hash ^= word;
                break;
            } else {
                scanner.add(8);
                hash ^= word;
            }
        }

        if delimiter_mask == 0 {
            while scanner.pos < scanner.end {
                let b = scanner.data[scanner.pos];
                if b == b';' {
                    break;
                }
                hash ^= b as u64;
                scanner.add(1);
            }
        }
    } else {
        if (delimiter_mask | delimiter_mask2) != 0 {
            let letter_count1 = (delimiter_mask.trailing_zeros() as usize) >> 3;
            let letter_count2 = (delimiter_mask2.trailing_zeros() as usize) >> 3;

            let mask = MASK2[letter_count1];
            word &= MASK1[letter_count1];
            word2 = mask & word2 & MASK1[letter_count2];

            hash = word ^ word2;

            let i1 = letter_count1 + ((letter_count2 as u64 & mask) as usize);
            scanner.add(i1);

            // probe_counters.lookups += 1;
            let idx = hash_to_index(hash, table.len());

            // Fast hit: existing entry and both name words match
            let entry = unsafe { table_get(table, idx) };
            if entry != 0 {
                let r = unsafe { out_get(out, (entry - 1) as usize) };
                if r.first_name_word == word && r.second_name_word == word2 {
                    // return &mut out[(entry - 1) as usize];
                    // probe_counters.fast_hits += 1;
                    return unsafe { out_get_mut(out, (entry - 1) as usize) };
                }
            }
        } else {
            hash = word ^ word2;
            scanner.add(16);

            loop {
                word = scanner.get_u64_unsafe();
                delimiter_mask = find_delimiter(word);

                if delimiter_mask != 0 {
                    let tz = delimiter_mask.trailing_zeros() as usize;
                    word <<= 63 - tz;
                    scanner.add(tz >> 3);
                    hash ^= word;
                    break;
                } else {
                    scanner.add(8);
                    hash ^= word;
                }
            }
        }
    }

    let name_length = scanner.pos() - name_address;

    let mut table_index = hash_to_index(hash, table.len());
    let mask = table.len() - 1; // table len is power-of-two

    'outer: loop {
        // probe_counters.probes += 1;
        let entry = unsafe { table_get(table, table_index) };
        if entry == 0 {
            // Insert new Result
            let r = new_entry_unsafe(name_address, name_length, scanner);
            out.push(r);
            unsafe { table_set(table, table_index, out.len() as u32) };
        }

        let idx = (unsafe { table_get(table, table_index) } - 1) as usize;
        let existing_addr = unsafe { out_get(out, idx) }.name_address;

        // Collision check (8-byte chunks)
        let end = name_length + 1;
        let mut i: usize = 0;
        while i + 8 < end {
            // probe_counters.full_compares += 1;
            if scanner.get_u64_at_unsafe(existing_addr + i)
                != scanner.get_u64_at_unsafe(name_address + i)
            {
                table_index = (table_index + 31) & mask;
                continue 'outer;
            }
            i += 8;
        }

        let remaining_shift = 64 - ((end - i) << 3);

        let a = scanner.get_u64_at_unsafe(existing_addr + i);
        let b = scanner.get_u64_at_unsafe(name_address + i);

        if ((a ^ b) << remaining_shift) == 0 {
            return unsafe { out_get_mut(out, idx) };
        } else {
            table_index = (table_index + 31) & mask;
        }
    }
}

fn accumulate_results(data: &[u8], all_results: Vec<Vec<Result>>) -> BTreeMap<String, Result> {
    let mut result: BTreeMap<String, Result> = BTreeMap::new();

    for result_arr in all_results {
        for r in result_arr {
            match result.entry(r.calc_name(data).to_owned()) {
                std::collections::btree_map::Entry::Vacant(e) => {
                    e.insert(r);
                }
                std::collections::btree_map::Entry::Occupied(mut e) => {
                    e.get_mut().accumulate(&r);
                }
            }
        }
    }

    result
}
