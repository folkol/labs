use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn get_long(data: &[u8], pos: usize) -> u64 {
    let b: [u8; 8] = data[pos..pos + 8].try_into().expect("expected byte array");
    u64::from_le_bytes(b)
}

fn get_long_unaligned(bytes: &[u8], pos: usize) -> u64 {
    unsafe {
        let p = bytes.as_ptr().add(pos) as *const u64;
        u64::from_le(p.read_unaligned())
    }
}

fn get_long_aligned(bytes: &[u8], pos: usize) -> u64 {
    unsafe {
        let p = bytes.as_ptr().add(pos) as *const u64;
        u64::from_le(*p)
    }
}

fn bench_fn(c: &mut Criterion) {
    let data = b"The quick brown fox jumps over the lazy dogThe quick brown fox jumps over the lazy dogThe quick brown fox jumps over the lazy dogThe quick brown fox jumps over the lazy dogThe quick brown fox jumps over the lazy dogThe quick brown fox jumps over the lazy dogThe quick brown fox jumps over the lazy dog\nThe quick brown fox jumps over the lazy dog";

    assert_eq!(get_long(data, 0), get_long_unaligned(data, 0));
    assert_eq!(get_long(data, 0), get_long_aligned(data, 0));

    let mut g = c.benchmark_group("get_long");
    g.bench_function("safe", |b| b.iter(|| get_long(black_box(data), 0)));
    g.bench_function("unaligned", |b| {
        b.iter(|| get_long_unaligned(black_box(data), 0))
    });
    g.bench_function("aligned", |b| {
        b.iter(|| get_long_aligned(black_box(data), 0))
    });
    g.finish();
}

criterion_group!(benches, bench_fn);
criterion_main!(benches);
