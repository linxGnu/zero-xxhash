use std::hash::{Hasher, SipHasher};

use fnv;
use test;
use twox_hash::XxHash;
use zero_xxhash::hash64::xxhash64;

const size_0: usize = 0;
const size_1: usize = 1;
const size_4: usize = 4;
const size_16: usize = 16;
const size_64: usize = 64;
const size_128: usize = 128;
const size_256: usize = 256;
const size_512: usize = 512;
const size_1024: usize = 1024;
const size_mega: usize = 1024 * 1024 * 16; // 16MB

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

fn xxhash64_bench(b: &mut test::Bencher, len: usize) {
    hasher_bench(b, XxHash::with_seed(0), len)
}

fn zerohash64_bench(b: &mut test::Bencher, len: usize) {
    let bytes: Vec<_> = (0..100).cycle().take(len).collect();
    b.bytes = bytes.len() as u64;
    b.iter(|| xxhash64(&bytes, 0));
}

fn siphash_bench(b: &mut test::Bencher, len: usize) {
    hasher_bench(b, SipHasher::new(), len)
}

fn fnvhash_bench(b: &mut test::Bencher, len: usize) {
    hasher_bench(b, <fnv::FnvHasher as Default>::default(), len)
}

#[bench]
fn siphash_megabyte(b: &mut test::Bencher) {
    siphash_bench(b, size_mega)
}

#[bench]
fn siphash_1024_byte(b: &mut test::Bencher) {
    siphash_bench(b, size_1024)
}

#[bench]
fn siphash_512_byte(b: &mut test::Bencher) {
    siphash_bench(b, size_512)
}

#[bench]
fn siphash_256_byte(b: &mut test::Bencher) {
    siphash_bench(b, size_256)
}

#[bench]
fn siphash_128_byte(b: &mut test::Bencher) {
    siphash_bench(b, size_128)
}

#[bench]
fn siphash_64_byte(b: &mut test::Bencher) {
    siphash_bench(b, size_64)
}

#[bench]
fn siphash_16_byte(b: &mut test::Bencher) {
    siphash_bench(b, size_16)
}

#[bench]
fn siphash_4_byte(b: &mut test::Bencher) {
    siphash_bench(b, size_4)
}

#[bench]
fn siphash_1_byte(b: &mut test::Bencher) {
    siphash_bench(b, size_1)
}

#[bench]
fn siphash_0_byte(b: &mut test::Bencher) {
    siphash_bench(b, size_0)
}

#[bench]
fn fnvhash_megabyte(b: &mut test::Bencher) {
    fnvhash_bench(b, size_mega)
}

#[bench]
fn fnvhash_1024_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, size_1024)
}

#[bench]
fn fnvhash_512_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, size_512)
}

#[bench]
fn fnvhash_256_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, size_256)
}

#[bench]
fn fnvhash_128_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, size_128)
}

#[bench]
fn fnvhash_64_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, size_64)
}

#[bench]
fn fnvhash_16_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, size_16)
}

#[bench]
fn fnvhash_4_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, size_4)
}

#[bench]
fn fnvhash_1_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, size_1)
}

#[bench]
fn fnvhash_0_byte(b: &mut test::Bencher) {
    fnvhash_bench(b, size_0)
}

#[bench]
fn xxhash64_megabyte(b: &mut test::Bencher) {
    xxhash64_bench(b, size_mega)
}

#[bench]
fn xxhash64_1024_byte(b: &mut test::Bencher) {
    xxhash64_bench(b, size_1024)
}

#[bench]
fn xxhash64_512_byte(b: &mut test::Bencher) {
    xxhash64_bench(b, size_512)
}

#[bench]
fn xxhash64_256_byte(b: &mut test::Bencher) {
    xxhash64_bench(b, size_256)
}

#[bench]
fn xxhash64_128_byte(b: &mut test::Bencher) {
    xxhash64_bench(b, size_128)
}

#[bench]
fn xxhash64_64_byte(b: &mut test::Bencher) {
    xxhash64_bench(b, size_64)
}

#[bench]
fn xxhash64_16_byte(b: &mut test::Bencher) {
    xxhash64_bench(b, size_16)
}

#[bench]
fn xxhash64_4_byte(b: &mut test::Bencher) {
    xxhash64_bench(b, size_4)
}

#[bench]
fn xxhash64_1_byte(b: &mut test::Bencher) {
    xxhash64_bench(b, size_1)
}

#[bench]
fn xxhash64_0_byte(b: &mut test::Bencher) {
    xxhash64_bench(b, size_0)
}

#[bench]
fn zerohash64_megabyte(b: &mut test::Bencher) {
    zerohash64_bench(b, size_mega)
}

#[bench]
fn zerohash64_1024_byte(b: &mut test::Bencher) {
    zerohash64_bench(b, size_1024)
}

#[bench]
fn zerohash64_512_byte(b: &mut test::Bencher) {
    zerohash64_bench(b, size_512)
}

#[bench]
fn zerohash64_256_byte(b: &mut test::Bencher) {
    zerohash64_bench(b, size_256)
}

#[bench]
fn zerohash64_128_byte(b: &mut test::Bencher) {
    zerohash64_bench(b, size_128)
}

#[bench]
fn zerohash64_64_byte(b: &mut test::Bencher) {
    zerohash64_bench(b, size_64)
}

#[bench]
fn zerohash64_16_byte(b: &mut test::Bencher) {
    zerohash64_bench(b, size_16)
}

#[bench]
fn zerohash64_4_byte(b: &mut test::Bencher) {
    zerohash64_bench(b, size_4)
}

#[bench]
fn zerohash64_1_byte(b: &mut test::Bencher) {
    zerohash64_bench(b, size_1)
}

#[bench]
fn zerohash64_0_byte(b: &mut test::Bencher) {
    zerohash64_bench(b, size_0)
}
