use memmap2::Mmap;
use std::cmp::max_by;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{env, io, process};
use std::{sync::Arc, thread};

const FILE: &str = "./measurements.txt";
const MIN_TEMP: i32 = -999;
const MAX_TEMP: i32 = 999;
const MAX_NAME_LENGTH: i32 = 100;
const MAX_CITIES: i32 = 10000;
const SEGMENT_SIZE: i32 = 1 << 21;
const HASH_TABLE_SIZE: i32 = 1 << 17;

fn main() -> io::Result<()> {
    let is_worker = std::env::args().any(|a| a == "--worker");
    if is_worker {
        return spawn_worker();
    }

    let number_of_workers = env::var("NUM_THREADS").map_or(
        thread::available_parallelism().map_or(1, |n| n.get()),
        |n| n.parse::<usize>().unwrap(),
    );

    let file = File::open(FILE)?;
    let mmap = unsafe { Mmap::map(&file)? };
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

            handles.push(s.spawn(move || {
                // let mut results = Vec::with_capacity(MAX_CITIES as usize);
                let results = parse_loop(data, &cursor, file_end, file_start);
                // results
                // eprintln!("Scanning done {:?}", results);
                let mut final_results = Vec::with_capacity(MAX_CITIES as usize);
                for result in results {
                    if let Some(r) = result {
                        final_results.push(r);
                    }
                }
                final_results
            }));
        }
        let mut all_results = Vec::with_capacity(number_of_workers);
        for h in handles {
            all_results.push(h.join().expect("worker panicked"));
        }
        let final_result = accumulate_results(data, all_results);

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
    });
    Ok(())
}

fn spawn_worker() -> io::Result<()> {
    let exe = std::env::current_exe()?;
    let mut cmd = Command::new(exe);
    let mut child = cmd.arg("--worker").stdout(Stdio::piped()).spawn()?;
    let pipe = child.stdout.take().expect("stdout was piped");
    let mut result = String::new();
    BufReader::new(pipe).read_line(&mut result)?;
    println!("{result}");
    Ok(())
}

fn parse_loop(
    data: &[u8],
    _cursor: &AtomicUsize,
    _file_end: usize,
    _file_start: usize,
    // collected_results: &mut Vec<Result>,
) -> Vec<Option<Result>> {
    let mut results: Vec<Option<Result>> = vec![None; HASH_TABLE_SIZE as usize];

    loop {
        // let last_pos = _cursor.load(Ordering::SeqCst);
        let current = _cursor.fetch_add(SEGMENT_SIZE as usize, std::sync::atomic::Ordering::SeqCst);
        // let new_pos = _cursor.load(Ordering::SeqCst);
        // eprintln!("last_pos = {last_pos:?}, current={current}, new_pos = {new_pos}, SEGMENT_SIZE={SEGMENT_SIZE}, SEGMENT_SIZE as usize={}", SEGMENT_SIZE as usize);
        if current >= _file_end {
            return results;
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

        // let dist = (segment_end - segment_start) / 3;
        // let midpoint_1 = next_newline(data, segment_start + dist);
        // let midpoint_2 = next_newline(data, segment_start + dist + dist);

        let mut scanner_1 = Scanner::new(data, segment_start, segment_end);
        // let mut scanner_2 = Scanner::new(data, midpoint_1 + 1, midpoint_2);
        // let mut scanner_3 = Scanner::new(data, midpoint_2 + 1, segment_end);

        // while scanner_1.has_next() && scanner_2.has_next() && scanner_3.has_next_safe() {
        let mut foo = 0;
        while scanner_1.has_next() {
            foo += 1;
            if foo % 100_000 == 0 {
                // eprintln!("scanner_1.pos {} in {current}", scanner_1.pos);
            }
            let word_1 = scanner_1.get_long();
            // let word_2 = scanner_2.get_long();
            // let word_3 = scanner_3.get_long();

            let delimiter_mask_1 = find_delimiter(word_1);
            // let delimiter_mask_2 = find_delimiter(word_2);
            // let delimiter_mask_3 = find_delimiter(word_3);

            let word_1b = scanner_1.get_long_at(scanner_1.pos + 8);
            // let word_2b = scanner_2.get_long_at(scanner_2.pos + 8);
            // let word_3b = scanner_3.get_long_at(scanner_3.pos + 8);

            let delimiter_mask_1b = find_delimiter(word_1b);
            // let delimiter_mask_2b = find_delimiter(word_2b);
            // let delimiter_mask_3b = find_delimiter(word_3b);

            let existing_result_1 = find_result(
                word_1,
                delimiter_mask_1,
                word_1b,
                delimiter_mask_1b,
                &mut scanner_1,
                &mut results,
                // collected_results,
                _file_end,
            );

            let number_1 = scan_number(&mut scanner_1);

            existing_result_1.record(number_1)
        }

        // eprintln!("main loop done");

        if segment_end < _file_end {
            while scanner_1.has_next() {
                let word = scanner_1.get_long();
            }
        } else {
            while scanner_1.has_next_safe() {
                todo!()
            }
            // slow path for last lines of the file
            todo!()
        }
        // while scanner_2.has_next() {
        //     todo!()
        // }
        // if segment_end < _file_end {
        //     while scanner_3.has_next() {
        //         todo!()
        //     }
        // } else {
        //     while scanner_3.has_next_safe() {
        //         todo!()
        //     }
        //     // slow path for last lines of the file
        //     todo!()
        // }

        // eprintln!("{current} {segment_start} {segment_end} {dist} {midpoint_1} {midpoint_2}")
    }
}

/// Port of the Java `scanNumber`, returning `i16`.
///
/// Assumptions (same as the Java trick):
/// - The number starts at `scanner.pos + 1` (i.e. right after ';').
/// - The textual format is fixed-width like `[-]dd.d`,
///   and you want to advance past the number (matches `+ 4`).
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
    scanner.add(adv);

    number
}

