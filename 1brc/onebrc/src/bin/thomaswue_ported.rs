use memmap2::Mmap;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, stdout};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{env, io, process};
use std::{sync::Arc, thread};

const FILE: &str = "./measurements.txt";
const MAX_CITIES: i32 = 10000;
const SEGMENT_SIZE: i32 = 1 << 21;
const HASH_TABLE_SIZE: i32 = 1 << 17;

unsafe fn madvise(ptr: *const u8, len: usize, advice: i32) {
    unsafe {
        libc::madvise(ptr as *mut libc::c_void, len, advice);
    }
}

fn main() -> io::Result<()> {
    let is_worker = env::args().any(|a| a == "--worker");
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
        madvise(mmap.as_ptr(), mmap.len(), libc::MADV_SEQUENTIAL);
    }
    let file_len = mmap.len();
    let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize };
    let mapped_len = (file_len + page - 1) & !(page - 1);
    // SAFETY: the VMA backing the mapping is page-granular, so bytes up to mapped_len
    // are mapped. We only *interpret* bytes beyond file_len as padding.
    let data: &[u8] = unsafe { std::slice::from_raw_parts(mmap.as_ptr(), mapped_len) };

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
        let mut report = String::with_capacity(final_result.len() * 20);
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
        stdout().flush().unwrap();
        process::exit(0);
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
    process::exit(0);
}

pub struct Result {
    pub first_name_word: u64,
    pub second_name_word: u64,
    pub name_address: usize,
    pub count: u32,
    pub sum: i64,
    pub min: i16,
    pub max: i16,
}

impl Result {
    #[inline(always)]
    fn record(&mut self, number: i16) {
        if number < self.min {
            self.min = number;
        }
        if number > self.max {
            self.max = number;
        }

        self.sum += number as i64;
        self.count += 1;
    }

    pub(crate) fn calc_name(&self, data: &[u8]) -> String {
        let start = self.name_address.min(data.len());
        let bytes = &data[start..];

        let end = bytes
            .iter()
            .position(|&b| b == b';' || b == 0)
            .unwrap_or(bytes.len());
        String::from_utf8_lossy(&bytes[..end]).into_owned()
    }

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
}

