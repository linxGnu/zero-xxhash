#[macro_use]
extern crate criterion;
extern crate twox_hash;
extern crate zero_xxhash;

use std::hash::Hasher;

use criterion::Criterion;
use twox_hash::{XxHash, XxHash32};
use zero_xxhash::hash32::xxhash32;
use zero_xxhash::hash64::xxhash64;

const degree: usize = 16;
const size_0: usize = 0 * degree;
const size_1: usize = 1 * degree;
const size_4: usize = 4 * degree;
const size_16: usize = 16 * degree;
const size_32: usize = 32 * degree;
const size_64: usize = 64 * degree;
const size_128: usize = 128 * degree;
const size_256: usize = 256 * degree;
const size_512: usize = 512 * degree;
const size_1024: usize = 1024 * degree;
const size_mega: usize = 1024 * 1024 * degree;

fn hasher_bench<H>(mut hasher: H, bytes: &[u8]) -> u64
where
    H: Hasher,
{
    hasher.write(&bytes);
    hasher.finish()
}

fn bench_zeroxxhash64_with_length(c: &mut Criterion, len: usize) {
    let title = format!("zero-xxhash64-{}", len);
    let bytes: Vec<u8> = (0..100).cycle().take(len).collect();

    c.bench_function(&title, move |b| b.iter(|| xxhash64(&bytes, 0)));
}

fn bench_xxhash64_with_length(c: &mut Criterion, len: usize) {
    let title = format!("xxhash64-{}", len);
    let bytes: Vec<u8> = (0..100).cycle().take(len).collect();
    let hasher = XxHash::with_seed(0);

    c.bench_function(&title, move |b| b.iter(|| hasher_bench(hasher, &bytes)));
}

fn bench_lib(c: &mut Criterion) {
    bench_xxhash64_with_length(c, size_mega);
    bench_zeroxxhash64_with_length(c, size_mega);

    bench_xxhash64_with_length(c, size_1024);
    bench_zeroxxhash64_with_length(c, size_1024);

    bench_xxhash64_with_length(c, size_512);
    bench_zeroxxhash64_with_length(c, size_512);

    bench_xxhash64_with_length(c, size_256);
    bench_zeroxxhash64_with_length(c, size_256);

    bench_xxhash64_with_length(c, size_128);
    bench_zeroxxhash64_with_length(c, size_128);

    bench_xxhash64_with_length(c, size_64);
    bench_zeroxxhash64_with_length(c, size_64);

    bench_xxhash64_with_length(c, size_32);
    bench_zeroxxhash64_with_length(c, size_32);

    bench_xxhash64_with_length(c, size_16);
    bench_zeroxxhash64_with_length(c, size_16);

    bench_xxhash64_with_length(c, size_4);
    bench_zeroxxhash64_with_length(c, size_4);

    bench_xxhash64_with_length(c, size_1);
    bench_zeroxxhash64_with_length(c, size_1);
}

criterion_group!(benches, bench_lib);
criterion_main!(benches);
