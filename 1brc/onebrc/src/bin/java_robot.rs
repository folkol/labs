use memmap2::MmapOptions;
use std::collections::BTreeMap;
use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

const FILE_PATH: &str = "../java-orig/measurements.txt";
const MIN_TEMP: i16 = -999;
const MAX_TEMP: i16 = 999;
const MAX_CITIES: usize = 10000;
const SEGMENT_SIZE: usize = 1 << 21;
const HASH_TABLE_SIZE: usize = 1 << 17;

fn round(value: f64) -> f64 {
    (value * 10.0).round() / 10.0
}

struct ResultEntry {
    first_name_word: u64,
    second_name_word: u64,
    min: i16,
    max: i16,
    count: i32,
    sum: i64,
    name_offset: usize,
}

impl ResultEntry {
    fn new(name_offset: usize) -> Self {
        Self {
            first_name_word: 0,
            second_name_word: 0,
            min: MAX_TEMP,
            max: MIN_TEMP,
            count: 0,
            sum: 0,
            name_offset,
        }
    }

    fn accumulate(&mut self, other: &ResultEntry) {
        if other.min < self.min {
            self.min = other.min;
        }
        if other.max > self.max {
            self.max = other.max;
        }
        self.sum += other.sum;
        self.count += other.count;
    }

    fn calc_name(&self, data: &[u8]) -> String {
        let mut name_length = 0;
        while data[self.name_offset + name_length] != b';' {
            name_length += 1;
        }
        let slice = &data[self.name_offset..self.name_offset + name_length];
        String::from_utf8_lossy(slice).into_owned()
    }

    fn format_stats(&self) -> String {
        format!(
            "{:.1}/{:.1}/{:.1}",
            round(self.min as f64 / 10.0),
            round((self.sum as f64 / 10.0) / self.count as f64),
            round(self.max as f64 / 10.0)
        )
    }
}

struct Scanner<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Scanner<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn has_next(&self) -> bool {
        self.pos < self.data.len()
    }

    fn get_long(&self) -> u64 {
        self.get_long_at(self.pos)
    }

    fn get_long_at(&self, pos: usize) -> u64 {
        if pos + 8 <= self.data.len() {
            u64::from_ne_bytes(self.data[pos..pos + 8].try_into().unwrap())
        } else if pos < self.data.len() {
            let mut bytes = [0u8; 8];
            let len = self.data.len() - pos;
            bytes[..len].copy_from_slice(&self.data[pos..]);
            u64::from_ne_bytes(bytes)
        } else {
            0
        }
    }

    fn add(&mut self, delta: usize) {
        self.pos += delta;
    }
}