fn parse_loop(
    data: &[u8],
    cursor: &AtomicUsize,
    file_end: usize,
    file_start: usize,
) -> Vec<Result> {
    let mut table: Vec<u32> = vec![0; HASH_TABLE_SIZE as usize];

    let mut results: Vec<Result> = Vec::with_capacity(MAX_CITIES as usize);

    loop {
        let current = cursor.fetch_add(SEGMENT_SIZE as usize, Ordering::Relaxed);
        if current >= file_end {
            return results;
        }

        let segment_start = if current == file_start {
            current
        } else {
            next_newline(data, current) + 1
        };

        let segment_end_nl = next_newline(
            data,
            usize::min(file_end - 1, current + SEGMENT_SIZE as usize),
        );
        let segment_end = segment_end_nl + 1;

        let dist = (segment_end - segment_start) / 3;
        let mid1_nl = next_newline(data, segment_start + dist);
        let mid2_nl = next_newline(data, segment_start + 2 * dist);

        let mut scanner_1 = Scanner::new(data, segment_start, mid1_nl + 1);
        let mut scanner_2 = Scanner::new(data, mid1_nl + 1, mid2_nl + 1);
        let mut scanner_3 = Scanner::new(data, mid2_nl + 1, segment_end);

        while scanner_1.has_next_safe() && scanner_2.has_next_safe() && scanner_3.has_next_safe() {
            let word_1 = scanner_1.get_u64_unsafe();
            let word_2 = scanner_2.get_u64_unsafe();
            let word_3 = scanner_3.get_u64_unsafe();

            let delim_1 = find_delimiter(word_1);
            let delim_2 = find_delimiter(word_2);
            let delim_3 = find_delimiter(word_3);

            let word_1b = scanner_1.get_u64_at_unsafe(scanner_1.pos + 8);
            let word_2b = scanner_2.get_u64_at_unsafe(scanner_2.pos + 8);
            let word_3b = scanner_3.get_u64_at_unsafe(scanner_3.pos + 8);

            let delim_1b = find_delimiter(word_1b);
            let delim_2b = find_delimiter(word_2b);
            let delim_3b = find_delimiter(word_3b);

            let index1 = find_result_unsafe_idx(
                word_1,
                delim_1,
                word_1b,
                delim_1b,
                &mut scanner_1,
                &mut table,
                &mut results,
                file_end,
            );
            let index2 = find_result_unsafe_idx(
                word_2,
                delim_2,
                word_2b,
                delim_2b,
                &mut scanner_2,
                &mut table,
                &mut results,
                file_end,
            );
            let index3 = find_result_unsafe_idx(
                word_3,
                delim_3,
                word_3b,
                delim_3b,
                &mut scanner_3,
                &mut table,
                &mut results,
                file_end,
            );

            let number_1 = scan_number_unsafe(&mut scanner_1);
            let number_2 = scan_number_unsafe(&mut scanner_2);
            let number_3 = scan_number_unsafe(&mut scanner_3);

            // SAFETY: We only ever push to the Vec, so index should be valid
            unsafe { results.get_unchecked_mut(index1).record(number_1) };
            unsafe { results.get_unchecked_mut(index2).record(number_2) };
            unsafe { results.get_unchecked_mut(index3).record(number_3) };
        }

        while scanner_1.has_next_safe() {
            let word_1 = scanner_1.get_u64_unsafe();
            let delim_1 = find_delimiter(word_1);
            let word_1b = scanner_1.get_u64_at_unsafe(scanner_1.pos + 8);
            let delim_1b = find_delimiter(word_1b);

            let index = find_result_unsafe_idx(
                word_1,
                delim_1,
                word_1b,
                delim_1b,
                &mut scanner_1,
                &mut table,
                // &mut station_keys,
                // &mut station_stats,
                &mut results,
                file_end,
            );

            let number_1 = scan_number_unsafe(&mut scanner_1);
            // unsafe { stat_get_mut(&mut station_stats, index).record(number_1) };
            unsafe { results.get_unchecked_mut(index).record(number_1) };
        }
        while scanner_2.has_next_safe() {
            let word_2 = scanner_2.get_u64_unsafe();
            let delim_2 = find_delimiter(word_2);
            let word_2b = scanner_2.get_u64_at_unsafe(scanner_2.pos + 8);
            let delim_2b = find_delimiter(word_2b);

            let index = find_result_unsafe_idx(
                word_2,
                delim_2,
                word_2b,
                delim_2b,
                &mut scanner_2,
                &mut table,
                // &mut station_keys,
                // &mut station_stats,
                &mut results,
                file_end,
            );

            let number_2 = scan_number_unsafe(&mut scanner_2);
            // unsafe { stat_get_mut(&mut station_stats, index).record(number_2) };
            unsafe { results.get_unchecked_mut(index).record(number_2) };
        }
        while scanner_3.has_next_safe() {
            let word_3 = scanner_3.get_u64_unsafe();
            let delim_3 = find_delimiter(word_3);
            let word_1b = scanner_3.get_u64_at_unsafe(scanner_3.pos + 8);
            let delim_1b = find_delimiter(word_1b);

            let index = find_result_unsafe_idx(
                word_3,
                delim_3,
                word_1b,
                delim_1b,
                &mut scanner_3,
                &mut table,
                // &mut station_keys,
                // &mut station_stats,
                &mut results,
                file_end,
            );

            let number_3 = scan_number_unsafe(&mut scanner_3);
            // unsafe { stat_get_mut(&mut station_stats, index).record(number_3) };
            unsafe { results.get_unchecked_mut(index).record(number_3) };
        }
    }
}

