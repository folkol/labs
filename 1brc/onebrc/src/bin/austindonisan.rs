#![allow(unsafe_op_in_unsafe_fn)]
use std::arch::x86_64::*;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::mem::{self, align_of, size_of};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::slice;
use libc::{
    c_void, close, fstat, fputs, fork, madvise, mmap, open, pipe, poll, pollfd, qsort_r, read,
    sched_getaffinity, sched_setaffinity, sprintf, stat, wait, write, CPU_COUNT, CPU_ISSET, CPU_SET,
    CPU_ZERO, MAP_ANONYMOUS, MAP_FIXED, MAP_PRIVATE, MAP_SHARED, O_RDONLY, POLLIN, PROT_READ, PROT_WRITE,
    MADV_HUGEPAGE, MADV_POPULATE_WRITE,
};

const UNMAP: bool = false;
const PIN_CPU: bool = true;
const DEBUG: bool = false;

const HASH_SHIFT: u32 = 17;
const HASH_LONG_SHIFT: u32 = 14;
const HASH_RESULT_SHIFT: u32 = 14;

const MAX_CITIES: usize = 10000;
const MAX_TEMP: i32 = 999;
const MIN_TEMP: i32 = -999;

const SHORT_CITY_LENGTH: usize = 32;
const LONG_CITY_LENGTH: usize = 128;

const HASH_ENTRIES: usize = 1 << HASH_SHIFT;
const HASH_LONG_ENTRIES: usize = 1 << HASH_LONG_SHIFT;

const STRIDE: usize = 8;