fn next_new_line(data: &[u8], mut prev: usize) -> usize {
    while prev + 8 <= data.len() {
        let current_word = u64::from_ne_bytes(data[prev..prev + 8].try_into().unwrap());
        let input = current_word ^ 0x0A0A0A0A0A0A0A0Au64;
        let pos = (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64;
        if pos != 0 {
            return prev + (pos.trailing_zeros() as usize >> 3);
        }
        prev += 8;
    }
    while prev < data.len() && data[prev] != b'\n' {
        prev += 1;
    }
    prev
}

const MASK1: [u64; 9] = [
    0xFF,
    0xFFFF,
    0xFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFFFF,
    0xFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
];
const MASK2: [u64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0xFFFFFFFFFFFFFFFF];

fn hash_to_index(hash: u64, size: usize) -> usize {
    let hash_as_int = hash ^ (hash >> 33) ^ (hash >> 15);
    (hash_as_int as usize) & (size - 1)
}

fn convert_into_number(decimal_sep_pos: u32, number_word: u64) -> i64 {
    let shift = 28 - decimal_sep_pos;
    let signed = !((number_word << 59) as i64) >> 63;
    let design_mask = !(signed as u64 & 0xFF);
    let digits = ((number_word & design_mask) << shift) & 0x0F000F0F00u64;
    let abs_value = ((digits.wrapping_mul(0x640a0001)) >> 32) & 0x3FF;
    (abs_value as i64 ^ signed) - signed
}

fn scan_number(scanner: &mut Scanner) -> i64 {
    let number_word = scanner.get_long_at(scanner.pos + 1);
    let decimal_sep_pos = (!number_word & 0x10101000u64).trailing_zeros();
    let number = convert_into_number(decimal_sep_pos, number_word);
    scanner.add((decimal_sep_pos >> 3) as usize + 4);
    number
}

fn find_result(
    initial_word: u64,
    initial_delimiter_mask: u64,
    word_b: u64,
    delimiter_mask_b: u64,
    scanner: &mut Scanner,
    results: &mut [Option<Box<ResultEntry>>],
    collected_results: &mut Vec<usize>,
) -> usize {
    let mut word = initial_word;
    let mut delimiter_mask = initial_delimiter_mask;
    let mut hash: u64;
    let name_offset = scanner.pos;
    let mut word2 = word_b;
    let delimiter_mask2 = delimiter_mask_b;

    if (delimiter_mask | delimiter_mask2) != 0 {
        let letter_count1 = (delimiter_mask.trailing_zeros() >> 3) as usize;
        let letter_count2 = (delimiter_mask2.trailing_zeros() >> 3) as usize;
        let mask = MASK2[letter_count1];
        word = word & MASK1[letter_count1];
        word2 = mask & word2 & MASK1[letter_count2];
        hash = word ^ word2;
        let idx = hash_to_index(hash, HASH_TABLE_SIZE);
        if let Some(ref mut existing) = results[idx] {
            if existing.first_name_word == word && existing.second_name_word == word2 {
                scanner.add(letter_count1 + (letter_count2 & mask as usize));
                return idx;
            }
        }
        scanner.add(letter_count1 + (letter_count2 & mask as usize));
    } else {
        hash = word ^ word2;
        scanner.add(16);
        loop {
            word = scanner.get_long();
            delimiter_mask = {
                let input = word ^ 0x3B3B3B3B3B3B3B3Bu64;
                (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
            };
            if delimiter_mask != 0 {
                let trailing_zeros = delimiter_mask.trailing_zeros();
                word = word << (63 - trailing_zeros);
                scanner.add((trailing_zeros >> 3) as usize);
                hash ^= word;
                break;
            } else {
                scanner.add(8);
                hash ^= word;
            }
        }
    }

    let name_length = scanner.pos - name_offset;
    let mut table_index = hash_to_index(hash, HASH_TABLE_SIZE);

    loop {
        if results[table_index].is_none() {
            let mut r = Box::new(ResultEntry::new(name_offset));
            let total_length = name_length + 1;
            r.first_name_word = scanner.get_long_at(name_offset);
            r.second_name_word = scanner.get_long_at(name_offset + 8);
            if total_length <= 8 {
                r.first_name_word &= MASK1[total_length - 1];
                r.second_name_word = 0;
            } else if total_length < 16 {
                r.second_name_word &= MASK1[total_length - 9];
            }
            results[table_index] = Some(r);
            collected_results.push(table_index);
            return table_index;
        }

        let existing = results[table_index].as_mut().unwrap();

        let mut i = 0;
        let mut collision = false;
        while i + 8 <= name_length + 1 {
            if scanner.get_long_at(existing.name_offset + i) != scanner.get_long_at(name_offset + i)
            {
                collision = true;
                break;
            }
            i += 8;
        }

        if !collision {
            let remaining_shift = 64 - (((name_length + 1 - i) << 3) & 63);
            if ((scanner.get_long_at(existing.name_offset + i)
                ^ scanner.get_long_at(name_offset + i))
                << remaining_shift)
                == 0
            {
                return table_index;
            }
        }

        table_index = (table_index + 31) & (HASH_TABLE_SIZE - 1);
    }
}

fn record(existing_result: &mut ResultEntry, number: i64) {
    if (number as i16) < existing_result.min {
        existing_result.min = number as i16;
    }
    if (number as i16) > existing_result.max {
        existing_result.max = number as i16;
    }
    existing_result.sum += number;
    existing_result.count += 1;
}

fn parse_loop(
    counter: &AtomicUsize,
    data: &[u8],
    collected_results: &mut Vec<usize>,
    hash_table: &mut [Option<Box<ResultEntry>>],
) {
    loop {
        let current_offset = counter.fetch_add(SEGMENT_SIZE, Ordering::SeqCst);
        if current_offset >= data.len() {
            return;
        }

        let raw_segment_end = (current_offset + SEGMENT_SIZE).min(data.len().saturating_sub(1));
        let segment_end = next_new_line(data, raw_segment_end);

        let segment_start = if current_offset == 0 {
            0
        } else {
            next_new_line(data, current_offset) + 1
        };

        if segment_start >= segment_end {
            continue;
        }

        let dist = (segment_end - segment_start) / 3;
        let mid_point1 = next_new_line(data, segment_start + dist);
        let mid_point2 = next_new_line(data, segment_start + dist * 2);

        let mut scanner1 = Scanner::new(&data[..mid_point1]);
        scanner1.add(segment_start);
        let mut scanner2 = Scanner::new(&data[..mid_point2]);
        scanner2.add(mid_point1 + 1);
        let mut scanner3 = Scanner::new(&data[..segment_end]);
        scanner3.add(mid_point2 + 1);

        while scanner1.has_next() && scanner2.has_next() && scanner3.has_next() {
            let word1 = scanner1.get_long();
            let word2 = scanner2.get_long();
            let word3 = scanner3.get_long();
            let dm1 = {
                let input = word1 ^ 0x3B3B3B3B3B3B3B3Bu64;
                (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
            };
            let dm2 = {
                let input = word2 ^ 0x3B3B3B3B3B3B3B3Bu64;
                (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
            };
            let dm3 = {
                let input = word3 ^ 0x3B3B3B3B3B3B3B3Bu64;
                (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
            };

            let word1b = scanner1.get_long_at(scanner1.pos + 8);
            let word2b = scanner2.get_long_at(scanner2.pos + 8);
            let word3b = scanner3.get_long_at(scanner3.pos + 8);
            let dm1b = {
                let input = word1b ^ 0x3B3B3B3B3B3B3B3Bu64;
                (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
            };
            let dm2b = {
                let input = word2b ^ 0x3B3B3B3B3B3B3B3Bu64;
                (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
            };
            let dm3b = {
                let input = word3b ^ 0x3B3B3B3B3B3B3B3Bu64;
                (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
            };

            let res_idx1 = find_result(
                word1,
                dm1,
                word1b,
                dm1b,
                &mut scanner1,
                hash_table,
                collected_results,
            );
            let res_idx2 = find_result(
                word2,
                dm2,
                word2b,
                dm2b,
                &mut scanner2,
                hash_table,
                collected_results,
            );
            let res_idx3 = find_result(
                word3,
                dm3,
                word3b,
                dm3b,
                &mut scanner3,
                hash_table,
                collected_results,
            );

            let num1 = scan_number(&mut scanner1);
            let num2 = scan_number(&mut scanner2);
            let num3 = scan_number(&mut scanner3);

            record(hash_table[res_idx1].as_mut().unwrap(), num1);
            record(hash_table[res_idx2].as_mut().unwrap(), num2);
            record(hash_table[res_idx3].as_mut().unwrap(), num3);
        }

        while scanner1.has_next() {
            let word = scanner1.get_long();
            let dm = {
                let input = word ^ 0x3B3B3B3B3B3B3B3Bu64;
                (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
            };
            let word_b = scanner1.get_long_at(scanner1.pos + 8);
            let dm_b = {
                let input = word_b ^ 0x3B3B3B3B3B3B3B3Bu64;
                (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
            };
            let res_idx = find_result(
                word,
                dm,
                word_b,
                dm_b,
                &mut scanner1,
                hash_table,
                collected_results,
            );
            let num = scan_number(&mut scanner1);
            record(hash_table[res_idx].as_mut().unwrap(), num);
        }
        while scanner2.has_next() {
            let word = scanner2.get_long();
            let dm = {
                let input = word ^ 0x3B3B3B3B3B3B3B3Bu64;
                (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
            };
            let word_b = scanner2.get_long_at(scanner2.pos + 8);
            let dm_b = {
                let input = word_b ^ 0x3B3B3B3B3B3B3B3Bu64;
                (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
            };
            let res_idx = find_result(
                word,
                dm,
                word_b,
                dm_b,
                &mut scanner2,
                hash_table,
                collected_results,
            );
            let num = scan_number(&mut scanner2);
            record(hash_table[res_idx].as_mut().unwrap(), num);
        }
        while scanner3.has_next() {
            let word = scanner3.get_long();
            let dm = {
                let input = word ^ 0x3B3B3B3B3B3B3B3Bu64;
                (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
            };
            let word_b = scanner3.get_long_at(scanner3.pos + 8);
            let dm_b = {
                let input = word_b ^ 0x3B3B3B3B3B3B3B3Bu64;
                (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64
            };
            let res_idx = find_result(
                word,
                dm,
                word_b,
                dm_b,
                &mut scanner3,
                hash_table,
                collected_results,
            );
            let num = scan_number(&mut scanner3);
            record(hash_table[res_idx].as_mut().unwrap(), num);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args[1] != "--worker" {
        spawn_worker();
        return;
    }

    let file = File::open(FILE_PATH).expect("Could not open file");
    let mmap = unsafe { MmapOptions::new().map(&file).expect("Could not map file") };
    let data = &mmap[..];
    let cursor = AtomicUsize::new(0);

    let num_workers = thread::available_parallelism().unwrap().get();

    thread::scope(|s| {
        let mut handles = vec![];
        for _ in 0..num_workers {
            handles.push(s.spawn(|| {
                let mut collected_results = Vec::with_capacity(MAX_CITIES);
                let mut hash_table = (0..HASH_TABLE_SIZE)
                    .map(|_| None)
                    .collect::<Vec<Option<Box<ResultEntry>>>>();
                parse_loop(&cursor, data, &mut collected_results, &mut hash_table);
                (collected_results, hash_table)
            }));
        }

        let mut all_results = BTreeMap::new();
        for handle in handles {
            let (collected_indices, hash_table) = handle.join().unwrap();
            for idx in collected_indices {
                let r = hash_table[idx].as_ref().unwrap();
                let name = r.calc_name(data);
                all_results
                    .entry(name)
                    .and_modify(|existing: &mut ResultEntry| {
                        existing.accumulate(r);
                    })
                    .or_insert_with(|| ResultEntry {
                        first_name_word: r.first_name_word,
                        second_name_word: r.second_name_word,
                        min: r.min,
                        max: r.max,
                        count: r.count,
                        sum: r.sum,
                        name_offset: r.name_offset,
                    });
            }
        }

        print!("{{");
        for (i, (name, r)) in all_results.iter().enumerate() {
            print!("{}={}", name, r.format_stats());
            if i < all_results.len() - 1 {
                print!(", ");
            }
        }
        println!("}}");
    });
}

fn spawn_worker() {
    let mut child = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--worker")
        .stdout(std::process::Stdio::inherit())
        .spawn()
        .expect("failed to execute child");
    child.wait().expect("failed to wait on child");
}