/// Branchless ASCII-to-integer conversion (ported from the Java version by Quan Anh Mai).
///
/// `decimal_sep_pos` is the bit index of the decimal separator position as produced by the
/// `trailingZeros` trick (i.e. Java `Long.numberOfTrailingZeros(...)`).
#[inline(always)]
fn convert_into_number(decimal_sep_pos: u32, number_word: u64) -> u64 {
    // Java: int shift = 28 - decimalSepPos;
    // In practice decimal_sep_pos is small (fits), but keep it signed to match Java semantics.
    let shift: i32 = 28 - (decimal_sep_pos as i32);

    // Java:
    //   long signed = (~numberWord << 59) >> 63;
    //
    // This yields 0xFFFF...FFFF if negative, 0 otherwise.
    // Do it in u64 with explicit "arith shift" emulation via i64 cast.
    let signed: u64 = ((((!(number_word)) << 59) as i64) >> 63) as u64;

    // Java: long designMask = ~(signed & 0xFF);
    // If negative, clears low byte (removes '-'); otherwise all 1s.
    let design_mask: u64 = !(signed & 0xFF);

    // Java:
    //   long digits = ((numberWord & designMask) << shift) & 0x0F000F0F00L;
    //
    // Java shifts are masked (for long: & 63). Rust shifts must be < 64, so do the same masking.
    let sh: u32 = (shift as u32) & 63;
    let digits: u64 = ((number_word & design_mask) << sh) & 0x0F00_0F0F_00u64;

    // Java:
    //   long absValue = ((digits * 0x640a0001) >>> 32) & 0x3FF;
    let abs_value: u64 = ((digits.wrapping_mul(0x640A_0001u64)) >> 32) & 0x3FF;

    // Java: return (absValue ^ signed) - signed;
    // If signed == all-ones, this negates abs_value (two's complement); else returns abs_value.
    (abs_value ^ signed).wrapping_sub(signed)
}

