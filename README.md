# zero-xxhash
A Rust implementation of the [XXHash], zero allocation and inline hashing.

[![Build Status](https://travis-ci.org/linxGnu/zero-xxhash.svg)](https://travis-ci.org/linxGnu/zero-xxhash)

[XXHash]: https://github.com/Cyan4973/xxHash

## Examples

### xxhash64

```rust
use zero_xxhash::hash64::xxhash64;

let buf: Vec<_> = (0..100).cycle().take(len).collect();
let seed = 12345;

xxhash64(&buf, seed)
```

### xxhash32

```rust
use zero_xxhash::hash32::xxhash32;

let buf: Vec<_> = (0..100).cycle().take(len).collect();
let seed = 12345;

xxhash32(&buf, seed)
```

## Benchmarks

```
running 60 tests
test bench32::xxhash32_0_byte      ... bench:           7 ns/iter (+/- 1)
test bench32::xxhash32_1024_byte   ... bench:       2,147 ns/iter (+/- 370) = 7154 MB/s
test bench32::xxhash32_128_byte    ... bench:         289 ns/iter (+/- 52) = 6643 MB/s
test bench32::xxhash32_16_byte     ... bench:          37 ns/iter (+/- 5) = 6486 MB/s
test bench32::xxhash32_1_byte      ... bench:          23 ns/iter (+/- 2) = 652 MB/s
test bench32::xxhash32_256_byte    ... bench:         521 ns/iter (+/- 52) = 7370 MB/s
test bench32::xxhash32_32_byte     ... bench:          70 ns/iter (+/- 10) = 6857 MB/s
test bench32::xxhash32_4_byte      ... bench:          22 ns/iter (+/- 1) = 2727 MB/s
test bench32::xxhash32_512_byte    ... bench:       1,028 ns/iter (+/- 296) = 7470 MB/s
test bench32::xxhash32_megabyte    ... bench:   2,168,024 ns/iter (+/- 531,375) = 7254 MB/s
test bench32::zerohash32_0_byte    ... bench:           2 ns/iter (+/- 0)
test bench32::zerohash32_1024_byte ... bench:       2,039 ns/iter (+/- 194) = 7533 MB/s
test bench32::zerohash32_128_byte  ... bench:         261 ns/iter (+/- 8) = 7356 MB/s
test bench32::zerohash32_16_byte   ... bench:          37 ns/iter (+/- 0) = 6486 MB/s
test bench32::zerohash32_1_byte    ... bench:           7 ns/iter (+/- 1) = 2142 MB/s
test bench32::zerohash32_256_byte  ... bench:         521 ns/iter (+/- 62) = 7370 MB/s
test bench32::zerohash32_32_byte   ... bench:          71 ns/iter (+/- 9) = 6760 MB/s
test bench32::zerohash32_4_byte    ... bench:          11 ns/iter (+/- 0) = 5454 MB/s
test bench32::zerohash32_512_byte  ... bench:       1,030 ns/iter (+/- 59) = 7456 MB/s
test bench32::zerohash32_megabyte  ... bench:   2,145,970 ns/iter (+/- 164,592) = 7329 MB/s
test bench64::fnvhash_0_byte       ... bench:           0 ns/iter (+/- 0)
test bench64::fnvhash_1024_byte    ... bench:      33,502 ns/iter (+/- 2,556) = 947 MB/s
test bench64::fnvhash_128_byte     ... bench:       4,187 ns/iter (+/- 242) = 947 MB/s
test bench64::fnvhash_16_byte      ... bench:         523 ns/iter (+/- 43) = 948 MB/s
test bench64::fnvhash_1_byte       ... bench:          32 ns/iter (+/- 0) = 968 MB/s
test bench64::fnvhash_256_byte     ... bench:       8,375 ns/iter (+/- 383) = 947 MB/s
test bench64::fnvhash_4_byte       ... bench:         130 ns/iter (+/- 6) = 953 MB/s
test bench64::fnvhash_512_byte     ... bench:      16,750 ns/iter (+/- 910) = 947 MB/s
test bench64::fnvhash_64_byte      ... bench:       2,093 ns/iter (+/- 101) = 947 MB/s
test bench64::fnvhash_megabyte     ... bench:  35,113,176 ns/iter (+/- 1,149,847) = 925 MB/s
test bench64::siphash_0_byte       ... bench:           9 ns/iter (+/- 0)
test bench64::siphash_1024_byte    ... bench:      13,503 ns/iter (+/- 748) = 2350 MB/s
test bench64::siphash_128_byte     ... bench:       1,698 ns/iter (+/- 143) = 2336 MB/s
test bench64::siphash_16_byte      ... bench:         224 ns/iter (+/- 19) = 2214 MB/s
test bench64::siphash_1_byte       ... bench:          25 ns/iter (+/- 7) = 1240 MB/s
test bench64::siphash_256_byte     ... bench:       3,423 ns/iter (+/- 179) = 2318 MB/s
test bench64::siphash_4_byte       ... bench:          63 ns/iter (+/- 12) = 1968 MB/s
test bench64::siphash_512_byte     ... bench:       6,811 ns/iter (+/- 366) = 2330 MB/s
test bench64::siphash_64_byte      ... bench:         856 ns/iter (+/- 49) = 2317 MB/s
test bench64::siphash_megabyte     ... bench:  14,323,033 ns/iter (+/- 1,883,569) = 2269 MB/s
test bench64::xxhash64_0_byte      ... bench:           8 ns/iter (+/- 1)
test bench64::xxhash64_1024_byte   ... bench:       2,110 ns/iter (+/- 149) = 15044 MB/s
test bench64::xxhash64_128_byte    ... bench:         278 ns/iter (+/- 18) = 14273 MB/s
test bench64::xxhash64_16_byte     ... bench:          49 ns/iter (+/- 7) = 10122 MB/s
test bench64::xxhash64_1_byte      ... bench:          28 ns/iter (+/- 1) = 1107 MB/s
test bench64::xxhash64_256_byte    ... bench:         540 ns/iter (+/- 142) = 14696 MB/s
test bench64::xxhash64_4_byte      ... bench:          31 ns/iter (+/- 4) = 4000 MB/s
test bench64::xxhash64_512_byte    ... bench:       1,067 ns/iter (+/- 102) = 14875 MB/s
test bench64::xxhash64_64_byte     ... bench:         142 ns/iter (+/- 13) = 13971 MB/s
test bench64::xxhash64_megabyte    ... bench:   2,533,382 ns/iter (+/- 172,558) = 12831 MB/s
test bench64::zerohash64_0_byte    ... bench:           4 ns/iter (+/- 0)
test bench64::zerohash64_1024_byte ... bench:       2,112 ns/iter (+/- 184) = 15030 MB/s
test bench64::zerohash64_128_byte  ... bench:         274 ns/iter (+/- 42) = 14481 MB/s
test bench64::zerohash64_16_byte   ... bench:          42 ns/iter (+/- 4) = 11809 MB/s
test bench64::zerohash64_1_byte    ... bench:           8 ns/iter (+/- 0) = 3875 MB/s
test bench64::zerohash64_256_byte  ... bench:         540 ns/iter (+/- 54) = 14696 MB/s
test bench64::zerohash64_4_byte    ... bench:          20 ns/iter (+/- 5) = 6200 MB/s
test bench64::zerohash64_512_byte  ... bench:       1,058 ns/iter (+/- 38) = 15001 MB/s
test bench64::zerohash64_64_byte   ... bench:         144 ns/iter (+/- 6) = 13777 MB/s
test bench64::zerohash64_megabyte  ... bench:   2,610,637 ns/iter (+/- 99,186) = 12451 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 60 measured; 0 filtered out


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```