use std::hash::Hasher;
use test;

use twox_hash::XxHash32;
use zero_xxhash::hash32::xxhash32;

const degree: usize = 2;
const size_0: usize = 0 * degree;
const size_1: usize = 1 * degree;
const size_4: usize = 4 * degree;
const size_16: usize = 16 * degree;
const size_32: usize = 32 * degree;
const size_128: usize = 128 * degree;
const size_256: usize = 256 * degree;
const size_512: usize = 512 * degree;
const size_1024: usize = 1024 * degree;
const size_mega: usize = 1024 * 1024 * degree;

fn hasher_bench<H>(b: &mut test::Bencher, mut hasher: H, len: usize)
where
    H: Hasher,
{
    let bytes: Vec<_> = (0..100).cycle().take(len).collect();
    b.bytes = bytes.len() as u64;
    b.iter(|| {
        hasher.write(&bytes);
        hasher.finish()
    });
}

fn xxhash32_bench(b: &mut test::Bencher, len: usize) {
    hasher_bench(b, XxHash32::with_seed(0), len)
}

fn zerohash32_bench(b: &mut test::Bencher, len: usize) {
    let bytes: Vec<_> = (0..100).cycle().take(len).collect();
    b.bytes = bytes.len() as u64;
    b.iter(|| xxhash32(&bytes, 0));
}

#[bench]
fn xxhash32_megabyte(b: &mut test::Bencher) {
    xxhash32_bench(b, size_mega)
}

#[bench]
fn xxhash32_1024_byte(b: &mut test::Bencher) {
    xxhash32_bench(b, size_1024)
}

#[bench]
fn xxhash32_512_byte(b: &mut test::Bencher) {
    xxhash32_bench(b, size_512)
}

#[bench]
fn xxhash32_256_byte(b: &mut test::Bencher) {
    xxhash32_bench(b, size_256)
}

#[bench]
fn xxhash32_128_byte(b: &mut test::Bencher) {
    xxhash32_bench(b, size_128)
}

#[bench]
fn xxhash32_32_byte(b: &mut test::Bencher) {
    xxhash32_bench(b, size_32)
}

#[bench]
fn xxhash32_16_byte(b: &mut test::Bencher) {
    xxhash32_bench(b, size_16)
}

#[bench]
fn xxhash32_4_byte(b: &mut test::Bencher) {
    xxhash32_bench(b, size_4)
}

#[bench]
fn xxhash32_1_byte(b: &mut test::Bencher) {
    xxhash32_bench(b, size_1)
}

#[bench]
fn xxhash32_0_byte(b: &mut test::Bencher) {
    xxhash32_bench(b, size_0)
}

#[bench]
fn zerohash32_megabyte(b: &mut test::Bencher) {
    zerohash32_bench(b, size_mega)
}

#[bench]
fn zerohash32_1024_byte(b: &mut test::Bencher) {
    zerohash32_bench(b, size_1024)
}

#[bench]
fn zerohash32_512_byte(b: &mut test::Bencher) {
    zerohash32_bench(b, size_512)
}

#[bench]
fn zerohash32_256_byte(b: &mut test::Bencher) {
    zerohash32_bench(b, size_256)
}

#[bench]
fn zerohash32_128_byte(b: &mut test::Bencher) {
    zerohash32_bench(b, size_128)
}

#[bench]
fn zerohash32_32_byte(b: &mut test::Bencher) {
    zerohash32_bench(b, size_32)
}

#[bench]
fn zerohash32_16_byte(b: &mut test::Bencher) {
    zerohash32_bench(b, size_16)
}

#[bench]
fn zerohash32_4_byte(b: &mut test::Bencher) {
    zerohash32_bench(b, size_4)
}

#[bench]
fn zerohash32_1_byte(b: &mut test::Bencher) {
    zerohash32_bench(b, size_1)
}

#[bench]
fn zerohash32_0_byte(b: &mut test::Bencher) {
    zerohash32_bench(b, size_0)
}
