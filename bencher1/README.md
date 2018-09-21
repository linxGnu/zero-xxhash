# Bencher 1

This benchmark compare siphash, fnv, xxhash and zero-xxhash all in one place.

# Run benchmark

```
rustup run nightly cargo bench --features=bench
```

# Result

```
running 60 tests
test bench32::xxhash32_0_byte      ... bench:           7 ns/iter (+/- 2)
test bench32::xxhash32_1024_byte   ... bench:         301 ns/iter (+/- 24) = 6803 MB/s
test bench32::xxhash32_128_byte    ... bench:          41 ns/iter (+/- 8) = 6243 MB/s
test bench32::xxhash32_16_byte     ... bench:          11 ns/iter (+/- 1) = 2909 MB/s
test bench32::xxhash32_1_byte      ... bench:          12 ns/iter (+/- 2) = 166 MB/s
test bench32::xxhash32_256_byte    ... bench:          83 ns/iter (+/- 14) = 6168 MB/s
test bench32::xxhash32_32_byte     ... bench:          15 ns/iter (+/- 2) = 4266 MB/s
test bench32::xxhash32_4_byte      ... bench:          10 ns/iter (+/- 1) = 800 MB/s
test bench32::xxhash32_512_byte    ... bench:         150 ns/iter (+/- 22) = 6826 MB/s
test bench32::xxhash32_megabyte    ... bench:     278,979 ns/iter (+/- 54,589) = 7517 MB/s
test bench32::zerohash32_0_byte    ... bench:           2 ns/iter (+/- 0)
test bench32::zerohash32_1024_byte ... bench:         276 ns/iter (+/- 48) = 7420 MB/s
test bench32::zerohash32_128_byte  ... bench:          36 ns/iter (+/- 5) = 7111 MB/s
test bench32::zerohash32_16_byte   ... bench:           7 ns/iter (+/- 1) = 4571 MB/s
test bench32::zerohash32_1_byte    ... bench:           3 ns/iter (+/- 0) = 666 MB/s
test bench32::zerohash32_256_byte  ... bench:          70 ns/iter (+/- 13) = 7314 MB/s
test bench32::zerohash32_32_byte   ... bench:          11 ns/iter (+/- 1) = 5818 MB/s
test bench32::zerohash32_4_byte    ... bench:           3 ns/iter (+/- 0) = 2666 MB/s
test bench32::zerohash32_512_byte  ... bench:         138 ns/iter (+/- 17) = 7420 MB/s
test bench32::zerohash32_megabyte  ... bench:     277,474 ns/iter (+/- 26,249) = 7558 MB/s
test bench64::fnvhash_0_byte       ... bench:           0 ns/iter (+/- 0)
test bench64::fnvhash_1024_byte    ... bench:       2,161 ns/iter (+/- 206) = 947 MB/s
test bench64::fnvhash_128_byte     ... bench:         279 ns/iter (+/- 61) = 917 MB/s
test bench64::fnvhash_16_byte      ... bench:          33 ns/iter (+/- 6) = 969 MB/s
test bench64::fnvhash_1_byte       ... bench:           2 ns/iter (+/- 0) = 1000 MB/s
test bench64::fnvhash_256_byte     ... bench:         540 ns/iter (+/- 38) = 948 MB/s
test bench64::fnvhash_4_byte       ... bench:           8 ns/iter (+/- 0) = 1000 MB/s
test bench64::fnvhash_512_byte     ... bench:       1,080 ns/iter (+/- 70) = 948 MB/s
test bench64::fnvhash_64_byte      ... bench:         135 ns/iter (+/- 13) = 948 MB/s
test bench64::fnvhash_megabyte     ... bench:   2,330,381 ns/iter (+/- 318,062) = 899 MB/s
test bench64::siphash_0_byte       ... bench:           9 ns/iter (+/- 2)
test bench64::siphash_1024_byte    ... bench:         917 ns/iter (+/- 71) = 2233 MB/s
test bench64::siphash_128_byte     ... bench:         125 ns/iter (+/- 18) = 2048 MB/s
test bench64::siphash_16_byte      ... bench:          24 ns/iter (+/- 4) = 1333 MB/s
test bench64::siphash_1_byte       ... bench:          12 ns/iter (+/- 2) = 166 MB/s
test bench64::siphash_256_byte     ... bench:         229 ns/iter (+/- 21) = 2235 MB/s
test bench64::siphash_4_byte       ... bench:          13 ns/iter (+/- 0) = 615 MB/s
test bench64::siphash_512_byte     ... bench:         450 ns/iter (+/- 33) = 2275 MB/s
test bench64::siphash_64_byte      ... bench:          65 ns/iter (+/- 8) = 1969 MB/s
test bench64::siphash_megabyte     ... bench:     897,636 ns/iter (+/- 96,180) = 2336 MB/s
test bench64::xxhash64_0_byte      ... bench:           9 ns/iter (+/- 1)
test bench64::xxhash64_1024_byte   ... bench:         149 ns/iter (+/- 45) = 13744 MB/s
test bench64::xxhash64_128_byte    ... bench:          30 ns/iter (+/- 3) = 8533 MB/s
test bench64::xxhash64_16_byte     ... bench:          15 ns/iter (+/- 1) = 2133 MB/s
test bench64::xxhash64_1_byte      ... bench:          17 ns/iter (+/- 1) = 117 MB/s
test bench64::xxhash64_256_byte    ... bench:          46 ns/iter (+/- 3) = 11130 MB/s
test bench64::xxhash64_4_byte      ... bench:          16 ns/iter (+/- 0) = 500 MB/s
test bench64::xxhash64_512_byte    ... bench:          79 ns/iter (+/- 5) = 12962 MB/s
test bench64::xxhash64_64_byte     ... bench:          21 ns/iter (+/- 3) = 6095 MB/s
test bench64::xxhash64_megabyte    ... bench:     138,646 ns/iter (+/- 11,434) = 15125 MB/s
test bench64::zerohash64_0_byte    ... bench:           3 ns/iter (+/- 0)
test bench64::zerohash64_1024_byte ... bench:         151 ns/iter (+/- 41) = 13562 MB/s
test bench64::zerohash64_128_byte  ... bench:          25 ns/iter (+/- 1) = 10240 MB/s
test bench64::zerohash64_16_byte   ... bench:          10 ns/iter (+/- 0) = 3200 MB/s
test bench64::zerohash64_1_byte    ... bench:           4 ns/iter (+/- 0) = 500 MB/s
test bench64::zerohash64_256_byte  ... bench:          41 ns/iter (+/- 2) = 12487 MB/s
test bench64::zerohash64_4_byte    ... bench:           5 ns/iter (+/- 0) = 1600 MB/s
test bench64::zerohash64_512_byte  ... bench:          75 ns/iter (+/- 3) = 13653 MB/s
test bench64::zerohash64_64_byte   ... bench:          18 ns/iter (+/- 0) = 7111 MB/s
test bench64::zerohash64_megabyte  ... bench:     138,870 ns/iter (+/- 9,956) = 15101 MB/s

test result: ok. 0 passed; 0 failed; 0 ignored; 60 measured; 0 filtered out
```