/// Branchless ASCII-to-i16 conversion (ported from Java by Quan Anh Mai).
///
/// `decimal_sep_pos` is the *bit index* of the decimal separator (from trailing_zeros).
#[inline(always)]
fn convert_into_number_i16(decimal_sep_pos: u32, number_word: u64) -> i16 {
    // Java: int shift = 28 - decimalSepPos;
    let shift: i32 = 28 - decimal_sep_pos as i32;

    // Java: long signed = (~numberWord << 59) >> 63;
    // Result: 0xFFFF...FFFF if negative, 0 otherwise
    let signed: i64 = (((!number_word << 59) as i64) >> 63);

    // Java: long designMask = ~(signed & 0xFF);
    let design_mask: u64 = !((signed as u64) & 0xFF);

    // Java shifts are masked (& 63); Rust requires us to do this explicitly
    let sh = (shift as u32) & 63;

    // Java: ((numberWord & designMask) << shift) & 0x0F000F0F00L
    let digits: u64 = ((number_word & design_mask) << sh) & 0x0F00_0F0F_00u64;

    // Java: ((digits * 0x640a0001) >>> 32) & 0x3FF
    let abs_value: i64 = (((digits.wrapping_mul(0x640A_0001u64)) >> 32) & 0x3FF) as i64;

    // Java: (absValue ^ signed) - signed
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
        self.pos < self.end
    }

    fn has_next_safe(&self) -> bool {
        self.pos < self.end - 16
    }

    fn get_long(&self) -> u64 {
        self.get_long_at(self.pos)
    }

    #[inline(always)]
    fn get_long_at(&self, pos: usize) -> u64 {
        let len = self.data.len();

        let b: [u8; 8] = if pos + 8 <= len {
            // Fast path: full 8 bytes available
            self.data[pos..pos + 8].try_into().unwrap()
        } else {
            // Slow path: fewer than 8 bytes, pad with 0
            let mut tmp = [0u8; 8];
            let available = len.saturating_sub(pos);
            tmp[..available].copy_from_slice(&self.data[pos..len]);
            tmp
        };

        u64::from_le_bytes(b)
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
    pub fn get_u64_at(&self, off: usize) -> u64 {
        let mut buf = [0u8; 8];

        let len = self.data.len();
        if off < len {
            let n = (len - off).min(8);
            buf[..n].copy_from_slice(&self.data[off..off + n]);
        }

        u64::from_le_bytes(buf)
    }
}

