#![cfg_attr(feature = "bench", feature(test))]
extern crate twox_hash;

extern crate zero_xxhash;

#[cfg(all(feature = "bench", test))]
extern crate fnv;

#[cfg(all(feature = "bench", test))]
extern crate test;

#[cfg(all(feature = "bench", test))]
mod bench32;

#[cfg(all(feature = "bench", test))]
mod bench64;

#[cfg(all(feature = "quickcheck", test))]
extern crate quickcheck;

#[cfg(all(feature = "quickcheck", test))]
extern crate rand;

#[cfg(all(feature = "quickcheck", test))]
extern crate xxhash2;
