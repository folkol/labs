use memmap2::Mmap;
use std::fs::File;

fn main() {
    let file = File::open("../java-orig/measurements.txt").expect("failed to open file");
    let data = unsafe { Mmap::map(&file).expect("failed to map file") };
    println!("{}", count_lines(&data));
}

fn count_lines(data: &[u8]) -> i64 {
    memchr::memchr_iter(b'\n', data).count() as i64
}