#[derive(Clone, Copy)]
#[repr(C)]
struct HashEntry {
    packed_sum: i64,
    min: i32,
    max: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct HashPointers {
    packed_offsets: *mut i32,
    hashed_cities: *mut c_void,
    hashed_storage: *mut c_void,
    hashed_cities_long: *mut c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct HashCounts {
    num_cities: i32,
    num_cities_long: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Hash {
    p: HashPointers,
    counts: HashCounts,
}

#[repr(C)]
struct Worker {
    start: i64,
    end: i64,
    fd: i32,
    worker_id: i32,
    cpu_id: i32,
    fork_proc: bool,
    warmup: bool,
    first: bool,
    last: bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct HashRow {
    packed_sum_count: i64,
    min: i32,
    max: i32,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct LongCityRef {
    sentinel: i32,
    index: i32,
    padding: [i32; 6],
}

#[derive(Clone, Copy)]
#[repr(C)]
union ShortCity {
    reg: __m256i,
    bytes: [u8; SHORT_CITY_LENGTH],
}

#[derive(Clone, Copy)]
#[repr(C)]
union LongCity {
    regs: [__m256i; 4],
    bytes: [u8; LONG_CITY_LENGTH],
}

#[derive(Clone, Copy)]
#[repr(C)]
union PackedCity {
    reg: __m256i,
    short_city: ShortCity,
    long_ref: ManuallyDrop<LongCityRef>,
}

use std::mem::ManuallyDrop;

#[derive(Clone, Copy)]
#[repr(C)]
struct ResultsRow {
    city: PackedCity,
    sum: i64,
    count: i32,
    min: i16,
    max: i16,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct ResultsRef {
    offset: u32,
}

#[repr(C)]
struct Results {
    num_cities: i32,
    num_long_cities: i32,
    refs: *mut ResultsRef,
    rows: *mut ResultsRow,
    long_cities: *mut LongCity,
}

const PAGE_SIZE: usize = 0x1000;
const HUGE_PAGE_SIZE: usize = 0x200000;
const PAGE_MASK: usize = !(PAGE_SIZE - 1);
const HUGE_PAGE_MASK: usize = !(HUGE_PAGE_SIZE - 1);

const fn page_trunc(v: usize) -> usize { v & PAGE_MASK }
const fn huge_page_trunc(v: usize) -> usize { v & HUGE_PAGE_MASK }
const fn page_ceil(v: usize) -> usize { page_trunc(v + PAGE_SIZE - 1) }
const fn huge_page_ceil(v: usize) -> usize { huge_page_trunc(v + HUGE_PAGE_SIZE - 1) }

const LINE_SIZE: usize = 64;
const LINE_MASK: usize = !(LINE_SIZE - 1);
const fn line_trunc(v: usize) -> usize { v & LINE_MASK }
const fn line_ceil(v: usize) -> usize { line_trunc(v + LINE_SIZE - 1) }

const HASH_ENTRY_SIZE: usize = STRIDE * size_of::<HashEntry>();

const HASH_DATA_OFFSET: u32 = 5;
const HASH_CITY_OFFSET: u32 = 5;
const HASH_CITY_LONG_OFFSET: u32 = 7;

const fn min_u32(a: u32, b: u32) -> u32 { if a < b { a } else { b } }

const HASH_SHORT_MASK: u32 = ((1 << HASH_SHIFT) - 1) << min_u32(HASH_DATA_OFFSET, HASH_CITY_OFFSET);
const HASH_LONG_MASK: u32 = ((1 << HASH_LONG_SHIFT) - 1) << HASH_CITY_LONG_OFFSET;
const HASH_RESULT_MASK: u32 = ((1 << HASH_RESULT_SHIFT) - 1) << HASH_CITY_OFFSET;

const PACKED_OFFSETS_SIZE: usize = page_ceil(size_of::<i32>() * MAX_CITIES);
const HASHED_CITIES_SIZE: usize = huge_page_ceil(SHORT_CITY_LENGTH * HASH_ENTRIES);
const HASHED_DATA_SIZE: usize = huge_page_ceil(HASH_ENTRY_SIZE * HASH_ENTRIES);
const HASHED_CITIES_LONG_SIZE: usize = huge_page_ceil(LONG_CITY_LENGTH * HASH_LONG_ENTRIES);

const HASH_MEMORY_SIZE: usize = PACKED_OFFSETS_SIZE + HASHED_CITIES_SIZE + HASHED_DATA_SIZE + HASHED_CITIES_LONG_SIZE;

const RESULTS_SIZE: usize = line_ceil(size_of::<Results>());
const RESULTS_REFS_SIZE: usize = line_ceil(size_of::<ResultsRef>() * MAX_CITIES);
const RESULTS_ROWS_SIZE: usize = line_ceil(size_of::<ResultsRow>() * HASH_ENTRIES);
const RESULTS_LONG_CITIES_SIZE: usize = line_ceil(size_of::<LongCity>() * MAX_CITIES);

const RESULTS_MEMORY_SIZE: usize = page_ceil(RESULTS_SIZE + RESULTS_REFS_SIZE + RESULTS_ROWS_SIZE + RESULTS_LONG_CITIES_SIZE);

const MMAP_DATA_SIZE: usize = 1 << 32;
const DUMMY_SIZE: usize = PAGE_SIZE;
const TRAILING_SPACE: usize = PAGE_SIZE;
const MAX_CHUNK_SIZE: usize = MMAP_DATA_SIZE - DUMMY_SIZE - TRAILING_SPACE;

const SUM_BITS: u32 = 35;
const SUM_SIGN_BIT: i64 = 1 << SUM_BITS;
const COUNT_BITS_START: u32 = SUM_BITS + 1;

fn extract_count(v: i64) -> i32 { (v >> COUNT_BITS_START) as i32 }
const SUM_MASK: i64 = (1 << COUNT_BITS_START) - 1;
fn extract_sum(v: i64) -> i64 { (v & SUM_MASK) - SUM_SIGN_BIT }

const LONG_CITY_SENTINEL: i32 = 0xFACADE00u32 as i32;

#[repr(align(32))]
struct MaskedDummy([u8; 32]);
static MASKED_DUMMY: MaskedDummy = MaskedDummy([
    0,
    0x41, // 'A'
    0x44, // 'D'
    0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
]);

#[repr(align(64))]
struct CityMask([i64; 8]);
static CITY_MASK: CityMask = CityMask([-1, -1, -1, -1, 0, 0, 0, 0]);

fn main() {
    unsafe {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 || args.len() > 4 {
            eprintln!("Usage: 1brc file workers [warmup]");
            std::process::exit(1);
        }

        let filename = &args[1];
        let fd = open(filename.as_ptr() as *const i8, O_RDONLY);
        if fd == -1 {
            perror(b"Error opening file\0".as_ptr() as *const i8);
            std::process::exit(1);
        }

        let mut file_stat: stat = mem::zeroed();
        if fstat(fd, &mut file_stat) == -1 {
            perror(b"Error getting file size\0".as_ptr() as *const i8);
            close(fd);
            std::process::exit(1);
        }

        let mut num_workers: i32 = args[2].parse().unwrap_or(0);
        if num_workers < 1 || num_workers > 256 {
            eprintln!("workers must be between 1 and 256");
            std::process::exit(1);
        }

        let warmup = if args.len() == 4 { args[3].parse::<i32>().unwrap_or(0) != 0 } else { false };

        if (file_stat.st_size - 1) as usize / PAGE_SIZE < num_workers as usize {
            num_workers = (file_stat.st_size as usize / PAGE_SIZE + 1) as i32;
        }

        let mut mem_ptr = mmap(ptr::null_mut(), RESULTS_MEMORY_SIZE + size_of::<Worker>() * num_workers as usize, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
        let results: *mut Results = mem_ptr as *mut Results;
        setup_results(results);

        let workers_mem = mem_ptr.add(RESULTS_MEMORY_SIZE);
        let workers: *mut Worker = workers_mem as *mut Worker;
        prep_workers(workers, num_workers, warmup, fd, &file_stat);

        if UNMAP && num_workers == 1 {
            start_worker(workers, results);
        } else {
            process(0, workers, num_workers, -1, results);
        }

        qsort_r((*results).refs as *mut c_void, (*results).num_cities as usize, size_of::<ResultsRef>(), Some(sort_result), results as *mut c_void);

        print_results(results);
    }
}

unsafe fn setup_results(r: *mut Results) {
    (*r).num_cities = 0;
    (*r).num_long_cities = 0;

    let mut p = r as *mut u8;
    p = p.add(RESULTS_SIZE);

    (*r).refs = p as *mut ResultsRef;
    p = p.add(RESULTS_REFS_SIZE);

    (*r).rows = p as *mut ResultsRow;
    p = p.add(RESULTS_ROWS_SIZE);

    (*r).long_cities = p as *mut LongCity;
}

unsafe fn prep_workers(workers: *mut Worker, num_workers: i32, warmup: bool, fd: i32, file_stat: *const stat) {
    let mut cpuset: libc::cpu_set_t = mem::zeroed();
    CPU_ZERO(&mut cpuset);
    sched_getaffinity(0, size_of::<libc::cpu_set_t>(), &mut cpuset);
    let num_cpus = CPU_COUNT(&cpuset);

    if num_cpus < num_workers {
        eprintln!("{} threads is less than {} available CPUS", num_workers, num_cpus);
        std::process::exit(1);
    }

    let mut cpu = 0;
    let mut start: i64 = 0;
    let delta = page_trunc(((*file_stat).st_size as usize / num_workers as usize)) as i64;
    for i in 0..num_workers {
        while !CPU_ISSET(cpu as usize, &cpuset) {
            cpu += 1;
        }

        let w = workers.add(i as usize);
        (*w).worker_id = i;
        (*w).cpu_id = cpu;
        cpu += 1;
        (*w).fd = fd;
        (*w).start = start;
        start += delta;
        (*w).end = start;
        (*w).first = i == 0;
        (*w).last = i == num_workers - 1;
        if (*w).last {
            (*w).end = (*file_stat).st_size;
        }
        (*w).warmup = warmup;
    }
}

unsafe fn process(id: i32, workers: *mut Worker, mut num_workers: i32, fd_out: i32, out: *mut Results) {
    unsafe {
        let max_k = 8;
        let do_work = num_workers <= max_k;
        let k = if do_work { num_workers } else { (num_workers + (max_k - 1)) / max_k };

        let mut fd_pipes = vec![[0i32; 2]; k as usize];
        let mut poll_fds = vec![pollfd { fd: 0, events: 0, revents: 0 }; k as usize];

        let tmp = mmap(ptr::null_mut(), RESULTS_MEMORY_SIZE * k as usize, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_ANONYMOUS, -1, 0);
        let mut child_results = vec![ptr::null_mut::<Results>(); k as usize];
        for i in 0..k {
            child_results[i as usize] = tmp.add(i as usize * RESULTS_MEMORY_SIZE) as *mut Results;
            setup_results(child_results[i as usize]);
        }

        let mut new_id = id;
        for i in 0..k {
            if pipe(fd_pipes[i as usize].as_mut_ptr()) != 0 {
                perror(b"pipe\0".as_ptr() as *const i8);
                std::process::exit(1);
            }
            poll_fds[i as usize].fd = fd_pipes[i as usize][0];
            poll_fds[i as usize].events = POLLIN;

            let n = (num_workers + ((k - i) / 2)) / (k - i);
            num_workers -= n;

            let pid = fork();
            if pid == 0 {
                close(fd_pipes[i as usize][0]);

                if do_work {
                    if PIN_CPU {
                        let mut cpu_set: libc::cpu_set_t = mem::zeroed();
                        CPU_ZERO(&mut cpu_set);
                        CPU_SET((*workers.add(new_id as usize)).cpu_id as usize, &mut cpu_set);
                        if sched_setaffinity(0, size_of::<libc::cpu_set_t>(), &cpu_set) == -1 {
                            perror(b"sched_setaffinity\0".as_ptr() as *const i8);
                        }
                    }

                    start_worker(workers.add(new_id as usize), child_results[i as usize]);
                    if write(fd_pipes[i as usize][1], b"0".as_ptr() as *const c_void, 1) < 0 {
                        perror(b"write\0".as_ptr() as *const i8);
                        std::process::exit(1);
                    }
                    std::process::exit(0);
                }

                process(new_id, workers, n, fd_pipes[i as usize][1], child_results[i as usize]);
                if UNMAP {
                    while wait(ptr::null_mut()) != -1 {}
                }
                std::process::exit(0);
            }
            new_id += n;
        }

        let mut children_finished = 0;
        while children_finished < k {
            poll(poll_fds.as_mut_ptr(), k as u64, -1);
            for i in 0..k {
                if poll_fds[i as usize].fd == -1 { continue; }
                if (poll_fds[i as usize].revents & POLLIN) != 0 {
                    let mut buffer = [0u8; 4];
                    let num_bytes = read(poll_fds[i as usize].fd, buffer.as_mut_ptr() as *mut c_void, buffer.len());
                    if num_bytes > 0 {
                        children_finished += 1;
                        merge(out, child_results[i as usize]);
                        poll_fds[i as usize].fd = -1;
                    }
                } else if (poll_fds[i as usize].revents & (libc::POLLERR | libc::POLLHUP | libc::POLLNVAL)) != 0 {
                    children_finished += 1;
                    poll_fds[i as usize].fd = -1;
                }
            }
        }

        if fd_out != -1 {
            if write(fd_out, b"0".as_ptr() as *const c_void, 1) < 0 {
                perror(b"parent write\0".as_ptr() as *const i8);
                std::process::exit(1);
            }
            if UNMAP {
                while wait(ptr::null_mut()) != -1 {}
            }
            std::process::exit(0);
        }

        if UNMAP {
            while wait(ptr::null_mut()) != -1 {}
        }
    }
}

unsafe fn start_worker(w: *mut Worker, out: *mut Results) {
    unsafe {
        let mut hash_data = mmap(ptr::null_mut(), HASH_MEMORY_SIZE + HUGE_PAGE_SIZE, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);

        hash_data = hash_data.add(PACKED_OFFSETS_SIZE);
        hash_data = (huge_page_ceil(hash_data as usize)) as *mut c_void;
        hash_data = hash_data.sub(PACKED_OFFSETS_SIZE);

        madvise(hash_data.add(PACKED_OFFSETS_SIZE), HASHED_CITIES_SIZE + HASHED_DATA_SIZE + HASHED_CITIES_LONG_SIZE, MADV_HUGEPAGE);
        madvise(hash_data, HASH_MEMORY_SIZE, MADV_POPULATE_WRITE);

        let packed_offsets = hash_data as *mut i32;
        hash_data = hash_data.add(PACKED_OFFSETS_SIZE);

        let hashed_cities = hash_data;
        hash_data = hash_data.add(HASHED_CITIES_SIZE);

        let hashed_storage = hash_data;
        hash_data = hash_data.add(HASHED_DATA_SIZE);

        let hashed_cities_long = hash_data;

        let mut hash = Hash {
            p: HashPointers {
                packed_offsets,
                hashed_cities,
                hashed_storage,
                hashed_cities_long,
            },
            counts: HashCounts { num_cities: 0, num_cities_long: 0 },
        };

        let data = mmap(ptr::null_mut(), MMAP_DATA_SIZE, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);

        let dummy_data = _mm256_set1_epi64x(0x0A302E303B444100u64 as i64);
        for i in (0..DUMMY_SIZE).step_by(32) {
            _mm256_store_si256(data.add(i) as *mut __m256i, dummy_data);
        }

        let mut start = (*w).start;
        while start < (*w).end {
            let mut end = start + MAX_CHUNK_SIZE as i64;
            if end > (*w).end { end = (*w).end; }

            let first = (*w).first && start == (*w).start;
            let last = (*w).last && end == (*w).end;

            let chunk_size = (end - start) as u32;
            let mapped_file_length = if last { page_ceil(chunk_size as usize) } else { chunk_size as usize + PAGE_SIZE };

            mmap(data.add(DUMMY_SIZE), mapped_file_length, PROT_READ, MAP_PRIVATE | MAP_FIXED, (*w).fd, start);

            let mut offsets = [0u32; STRIDE + 1];
            if first {
                offsets[0] = DUMMY_SIZE as u32;
            }
            for i in (if first { 1 } else { 0 })..STRIDE {
                offsets[i] = find_next_row(data, (chunk_size / STRIDE as u32 * i as u32) + DUMMY_SIZE as u32);
            }
            offsets[STRIDE] = if last { chunk_size + DUMMY_SIZE as u32 } else { find_next_row(data, chunk_size + DUMMY_SIZE as u32) };

            process_chunk(data, offsets.as_ptr(), &mut hash);
            start += MAX_CHUNK_SIZE as i64;
        }

        convert_hash_to_results(&mut hash, out);
    }
}

unsafe fn find_next_row(data: *const c_void, offset: u32) -> u32 {
    let newlines = _mm256_set1_epi8('\n' as i8);
    let chars = _mm256_loadu_si256(data.add(offset as usize) as *const __m256i);
    let bytes = _mm_tzcnt_32(_mm256_movemask_epi8(_mm256_cmpeq_epi8(chars, newlines)) as u32) as u32;
    if bytes < 32 {
        return offset + bytes + 1;
    }
    let mut curr = offset;
    loop {
        if *(data.add(curr as usize) as *const u8) == b'\n' {
            return curr + 1;
        }
        curr += 1;
    }
}

unsafe fn process_chunk(base: *const c_void, offsets: *const u32, hash_out: *mut Hash) {
    let mut hash = *hash_out;
    let mut nums = [0u64; STRIDE];
    let mut starts = [0u32; STRIDE];
    let mut check_finished: bool;

    let mut starts_v = _mm256_loadu_si256(offsets as *const __m256i);
    let mut ends_v = _mm256_loadu_si256(offsets.add(1) as *const __m256i);
    let mut finished_v = _mm256_set1_epi32(0);

    let mut at_end_mask = _mm256_cmpeq_epi32(starts_v, ends_v);
    check_finished = _mm256_testz_si256(at_end_mask, at_end_mask) == 0;

    _mm256_store_si256(starts.as_mut_ptr() as *mut __m256i, starts_v);

    let dummy = _mm256_load_si256(MASKED_DUMMY.0.as_ptr() as *const __m256i);
    _mm256_store_si256(hash.p.hashed_cities.add(hash_city(dummy) as usize) as *mut __m256i, dummy);

    loop {
        if check_finished {
            finished_v = _mm256_or_si256(finished_v, at_end_mask);

            if _mm256_movemask_epi8(finished_v) == -1 {
                (*hash_out).counts = hash.counts;
                return;
            }

            starts_v = _mm256_andnot_si256(finished_v, starts_v);
            ends_v = _mm256_castps_si256(_mm256_blendv_ps(_mm256_castsi256_ps(ends_v), _mm256_castsi256_ps(_mm256_set1_epi32(DUMMY_SIZE as i32)), _mm256_castsi256_ps(finished_v)));

            _mm256_maskstore_epi32(starts.as_mut_ptr() as *mut i32, finished_v, starts_v);
        }

        let raw_city0 = _mm256_loadu_si256(base.add(starts[0] as usize) as *const __m256i);
        let raw_city1 = _mm256_loadu_si256(base.add(starts[1] as usize) as *const __m256i);
        let raw_city2 = _mm256_loadu_si256(base.add(starts[2] as usize) as *const __m256i);
        let raw_city3 = _mm256_loadu_si256(base.add(starts[3] as usize) as *const __m256i);
        let raw_city4 = _mm256_loadu_si256(base.add(starts[4] as usize) as *const __m256i);
        let raw_city5 = _mm256_loadu_si256(base.add(starts[5] as usize) as *const __m256i);
        let raw_city6 = _mm256_loadu_si256(base.add(starts[6] as usize) as *const __m256i);
        let raw_city7 = _mm256_loadu_si256(base.add(starts[7] as usize) as *const __m256i);

        let semicolons = _mm256_set1_epi8(';' as i8);
        let mut sc0 = _mm_tzcnt_32(_mm256_movemask_epi8(_mm256_cmpeq_epi8(raw_city0, semicolons)) as u32) as i32;
        let mut sc1 = _mm_tzcnt_32(_mm256_movemask_epi8(_mm256_cmpeq_epi8(raw_city1, semicolons)) as u32) as i32;
        let mut sc2 = _mm_tzcnt_32(_mm256_movemask_epi8(_mm256_cmpeq_epi8(raw_city2, semicolons)) as u32) as i32;
        let mut sc3 = _mm_tzcnt_32(_mm256_movemask_epi8(_mm256_cmpeq_epi8(raw_city3, semicolons)) as u32) as i32;
        let mut sc4 = _mm_tzcnt_32(_mm256_movemask_epi8(_mm256_cmpeq_epi8(raw_city4, semicolons)) as u32) as i32;
        let mut sc5 = _mm_tzcnt_32(_mm256_movemask_epi8(_mm256_cmpeq_epi8(raw_city5, semicolons)) as u32) as i32;
        let mut sc6 = _mm_tzcnt_32(_mm256_movemask_epi8(_mm256_cmpeq_epi8(raw_city6, semicolons)) as u32) as i32;
        let mut sc7 = _mm_tzcnt_32(_mm256_movemask_epi8(_mm256_cmpeq_epi8(raw_city7, semicolons)) as u32) as i32;

        sc0 = if sc0 > 32 { 32 } else { sc0 };
        sc1 = if sc1 > 32 { 32 } else { sc1 };
        sc2 = if sc2 > 32 { 32 } else { sc2 };
        sc3 = if sc3 > 32 { 32 } else { sc3 };
        sc4 = if sc4 > 32 { 32 } else { sc4 };
        sc5 = if sc5 > 32 { 32 } else { sc5 };
        sc6 = if sc6 > 32 { 32 } else { sc6 };
        sc7 = if sc7 > 32 { 32 } else { sc7 };

        let masked_city0 = _mm256_and_si256(raw_city0, _mm256_loadu_si256(CITY_MASK.0.as_ptr().add(32 - sc0 as usize) as *const __m256i));
        let masked_city1 = _mm256_and_si256(raw_city1, _mm256_loadu_si256(CITY_MASK.0.as_ptr().add(32 - sc1 as usize) as *const __m256i));
        let masked_city2 = _mm256_and_si256(raw_city2, _mm256_loadu_si256(CITY_MASK.0.as_ptr().add(32 - sc2 as usize) as *const __m256i));
        let masked_city3 = _mm256_and_si256(raw_city3, _mm256_loadu_si256(CITY_MASK.0.as_ptr().add(32 - sc3 as usize) as *const __m256i));
        let masked_city4 = _mm256_and_si256(raw_city4, _mm256_loadu_si256(CITY_MASK.0.as_ptr().add(32 - sc4 as usize) as *const __m256i));
        let masked_city5 = _mm256_and_si256(raw_city5, _mm256_loadu_si256(CITY_MASK.0.as_ptr().add(32 - sc5 as usize) as *const __m256i));
        let masked_city6 = _mm256_and_si256(raw_city6, _mm256_loadu_si256(CITY_MASK.0.as_ptr().add(32 - sc6 as usize) as *const __m256i));
        let masked_city7 = _mm256_and_si256(raw_city7, _mm256_loadu_si256(CITY_MASK.0.as_ptr().add(32 - sc7 as usize) as *const __m256i));

        let mut masked_city0 = masked_city0;
        let mut masked_city1 = masked_city1;
        let mut masked_city2 = masked_city2;
        let mut masked_city3 = masked_city3;
        let mut masked_city4 = masked_city4;
        let mut masked_city5 = masked_city5;
        let mut masked_city6 = masked_city6;
        let mut masked_city7 = masked_city7;

        let mut semicolons_v = _mm256_set_epi32(sc7, sc6, sc5, sc4, sc3, sc2, sc1, sc0);
        let long_cities = _mm256_cmpeq_epi32(semicolons_v, _mm256_set1_epi32(32));

        if _mm256_testz_si256(long_cities, long_cities) == 0 {
            if sc0 == 32 { masked_city0 = process_long(base.add(starts[0] as usize), &mut hash, &mut sc0); semicolons_v = _mm256_insert_epi32(semicolons_v, sc0, 0); }
            if sc1 == 32 { masked_city1 = process_long(base.add(starts[1] as usize), &mut hash, &mut sc1); semicolons_v = _mm256_insert_epi32(semicolons_v, sc1, 1); }
            if sc2 == 32 { masked_city2 = process_long(base.add(starts[2] as usize), &mut hash, &mut sc2); semicolons_v = _mm256_insert_epi32(semicolons_v, sc2, 2); }
            if sc3 == 32 { masked_city3 = process_long(base.add(starts[3] as usize), &mut hash, &mut sc3); semicolons_v = _mm256_insert_epi32(semicolons_v, sc3, 3); }
            if sc4 == 32 { masked_city4 = process_long(base.add(starts[4] as usize), &mut hash, &mut sc4); semicolons_v = _mm256_insert_epi32(semicolons_v, sc4, 4); }
            if sc5 == 32 { masked_city5 = process_long(base.add(starts[5] as usize), &mut hash, &mut sc5); semicolons_v = _mm256_insert_epi32(semicolons_v, sc5, 5); }
            if sc6 == 32 { masked_city6 = process_long(base.add(starts[6] as usize), &mut hash, &mut sc6); semicolons_v = _mm256_insert_epi32(semicolons_v, sc6, 6); }
            if sc7 == 32 { masked_city7 = process_long(base.add(starts[7] as usize), &mut hash, &mut sc7); semicolons_v = _mm256_insert_epi32(semicolons_v, sc7, 7); }
        }

        let city_hashes = hash_cities(masked_city0, masked_city1, masked_city2, masked_city3, masked_city4, masked_city5, masked_city6, masked_city7);

        starts_v = _mm256_add_epi32(starts_v, semicolons_v);

        nums[0] = *(base.add((starts[0] + sc0 as u32) as usize - 2) as *const u64);
        nums[1] = *(base.add((starts[1] + sc1 as u32) as usize - 2) as *const u64);
        nums[2] = *(base.add((starts[4] + sc4 as u32) as usize - 2) as *const u64);
        nums[3] = *(base.add((starts[5] + sc5 as u32) as usize - 2) as *const u64);
        nums[4] = *(base.add((starts[2] + sc2 as u32) as usize - 2) as *const u64);
        nums[5] = *(base.add((starts[3] + sc3 as u32) as usize - 2) as *const u64);
        nums[6] = *(base.add((starts[6] + sc6 as u32) as usize - 2) as *const u64);
        nums[7] = *(base.add((starts[7] + sc7 as u32) as usize - 2) as *const u64);

        let nums_low = _mm256_load_si256(nums.as_ptr() as *const __m256i);
        let nums_high = _mm256_load_si256(nums.as_ptr().add(4) as *const __m256i);

        let low_words = _mm256_castps_si256(_mm256_shuffle_ps(_mm256_castsi256_ps(nums_low), _mm256_castsi256_ps(nums_high), 0x88));
        let high_words = _mm256_castps_si256(_mm256_shuffle_ps(_mm256_castsi256_ps(nums_low), _mm256_castsi256_ps(nums_high), 0xDD));

        let minus_mask = _mm256_cmpeq_epi8(low_words, _mm256_set1_epi16(';' as i16 | (('-' as i16) << 8)));

        let nums_low_left1 = _mm256_slli_epi64(nums_low, 8);
        let nums_high_left1 = _mm256_slli_epi64(nums_high, 8);
        let high_words_left1 = _mm256_castps_si256(_mm256_shuffle_ps(_mm256_castsi256_ps(nums_low_left1), _mm256_castsi256_ps(nums_high_left1), 0xDD));

        let mut nums_blended = _mm256_castps_si256(_mm256_blendv_ps(_mm256_castsi256_ps(high_words_left1), _mm256_castsi256_ps(high_words), _mm256_castsi256_ps(minus_mask)));

        starts_v = _mm256_add_epi32(starts_v, _mm256_set1_epi32(6));
        let minus_mask_shift = _mm256_srli_epi32(minus_mask, 31);
        let newline_mask = _mm256_cmpeq_epi8(nums_blended, _mm256_set1_epi8('\n' as i8));
        let newline_mask_shift = _mm256_srli_epi32(newline_mask, 31);
        let newline_shift = _mm256_slli_epi32(newline_mask_shift, 3);
        nums_blended = _mm256_sllv_epi32(nums_blended, newline_shift);

        let numbers = _mm256_subs_epu8(nums_blended, _mm256_set1_epi8('0' as i8));
        let mut mulled = _mm256_madd_epi16(numbers, _mm256_set1_epi32(0x0001640a));

        starts_v = _mm256_add_epi32(starts_v, minus_mask_shift);
        starts_v = _mm256_sub_epi32(starts_v, newline_mask_shift);
        _mm256_store_si256(starts.as_mut_ptr() as *mut __m256i, starts_v);

        at_end_mask = _mm256_cmpeq_epi32(starts_v, ends_v);
        check_finished = _mm256_testz_si256(at_end_mask, at_end_mask) == 0;

        mulled = _mm256_slli_epi32(mulled, 14);
        mulled = _mm256_srli_epi32(mulled, 22);
        let final_num = _mm256_sign_epi32(mulled, minus_mask);

        let h0 = insert_city(&mut hash, _mm256_extract_epi32(city_hashes, 0) as i64, masked_city0);
        let v0 = _mm_load_si128(hash.p.hashed_storage.add(4 * h0 as usize + 16 * 0) as *const __m128i);
        let h4 = insert_city(&mut hash, _mm256_extract_epi32(city_hashes, 2) as i64, masked_city4);
        let v4 = _mm_load_si128(hash.p.hashed_storage.add(4 * h4 as usize + 16 * 4) as *const __m128i);
        let h1 = insert_city(&mut hash, _mm256_extract_epi32(city_hashes, 4) as i64, masked_city1);
        let v1 = _mm_load_si128(hash.p.hashed_storage.add(4 * h1 as usize + 16 * 1) as *const __m128i);
        let h5 = insert_city(&mut hash, _mm256_extract_epi32(city_hashes, 6) as i64, masked_city5);
        let v5 = _mm_load_si128(hash.p.hashed_storage.add(4 * h5 as usize + 16 * 5) as *const __m128i);
        let h2 = insert_city(&mut hash, _mm256_extract_epi32(city_hashes, 1) as i64, masked_city2);
        let v2 = _mm_load_si128(hash.p.hashed_storage.add(4 * h2 as usize + 16 * 2) as *const __m128i);
        let h6 = insert_city(&mut hash, _mm256_extract_epi32(city_hashes, 3) as i64, masked_city6);
        let v6 = _mm_load_si128(hash.p.hashed_storage.add(4 * h6 as usize + 16 * 6) as *const __m128i);
        let h3 = insert_city(&mut hash, _mm256_extract_epi32(city_hashes, 5) as i64, masked_city3);
        let v3 = _mm_load_si128(hash.p.hashed_storage.add(4 * h3 as usize + 16 * 3) as *const __m128i);
        let h7 = insert_city(&mut hash, _mm256_extract_epi32(city_hashes, 7) as i64, masked_city7);
        let v7 = _mm_load_si128(hash.p.hashed_storage.add(4 * h7 as usize + 16 * 7) as *const __m128i);

        let ae = _mm256_set_m128i(v4, v0);
        let bf = _mm256_set_m128i(v5, v1);
        let cg = _mm256_set_m128i(v6, v2);
        let dh = _mm256_set_m128i(v7, v3);

        let abef_low = _mm256_unpacklo_epi64(ae, bf);
        let cdgh_low = _mm256_unpacklo_epi64(cg, dh);
        let abef_high = _mm256_unpackhi_epi32(ae, bf);
        let cdgh_high = _mm256_unpackhi_epi32(cg, dh);

        let mins = _mm256_unpacklo_epi64(abef_high, cdgh_high);
        let maxs = _mm256_unpackhi_epi64(abef_high, cdgh_high);

        let abef_shift = _mm256_set_epi64x(0x0707070707060504, 0x0303030303020100, 0x0707070707060504, 0x0303030303020100);
        let final_abef = _mm256_shuffle_epi8(final_num, abef_shift);
        let cdgh_shift = _mm256_set_epi64x(0x0F0F0F0F0F0E0D0C, 0x0B0B0B0B0B0A0908, 0x0F0F0F0F0F0E0D0C, 0x0B0B0B0B0B0A0908);
        let final_cdgh = _mm256_shuffle_epi8(final_num, cdgh_shift);

        let inc = _mm256_set1_epi64x(1i64 << COUNT_BITS_START);
        let n_abef_low = _mm256_add_epi64(_mm256_add_epi64(abef_low, final_abef), inc);
        let n_cdgh_low = _mm256_add_epi64(_mm256_add_epi64(cdgh_low, final_cdgh), inc);
        let n_mins = _mm256_min_epi32(mins, final_num);
        let n_maxs = _mm256_max_epi32(maxs, final_num);

        let n_abef_high = _mm256_unpacklo_epi32(n_mins, n_maxs);
        let n_cdgh_high = _mm256_unpackhi_epi32(n_mins, n_maxs);
        let n_ae = _mm256_unpacklo_epi64(n_abef_low, n_abef_high);
        let n_bf = _mm256_unpackhi_epi64(n_abef_low, n_abef_high);
        let n_cg = _mm256_unpacklo_epi64(n_cdgh_low, n_cdgh_high);
        let n_dh = _mm256_unpackhi_epi64(n_cdgh_low, n_cdgh_high);

        _mm_store_si128(hash.p.hashed_storage.add(4 * h0 as usize + 16 * 0) as *mut __m128i, _mm256_extracti128_si256(n_ae, 0));
        _mm_store_si128(hash.p.hashed_storage.add(4 * h1 as usize + 16 * 1) as *mut __m128i, _mm256_extracti128_si256(n_bf, 0));
        _mm_store_si128(hash.p.hashed_storage.add(4 * h2 as usize + 16 * 2) as *mut __m128i, _mm256_extracti128_si256(n_cg, 0));
        _mm_store_si128(hash.p.hashed_storage.add(4 * h3 as usize + 16 * 3) as *mut __m128i, _mm256_extracti128_si256(n_dh, 0));
        _mm_store_si128(hash.p.hashed_storage.add(4 * h4 as usize + 16 * 4) as *mut __m128i, _mm256_extracti128_si256(n_ae, 1));
        _mm_store_si128(hash.p.hashed_storage.add(4 * h5 as usize + 16 * 5) as *mut __m128i, _mm256_extracti128_si256(n_bf, 1));
        _mm_store_si128(hash.p.hashed_storage.add(4 * h6 as usize + 16 * 6) as *mut __m128i, _mm256_extracti128_si256(n_cg, 1));
        _mm_store_si128(hash.p.hashed_storage.add(4 * h7 as usize + 16 * 7) as *mut __m128i, _mm256_extracti128_si256(n_dh, 1));
    }
}

unsafe fn hash_long(x: i64, y: i64) -> i32 {
    let seed = 0x9e3779b97f4a7c15u64 as i64;
    let val = (x.wrapping_mul(seed).rotate_left(5) ^ y).wrapping_mul(seed);
    (val as u32 & HASH_LONG_MASK) as i32
}

unsafe fn process_long(start: *const c_void, h: *mut Hash, sc_out: *mut i32) -> __m256i {
    let seg0 = _mm256_loadu_si256(start.add(0) as *const __m256i);
    let mut seg1 = _mm256_loadu_si256(start.add(32) as *const __m256i);
    let mut seg2 = _mm256_loadu_si256(start.add(64) as *const __m256i);
    let mut seg3 = _mm256_loadu_si256(start.add(96) as *const __m256i);

    let semicolons = _mm256_set1_epi8(';' as i8);
    let sc1 = _mm256_movemask_epi8(_mm256_cmpeq_epi8(seg1, semicolons)).trailing_zeros() as i32;
    let sc2 = _mm256_movemask_epi8(_mm256_cmpeq_epi8(seg2, semicolons)).trailing_zeros() as i32;
    let sc3 = _mm256_movemask_epi8(_mm256_cmpeq_epi8(seg3, semicolons)).trailing_zeros() as i32;

    let mut hash_val = hash_long(*(start as *const i64), *(start.add(8) as *const i64));

    if sc1 < 32 {
        *sc_out = 32 + sc1;
        seg1 = _mm256_and_si256(seg1, _mm256_loadu_si256(CITY_MASK.0.as_ptr().add(32 - sc1 as usize) as *const __m256i));
        hash_val = insert_city_long1(h, hash_val, seg0, seg1);
    } else if sc2 < 32 {
        *sc_out = 64 + sc2;
        seg2 = _mm256_and_si256(seg2, _mm256_loadu_si256(CITY_MASK.0.as_ptr().add(32 - sc2 as usize) as *const __m256i));
        hash_val = insert_city_long2(h, hash_val, seg0, seg1, seg2);
    } else {
        *sc_out = 96 + sc3;
        seg3 = _mm256_and_si256(seg3, _mm256_loadu_si256(CITY_MASK.0.as_ptr().add(32 - sc3 as usize) as *const __m256i));
        hash_val = insert_city_long3(h, hash_val, seg0, seg1, seg2, seg3);
    }
    city_from_long_hash(hash_val).reg
}

unsafe fn hash_cities(a: __m256i, b: __m256i, c: __m256i, d: __m256i, e: __m256i, f: __m256i, g: __m256i, h: __m256i) -> __m256i {
    let ab = _mm256_inserti128_si256(a, _mm256_castsi256_si128(b), 1);
    let mut cd = _mm256_inserti128_si256(c, _mm256_castsi256_si128(d), 1);
    let ef = _mm256_inserti128_si256(e, _mm256_castsi256_si128(f), 1);
    let mut gh = _mm256_inserti128_si256(g, _mm256_castsi256_si128(h), 1);

    cd = _mm256_slli_si256(cd, 8);
    gh = _mm256_slli_si256(gh, 8);

    let acbd = _mm256_blend_epi32(ab, cd, 0xCC);
    let egfh = _mm256_blend_epi32(ef, gh, 0xCC);

    let acbd2 = _mm256_srli_epi64(acbd, 28);
    let egfh2 = _mm256_srli_epi64(egfh, 28);

    let acbd_xor = _mm256_xor_si256(acbd, acbd2);
    let egfh_xor = _mm256_xor_si256(egfh, egfh2);

    let acegbdfh = _mm256_castps_si256(_mm256_shuffle_ps(_mm256_castsi256_ps(acbd_xor), _mm256_castsi256_ps(egfh_xor), 0x88));
    let hash = _mm256_madd_epi16(acegbdfh, acegbdfh);
    _mm256_and_si256(hash, _mm256_set1_epi32(HASH_SHORT_MASK as i32))
}

unsafe fn hash_city(str: __m256i) -> i32 {
    let zero = _mm256_set1_epi32(0);
    let hash = hash_cities(str, zero, zero, zero, zero, zero, zero, zero);
    _mm256_extract_epi32(hash, 0)
}

unsafe fn insert_city(h: *mut Hash, mut hash: i64, masked_city: __m256i) -> i64 {
    loop {
        let stored = _mm256_load_si256((*h).p.hashed_cities.add(hash as usize) as *const __m256i);
        let xor = _mm256_xor_si256(masked_city, stored);
        if _mm256_testz_si256(xor, xor) != 0 {
            return hash;
        }
        if _mm256_testz_si256(stored, stored) != 0 {
            _mm256_store_si256((*h).p.hashed_cities.add(hash as usize) as *mut __m256i, masked_city);
            *(*h).p.packed_offsets.add((*h).counts.num_cities as usize) = hash as i32;
            (*h).counts.num_cities += 1;

            let init_data = _mm256_set_epi32(MIN_TEMP, MAX_TEMP, (SUM_SIGN_BIT >> 32) as i32, 0, MIN_TEMP, MAX_TEMP, (SUM_SIGN_BIT >> 32) as i32, 0);
            _mm256_store_si256((*h).p.hashed_storage.add(4 * hash as usize + 0) as *mut __m256i, init_data);
            _mm256_store_si256((*h).p.hashed_storage.add(4 * hash as usize + 32) as *mut __m256i, init_data);
            _mm256_store_si256((*h).p.hashed_storage.add(4 * hash as usize + 64) as *mut __m256i, init_data);
            _mm256_store_si256((*h).p.hashed_storage.add(4 * hash as usize + 96) as *mut __m256i, init_data);
            return hash;
        }
        hash += SHORT_CITY_LENGTH as i64;
        hash &= HASH_SHORT_MASK as i64;
    }
}

unsafe fn insert_city_long1(hash: *mut Hash, mut hash_val: i32, seg0: __m256i, seg1: __m256i) -> i32 {
    loop {
        let stored0 = _mm256_loadu_si256((*hash).p.hashed_cities_long.add(hash_val as usize) as *const __m256i);
        let stored1 = _mm256_loadu_si256((*hash).p.hashed_cities_long.add(hash_val as usize + 32) as *const __m256i);
        let xor0 = _mm256_xor_si256(stored0, seg0);
        let xor1 = _mm256_xor_si256(stored1, seg1);

        if _mm256_testz_si256(xor0, xor0) != 0 && _mm256_testz_si256(xor1, xor1) != 0 { return hash_val; }
        if _mm256_testz_si256(stored0, stored0) != 0 {
            _mm256_store_si256((*hash).p.hashed_cities_long.add(hash_val as usize) as *mut __m256i, seg0);
            _mm256_store_si256((*hash).p.hashed_cities_long.add(hash_val as usize + 32) as *mut __m256i, seg1);
            (*hash).counts.num_cities_long += 1;
            return hash_val;
        }
        hash_val += LONG_CITY_LENGTH as i32;
        hash_val &= HASH_LONG_MASK as i32;
    }
}

unsafe fn insert_city_long2(hash: *mut Hash, mut hash_val: i32, seg0: __m256i, seg1: __m256i, seg2: __m256i) -> i32 {
    loop {
        let stored0 = _mm256_loadu_si256((*hash).p.hashed_cities_long.add(hash_val as usize) as *const __m256i);
        let stored1 = _mm256_loadu_si256((*hash).p.hashed_cities_long.add(hash_val as usize + 32) as *const __m256i);
        let stored2 = _mm256_loadu_si256((*hash).p.hashed_cities_long.add(hash_val as usize + 64) as *const __m256i);
        let xor0 = _mm256_xor_si256(stored0, seg0);
        let xor1 = _mm256_xor_si256(stored1, seg1);
        let xor2 = _mm256_xor_si256(stored2, seg2);

        if _mm256_testz_si256(xor0, xor0) != 0 && _mm256_testz_si256(xor1, xor1) != 0 && _mm256_testz_si256(xor2, xor2) != 0 { return hash_val; }
        if _mm256_testz_si256(stored0, stored0) != 0 {
            _mm256_store_si256((*hash).p.hashed_cities_long.add(hash_val as usize) as *mut __m256i, seg0);
            _mm256_store_si256((*hash).p.hashed_cities_long.add(hash_val as usize + 32) as *mut __m256i, seg1);
            _mm256_store_si256((*hash).p.hashed_cities_long.add(hash_val as usize + 64) as *mut __m256i, seg2);
            (*hash).counts.num_cities_long += 1;
            return hash_val;
        }
        hash_val += LONG_CITY_LENGTH as i32;
        hash_val &= HASH_LONG_MASK as i32;
    }
}

unsafe fn insert_city_long3(hash: *mut Hash, mut hash_val: i32, seg0: __m256i, seg1: __m256i, seg2: __m256i, seg3: __m256i) -> i32 {
    loop {
        let stored0 = _mm256_loadu_si256((*hash).p.hashed_cities_long.add(hash_val as usize) as *const __m256i);
        let stored1 = _mm256_loadu_si256((*hash).p.hashed_cities_long.add(hash_val as usize + 32) as *const __m256i);
        let stored2 = _mm256_loadu_si256((*hash).p.hashed_cities_long.add(hash_val as usize + 64) as *const __m256i);
        let stored3 = _mm256_loadu_si256((*hash).p.hashed_cities_long.add(hash_val as usize + 96) as *const __m256i);
        let xor0 = _mm256_xor_si256(stored0, seg0);
        let xor1 = _mm256_xor_si256(stored1, seg1);
        let xor2 = _mm256_xor_si256(stored2, seg2);
        let xor3 = _mm256_xor_si256(stored3, seg3);

        if _mm256_testz_si256(xor0, xor0) != 0 && _mm256_testz_si256(xor1, xor1) != 0 && _mm256_testz_si256(xor2, xor2) != 0 && _mm256_testz_si256(xor3, xor3) != 0 { return hash_val; }
        if _mm256_testz_si256(stored0, stored0) != 0 {
            _mm256_store_si256((*hash).p.hashed_cities_long.add(hash_val as usize) as *mut __m256i, seg0);
            _mm256_store_si256((*hash).p.hashed_cities_long.add(hash_val as usize + 32) as *mut __m256i, seg1);
            _mm256_store_si256((*hash).p.hashed_cities_long.add(hash_val as usize + 64) as *mut __m256i, seg2);
            _mm256_store_si256((*hash).p.hashed_cities_long.add(hash_val as usize + 96) as *mut __m256i, seg3);
            (*hash).counts.num_cities_long += 1;
            return hash_val;
        }
        hash_val += LONG_CITY_LENGTH as i32;
        hash_val &= HASH_LONG_MASK as i32;
    }
}

unsafe fn city_is_long(city: PackedCity) -> bool {
    city.long_ref.sentinel == LONG_CITY_SENTINEL
}

unsafe extern "C" fn sort_result(a: *const c_void, b: *const c_void, arg: *mut c_void) -> i32 {
    let r = arg as *mut Results;
    let left = a as *const ResultsRef;
    let right = b as *const ResultsRef;

    let left_row = &*(*r).rows.add((*left).offset as usize / SHORT_CITY_LENGTH);
    let right_row = &*(*r).rows.add((*right).offset as usize / SHORT_CITY_LENGTH);

    let left_bytes = if city_is_long(left_row.city) {
        (*(*r).long_cities.add(left_row.city.long_ref.index as usize)).bytes.as_ptr()
    } else {
        left_row.city.short_city.bytes.as_ptr()
    };

    let right_bytes = if city_is_long(right_row.city) {
        (*(*r).long_cities.add(right_row.city.long_ref.index as usize)).bytes.as_ptr()
    } else {
        right_row.city.short_city.bytes.as_ptr()
    };

    libc::strcmp(left_bytes as *const i8, right_bytes as *const i8)
}

unsafe fn merge(dst: *mut Results, src: *mut Results) {
    for i in 0..(*src).num_cities {
        let ref_val = *(*src).refs.add(i as usize);
        let row = *(*src).rows.add(ref_val.offset as usize / SHORT_CITY_LENGTH);
        let mut hash_val = hash_city(row.city.reg);

        let mut row_city = row.city;
        if city_is_long(row_city) {
            let long_city = &*(*src).long_cities.add(row_city.long_ref.index as usize);
            let mut dst_long_idx = 0;
            while dst_long_idx < (*dst).num_long_cities {
                let dst_long = &*(*dst).long_cities.add(dst_long_idx as usize);
                if long_city_equal(long_city, dst_long) { break; }
                dst_long_idx += 1;
            }
            if dst_long_idx == (*dst).num_long_cities {
                *(*dst).long_cities.add((*dst).num_long_cities as usize) = *long_city;
                (*dst).num_long_cities += 1;
            }
            row_city = city_from_long_hash(dst_long_idx);
            hash_val = hash_city(row_city.reg);
        }

        hash_val = ((hash_val as u32 >> (HASH_SHIFT - HASH_RESULT_SHIFT)) & HASH_RESULT_MASK) as i32;
        loop {
            let dst_row = &mut *(*dst).rows.add(hash_val as usize / SHORT_CITY_LENGTH);
            let xor = _mm256_xor_si256((*dst_row).city.reg, row_city.reg);
            if _mm256_testz_si256(xor, xor) != 0 {
                dst_row.sum += row.sum;
                dst_row.count += row.count;
                dst_row.min = if row.min < dst_row.min { row.min } else { dst_row.min };
                dst_row.max = if row.max > dst_row.max { row.max } else { dst_row.max };
                break;
            }

            if _mm256_testz_si256(dst_row.city.reg, dst_row.city.reg) != 0 {
                *(*dst).refs.add((*dst).num_cities as usize) = ResultsRef { offset: hash_val as u32 };
                *(*dst).rows.add(hash_val as usize / SHORT_CITY_LENGTH) = ResultsRow { city: row_city, ..row };
                (*dst).num_cities += 1;
                break;
            }
            hash_val = ((hash_val as u32 + SHORT_CITY_LENGTH as u32) & HASH_RESULT_MASK) as i32;
        }
    }
}

unsafe fn long_city_equal(a: *const LongCity, b: *const LongCity) -> bool {
    let xor0 = _mm256_xor_si256((*a).regs[0], (*b).regs[0]);
    let xor1 = _mm256_xor_si256((*a).regs[1], (*b).regs[1]);
    let xor2 = _mm256_xor_si256((*a).regs[2], (*b).regs[2]);
    let xor3 = _mm256_xor_si256((*a).regs[3], (*b).regs[3]);
    _mm256_testz_si256(xor0, xor0) != 0 && _mm256_testz_si256(xor1, xor1) != 0 && _mm256_testz_si256(xor2, xor2) != 0 && _mm256_testz_si256(xor3, xor3) != 0
}

unsafe fn city_from_long_hash(hash_val: i32) -> PackedCity {
    PackedCity { reg: _mm256_set_epi32(0, 0, 0, 0, 0, 0, hash_val, LONG_CITY_SENTINEL) }
}

unsafe fn convert_hash_to_results(hash: *mut Hash, out: *mut Results) {
    (*out).num_cities = (*hash).counts.num_cities;
    (*out).num_long_cities = 0;

    for i in 0..(*hash).counts.num_cities {
        let offset = *(*hash).p.packed_offsets.add(i as usize);
        let city_reg = _mm256_load_si256((*hash).p.hashed_cities.add(offset as usize) as *const __m256i);
        let mut city = PackedCity { reg: city_reg };
        let rows = (*hash).p.hashed_storage.add(offset as usize * (HASH_ENTRY_SIZE / SHORT_CITY_LENGTH)) as *mut HashRow;

        let mut sum = extract_sum((*rows).packed_sum_count);
        let mut count = extract_count((*rows).packed_sum_count);
        let mut min = (*rows).min;
        let mut max = (*rows).max;

        for j in 1..STRIDE {
            let row = &*rows.add(j);
            sum += extract_sum(row.packed_sum_count);
            count += extract_count(row.packed_sum_count);
            min = if row.min < min { row.min } else { min };
            max = if row.max > max { row.max } else { max };
        }

        if city_is_long(city) {
            let long_city = &*((*hash).p.hashed_cities_long.add(city.long_ref.index as usize) as *const LongCity);
            *(*out).long_cities.add((*out).num_long_cities as usize) = *long_city;
            (*city.long_ref).index = (*out).num_long_cities;
            (*out).num_long_cities += 1;
        }

        let mut res_offset = ((offset as u32 >> (HASH_SHIFT - HASH_RESULT_SHIFT)) & HASH_RESULT_MASK) as usize;
        loop {
            let out_row = &mut *(*out).rows.add(res_offset / SHORT_CITY_LENGTH);
            if _mm256_testz_si256(out_row.city.reg, out_row.city.reg) != 0 {
                *out_row = ResultsRow { city, sum, count, min: min as i16, max: max as i16 };
                break;
            }
            res_offset = (res_offset + SHORT_CITY_LENGTH) & HASH_RESULT_MASK as usize;
        }
        *(*out).refs.add(i as usize) = ResultsRef { offset: res_offset as u32 };
    }
}

unsafe fn print_results(results: *mut Results) {
    let mut buffer = vec![0u8; MAX_CITIES * 150];
    let mut pos = 0;
    buffer[pos] = '{' as u8; pos += 1;

    for i in 0..(*results).num_cities {
        let ref_val = *(*results).refs.add(i as usize);
        let row = &*(*results).rows.add(ref_val.offset as usize / SHORT_CITY_LENGTH);

        let bytes = if city_is_long(row.city) {
            (*(*results).long_cities.add(row.city.long_ref.index as usize)).bytes.as_ptr()
        } else {
            row.city.short_city.bytes.as_ptr()
        };

        let s = format!("{}={:.1}/{:.1}/{:.1}",
            std::ffi::CStr::from_ptr(bytes as *const i8).to_string_lossy(),
            row.min as f32 * 0.1,
            (row.sum as f32 * 0.1 / row.count as f32 * 10.0).round() * 0.1,
            row.max as f32 * 0.1
        );
        let s_bytes = s.as_bytes();
        buffer[pos..pos+s_bytes.len()].copy_from_slice(s_bytes);
        pos += s_bytes.len();

        if i != (*results).num_cities - 1 {
            buffer[pos] = ',' as u8; pos += 1;
            buffer[pos] = ' ' as u8; pos += 1;
        }
    }
    buffer[pos] = '}' as u8; pos += 1;
    buffer[pos] = '\n' as u8; pos += 1;
    io::stdout().write_all(&buffer[..pos]).unwrap();
}

unsafe extern "C" {
    fn perror(s: *const i8);
}
