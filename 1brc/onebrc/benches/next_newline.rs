use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

#[inline]
pub fn next_newline(bytes: &[u8], mut prev: usize) -> usize {
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
pub fn next_newline_memchr(bytes: &[u8], prev: usize) -> usize {
    memchr::memchr(b'\n', &bytes[prev..]).expect("expected newline")
}

#[inline]
pub fn next_newline_ptr(mut p: *const u8) -> *const u8 {
    loop {
        let word = unsafe { (p as *const u64).read_unaligned() }; // native endian load
        let word = u64::from_le(word); // match little-endian files / x86 behavior

        let input = word ^ 0x0A0A0A0A0A0A0A0Au64;
        let pos = (input.wrapping_sub(0x0101010101010101u64)) & !input & 0x8080808080808080u64;

        if pos != 0 {
            p = unsafe { p.add((pos.trailing_zeros() as usize) >> 3) };
            return p;
        } else {
            p = unsafe { p.add(8) };
        }
    }
}

fn bench_fn(c: &mut Criterion) {
    let data = b"The quick brown fox jumps over the lazy dog\n                   ";

    let p = data.as_ptr();
    assert_eq!(next_newline(data, 0), next_newline_memchr(data, 0));
    assert_eq!(next_newline(data, 0), unsafe {
        next_newline_ptr(p).offset_from(p) as usize
    });

    let mut g = c.benchmark_group("next_newline");
    g.bench_function("safe", |b| b.iter(|| next_newline(black_box(data), 0)));
    g.bench_function("memchr", |b| {
        b.iter(|| next_newline_memchr(black_box(data), 0))
    });
    g.bench_function("unsafe", |b| {
        b.iter(|| next_newline_ptr(black_box(data.as_ptr())))
    });
    g.finish();
}

criterion_group!(benches, bench_fn);
criterion_main!(benches);
