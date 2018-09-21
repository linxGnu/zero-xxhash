# Bencher 2

This benchmark use [criterion](https://github.com/japaric/criterion.rs) for analyzing and only focus on zero-xxhash and twox-hash/xxhash.

# Run benchmark

```
cargo build --release
./target/release/bencher2
```

# Result

```
xxhash64-16777216       time:   [1.9262 ms 1.9299 ms 1.9339 ms]                               
                        change: [-0.0822% +0.1750% +0.4285%] (p = 0.18 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

zero-xxhash64-16777216  time:   [1.9249 ms 1.9292 ms 1.9341 ms]                                    
                        change: [-0.1205% +0.0940% +0.3149%] (p = 0.40 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

xxhash64-16384          time:   [1.6723 us 1.6723 us 1.6724 us]                            
                        change: [+0.0190% +0.0258% +0.0328%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  6 (6.00%) high mild
  1 (1.00%) high severe

zero-xxhash64-16384     time:   [1.6653 us 1.6654 us 1.6655 us]                                 
                        change: [+0.1564% +0.1631% +0.1702%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low mild
  8 (8.00%) high mild
  3 (3.00%) high severe

xxhash64-8192           time:   [849.84 ns 850.06 ns 850.32 ns]                           
                        change: [-0.1293% -0.0988% -0.0702%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 22 outliers among 100 measurements (22.00%)
  16 (16.00%) high mild
  6 (6.00%) high severe

zero-xxhash64-8192      time:   [836.34 ns 836.39 ns 836.47 ns]                                
                        change: [+0.0255% +0.0367% +0.0516%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low mild
  7 (7.00%) high mild
  4 (4.00%) high severe
```