#[inline(always)]
fn find_delimiter(word: u64) -> u64 {
    let input = word ^ 0x3B3B3B3B3B3B3B3Bu64; // ';' = 0x3B
    (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
}

#[inline(always)]
fn first_delim_offset(mask: u64) -> Option<usize> {
    if mask == 0 {
        None
    } else {
        Some((mask.trailing_zeros() as usize) >> 3)
    }
}

#[inline]
pub fn next_newline_safe(bytes: &[u8], mut prev: usize) -> usize {
    // Search until we find '\n' (0x0A). Caller must ensure a '\n' exists
    // before end of slice, or this will read past bounds (panic) / loop forever.
    loop {
        // Read 8 bytes starting at prev (unaligned ok). Bounds-checked via slicing.
        let word = u64::from_le_bytes(bytes[prev..prev + 8].try_into().unwrap());

        let input = word ^ 0x0A0A0A0A0A0A0A0Au64;
        let pos = (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64;

        if pos != 0 {
            // pos has 0x80 in the byte lane(s) that matched
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

#[derive(Clone, Debug, Copy)]
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

    // #[inline(always)]
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

fn new_entry(
    // table: &'b mut [Option<Result>],
    name_address: usize,
    _table_index: usize,
    name_length: usize,
    scanner: &mut Scanner,
    // collected: &'b mut Vec<Result>,
) -> Result {
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
    // table[table_index] = Some(result);
    // table[table_index].as_mut().unwrap()
    result
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

/// Returns the found/created Result.
fn find_result<'a, 'b>(
    initial_word: u64,
    initial_delim_mask: u64,
    word_b: u64,
    delim_mask_b: u64,
    scanner: &mut Scanner<'a>,
    results: &'b mut [Option<Result>],
    // collected_results: &'b mut Vec<Result>,
    file_end: usize,
) -> &'b mut Result {
    // eprintln!("in find_result");
    let mut word = initial_word;
    let mut delimiter_mask = initial_delim_mask;
    let mut hash: u64;
    let name_address = scanner.pos;
    let mut word2 = word_b;
    let delimiter_mask2 = delim_mask_b;

    if scanner.pos > file_end - 120 {
        // eprintln!("In super-slow path...");
        // thread slowly
        hash = word ^ word2;

        // Advance by up to 16 bytes, but never past scanner.end
        let remaining = scanner.end.saturating_sub(scanner.pos);
        let adv = remaining.min(16);
        scanner.add(adv);

        // Scan 8 bytes at a time as long as it's safe.
        while scanner.pos + 8 <= scanner.end {
            word = scanner.get_u64_at(scanner.pos);
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

        // Tail: fewer than 8 bytes left; scan bytewise for ';'
        if delimiter_mask == 0 {
            while scanner.pos < scanner.end {
                if scanner.data[scanner.pos] == b';' {
                    break;
                }
                hash ^= scanner.data[scanner.pos] as u64; // or omit; depends on your intended hash semantics
                scanner.add(1);
            }
        }
    } else {
        // Fast path: ';' found within first 16 bytes.
        if (delimiter_mask | delimiter_mask2) != 0 {
            // eprintln!("fast path");
            let letter_count1 = (delimiter_mask.trailing_zeros() as usize) >> 3;
            let letter_count2 = (delimiter_mask2.trailing_zeros() as usize) >> 3;

            let mask = MASK2[letter_count1];
            word &= MASK1[letter_count1];
            word2 = mask & word2 & MASK1[letter_count2];

            hash = word ^ word2;

            let i1 = letter_count1 + ((letter_count2 as u64 & mask) as usize);
            // eprintln!(
            //     "fast path, advancing with {i1} (letter_count1={letter_count1} letter_count2={letter_count2})"
            // );
            scanner.add(i1);

            let idx = hash_to_index(hash, results.len());

            // let result = &mut results[idx];

            if let Some(Result {
                first_name_word,
                second_name_word,
                ..
            }) = results[idx]
            {
                if first_name_word == word && second_name_word == word2 {
                    return results[idx].as_mut().unwrap();
                }
            }
        } else {
            // eprintln!("slow path");

            // Slow path: ';' not in first 16 bytes.
            hash = word ^ word2;
            scanner.add(16);

            loop {
                // eprintln!("gonna get u64 at {}", scanner.pos);
                word = scanner.get_u64();
                delimiter_mask = find_delimiter(word);

                if delimiter_mask != 0 {
                    let tz = delimiter_mask.trailing_zeros() as usize;
                    word = word << (63 - tz);
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

    // eprintln!("in find_result, after initial if");

    let name_length = scanner.pos() - name_address;

    let mut table_index = hash_to_index(hash, results.len());

    let mut max_iters = 0;
    let num_results = results.len();
    'outer: loop {
        max_iters += 1;
        if max_iters > 10 {
            eprintln!("slow path, max_iters = {max_iters}");
        }
        if results[table_index].is_none() {
            let result = new_entry(
                // results,
                name_address,
                table_index,
                name_length,
                scanner,
                // collected_results,
            );
            results[table_index] = Some(result);
        }

        let mut inner_iters = 0;
        let end = name_length + 1;
        let mut i: usize = 0;
        while i + 8 < end {
            inner_iters += 1;
            if inner_iters > 100 {
                eprintln!("stuck? inner_iters = {inner_iters}");
            }
            assert!(name_length + 1 - 8 < 100_000_000, "wrap around");
            if scanner.get_u64_at(results[table_index].as_mut().unwrap().name_address + i)
                != scanner.get_u64_at(name_address + i)
            {
                // Collision error, try next.
                continue 'outer;
            }
            i += 8;
        }

        let remaining_shift = 64 - ((name_length + 1 - i) << 3);

        let a = scanner.get_u64_at(results[table_index].as_mut().unwrap().name_address + i);
        let b = scanner.get_u64_at(name_address + i);

        if ((a ^ b) << remaining_shift) == 0 {
            // eprintln!("got it at index {table_index}");
            return results[table_index].as_mut().unwrap();
        } else {
            // Collision error, try next.
            table_index = (table_index + 31) & (num_results - 1);
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