/// Port of the Java `scanNumber`, returning `i16`.
///
/// Assumptions (same as the Java trick):
/// - The number starts at `scanner.pos + 1` (i.e. right after ';').
/// - The textual format is fixed-width like `[-]dd.d`,
///   and you want to advance past the number (matches `+ 4`).
#[inline(always)]
fn scan_number_unsafe(scanner: &mut Scanner) -> i16 {
    let number_word: u64 = scanner.get_u64_at_unsafe(scanner.pos() + 1);
    let decimal_sep_pos: u32 = ((!number_word) & 0x1010_1000u64).trailing_zeros();
    let number: i16 = convert_into_number_i16(decimal_sep_pos, number_word);
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
    let signed: i64 = ((!number_word << 59) as i64) >> 63;
    let design_mask: u64 = !((signed as u64) & 0xFF);
    let sh = (shift as u32) & 63;
    let digits: u64 = ((number_word & design_mask) << sh) & 0x0F00_0F0F_00u64;
    let abs_value: i64 = (((digits.wrapping_mul(0x640A_0001u64)) >> 32) & 0x3FF) as i64;
    ((abs_value ^ signed) - signed) as i16
}

struct Scanner<'a> {
    data: &'a [u8],
    end: usize,
    pos: usize,
}

impl<'a> Scanner<'a> {
    fn new(data: &'a [u8], start: usize, end: usize) -> Scanner<'a> {
        Self {
            data,
            end,
            pos: start,
        }
    }

    #[inline(always)]
    fn has_next_safe(&self) -> bool {
        self.pos + 16 < self.end
    }

    #[inline(always)]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline(always)]
    pub fn add(&mut self, n: usize) {
        self.pos += n
    }

    #[inline(always)]
    pub fn get_u64_unsafe(&mut self) -> u64 {
        self.get_u64_at_unsafe(self.pos)
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
pub fn next_newline(data: &[u8], prev: usize) -> usize {
    prev + memchr::memchr(b'\n', &data[prev..]).expect("expected newline")
    // next_newline_swar(data, prev)
}

/// Find the offset (in bytes) from `prev` to the next `\n` byte, scanning 8 bytes at a time.
/// `data` must contain a `\n` at/after `prev` before the end of the slice, otherwise this loops forever
/// (same as the Java version).
#[inline(always)]
pub fn next_newline_swar(data: &[u8], mut prev: usize) -> usize {
    const NL: u64 = 0x0A0A0A0A0A0A0A0A;
    const ONES: u64 = 0x0101010101010101;
    const HIGHS: u64 = 0x8080808080808080;

    // You *can* keep this as a debug assert, or turn it into a checked loop below.
    debug_assert!(prev <= data.len());

    unsafe {
        loop {
            // Equivalent to Unsafe.getLong(prev) in Java: unaligned 8-byte load.
            let p = data.as_ptr().add(prev) as *const u64;
            let current_word = p.read_unaligned();

            // SWAR "has zero byte" after XOR with '\n'
            let input = current_word ^ NL;
            let pos = (input.wrapping_sub(ONES)) & !input & HIGHS;

            if pos != 0 {
                // tz / 8 gives the byte index of the first matching byte in the word
                prev += (pos.trailing_zeros() as usize) >> 3;
                return prev;
            } else {
                prev += 8;
                // If you want safety instead of "Java semantics", add:
                // if prev + 8 > data.len() { return data.len(); }
            }
        }
    }
}

//     private static long nextNewLine(long prev) {
//         while (true) {
//             long currentWord = Scanner.UNSAFE.getLong(prev);
//             long input = currentWord ^ 0x0A0A0A0A0A0A0A0AL;
//             long pos = (input - 0x0101010101010101L) & ~input & 0x8080808080808080L;
//             if (pos != 0) {
//                 prev += Long.numberOfTrailingZeros(pos) >>> 3;
//                 break;
//             }
//             else {
//                 prev += 8;
//             }
//         }
//         return prev;
//     }

#[inline(always)]
fn hash_to_index(hash: u64, table_len: usize) -> usize {
    debug_assert!(table_len.is_power_of_two());
    let hash_as_int = hash ^ (hash >> 33) ^ (hash >> 15);
    (hash_as_int as usize) & (table_len - 1)
}

#[inline(always)]
fn new_entry_unsafe(name_address: usize, name_length: usize, scanner: &Scanner) -> Result {
    let mut result = Result {
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
        result.first_name_word &= unsafe { *MASK1.get_unchecked(total_length - 1) };
        result.second_name_word = 0;
    } else if total_length < 16 {
        result.second_name_word &= unsafe { *MASK1.get_unchecked(total_length - 9) };
    }
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

#[inline(always)]
unsafe fn table_get(table: &[u32], i: usize) -> u32 {
    unsafe { *table.get_unchecked(i) }
}
#[inline(always)]
unsafe fn table_set(table: &mut [u32], i: usize, v: u32) {
    unsafe {
        *table.get_unchecked_mut(i) = v;
    }
}

#[inline(always)]
fn find_result_unsafe_idx<'a, 'b>(
    initial_word: u64,
    initial_delim_mask: u64,
    word_b: u64,
    delim_mask_b: u64,
    scanner: &mut Scanner<'a>,
    table: &mut [u32],
    results: &mut Vec<Result>,
    file_end: usize,
) -> usize {
    let orig_w1 = initial_word;
    let orig_w2 = word_b;

    let mut word = initial_word;
    let mut word2 = word_b;

    let mut delimiter_mask = initial_delim_mask;
    let delimiter_mask2 = delim_mask_b;

    let mut hash: u64;
    let name_address = scanner.pos;

    let mut cmp_w1 = orig_w1;
    let mut cmp_w2 = orig_w2;

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
                let b = unsafe { scanner.data.get_unchecked(scanner.pos) };
                if b == &b';' {
                    break;
                }
                hash ^= *b as u64;
                scanner.add(1);
            }
        }
    } else if (delimiter_mask | delimiter_mask2) != 0 {
        // ';' in first 16 bytes: compute the masked signature exactly like Java.
        let letter_count1 = (delimiter_mask.trailing_zeros() as usize) >> 3;
        let letter_count2 = (delimiter_mask2.trailing_zeros() as usize) >> 3;

        let mask = MASK2[letter_count1];
        word &= MASK1[letter_count1];
        word2 = mask & word2 & MASK1[letter_count2];

        cmp_w1 = word;
        cmp_w2 = word2;

        hash = word ^ word2;

        let i1 = letter_count1 + ((letter_count2 as u64 & mask) as usize);
        scanner.add(i1);
    } else {
        // Slow path: ';' not in first 16 bytes.
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

    let name_length = scanner.pos() - name_address;

    let mut table_index = hash_to_index(hash, table.len());
    let mask = table.len() - 1;

    loop {
        let mut entry = unsafe { table_get(table, table_index) };
        if entry == 0 {
            let result = new_entry_unsafe(name_address, name_length, scanner);
            results.push(result);
            entry = results.len() as u32;
            unsafe { table_set(table, table_index, entry) };
            return entry as usize - 1;
        }

        let idx = (entry - 1) as usize;

        let r = unsafe { results.get_unchecked(idx) };
        if r.first_name_word == cmp_w1 && r.second_name_word == cmp_w2 {
            return idx;
        }

        let existing_addr = r.name_address;

        let end = name_length + 1;
        let mut i: usize = 0;
        while i + 8 < end {
            if scanner.get_u64_at_unsafe(existing_addr + i)
                != scanner.get_u64_at_unsafe(name_address + i)
            {
                table_index = (table_index + 31) & mask;
                continue;
            }
            i += 8;
        }

        let remaining_shift = 64 - ((end - i) << 3);
        let a = scanner.get_u64_at_unsafe(existing_addr + i);
        let b = scanner.get_u64_at_unsafe(name_address + i);

        if ((a ^ b) << remaining_shift) == 0 {
            return idx;
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
