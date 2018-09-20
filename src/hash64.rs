use std::ptr;

const PRIME64_1: u64 = 11400714785074694791;
const PRIME64_2: u64 = 14029467366897019727;
const PRIME64_3: u64 = 1609587929392839161;
const PRIME64_4: u64 = 9650029242287828579;
const PRIME64_5: u64 = 2870177450012600261;

#[inline(always)]
fn read_u32(data: &[u8]) -> u32 {
    unsafe { ptr::read(data.as_ptr() as *const u32) }
}

#[inline(always)]
fn read_u64(data: &[u8]) -> u64 {
    unsafe { ptr::read(data.as_ptr() as *const u64) }
}

#[inline(always)]
fn avalanche64(inp: u64) -> u64 {
    let mut h64 = inp ^ (inp >> 33);
    h64 = h64.wrapping_mul(PRIME64_2);
    h64 ^= h64 >> 29;
    h64 = h64.wrapping_mul(PRIME64_3);
    h64 ^= h64 >> 32;
    h64
}

#[inline(always)]
fn xxh64_round(seed: u64, input: u64) -> u64 {
    seed.wrapping_add(input.wrapping_mul(PRIME64_2))
        .rotate_left(31)
        .wrapping_mul(PRIME64_1)
}

#[inline(always)]
fn xxh64_merge_round(acc: u64, val: u64) -> u64 {
    let val = xxh64_round(0, val);
    let acc = acc ^ val;
    acc.wrapping_mul(PRIME64_1).wrapping_add(PRIME64_4)
}

pub fn xxhash64(data: &[u8], seed: u64) -> u64 {
    let mut h64: u64;
    let n = data.len();
    let limit = n - (n & 31);

    let mut acc1;
    let mut acc2;
    let mut acc3;
    let mut acc4;

    let mut t = 0;

    if n >= 32 {
        acc1 = seed.wrapping_add(PRIME64_1).wrapping_add(PRIME64_2);
        acc2 = seed.wrapping_add(PRIME64_2);
        acc3 = seed;
        acc4 = seed.wrapping_sub(PRIME64_1);

        while t < limit {
            acc1 = xxh64_round(acc1, read_u64(&data[t..]));
            acc2 = xxh64_round(acc2, read_u64(&data[t + 8..]));
            acc3 = xxh64_round(acc3, read_u64(&data[t + 16..]));
            acc4 = xxh64_round(acc4, read_u64(&data[t + 24..]));
            t += 32;
        }

        h64 = acc1
            .rotate_left(1)
            .wrapping_add(acc2.rotate_left(7))
            .wrapping_add(acc3.rotate_left(12))
            .wrapping_add(acc4.rotate_left(18));

        h64 = xxh64_merge_round(h64, acc1);
        h64 = xxh64_merge_round(h64, acc2);
        h64 = xxh64_merge_round(h64, acc3);
        h64 = xxh64_merge_round(h64, acc4);
    } else {
        h64 = seed.wrapping_add(PRIME64_5);
    }

    h64 = h64.wrapping_add(data.len() as u64);

    // finalize
    xxh64_finalize(&data[limit..], h64)
}

fn xxh64_finalize(data: &[u8], mut h64: u64) -> u64 {
    #[inline(always)]
    fn process8(v: u64, mut h: u64) -> u64 {
        h ^= xxh64_round(0, v);
        h.rotate_left(27)
            .wrapping_mul(PRIME64_1)
            .wrapping_add(PRIME64_4)
    };

    #[inline(always)]
    fn process4(v: u32, mut h: u64) -> u64 {
        h ^= (v as u64).wrapping_mul(PRIME64_1);
        h.rotate_left(23)
            .wrapping_mul(PRIME64_2)
            .wrapping_add(PRIME64_3)
    };

    #[inline(always)]
    fn process1(v: u8, mut h: u64) -> u64 {
        h ^= (v as u64).wrapping_mul(PRIME64_5);
        h.rotate_left(11).wrapping_mul(PRIME64_1)
    };

    match data.len() {
        24 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process8(read_u64(&data[16..]), h64);
        }
        16 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
        }
        8 => {
            h64 = process8(read_u64(&data), h64);
        }
        28 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process8(read_u64(&data[16..]), h64);
            h64 = process4(read_u32(&data[24..]), h64);
        }
        20 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process4(read_u32(&data[16..]), h64);
        }
        12 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process4(read_u32(&data[8..]), h64);
        }
        4 => {
            h64 = process4(read_u32(&data), h64);
        }
        25 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process8(read_u64(&data[16..]), h64);
            h64 = process1(data[24], h64);
        }
        17 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process1(data[16], h64);
        }
        9 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process1(data[8], h64);
        }
        1 => {
            h64 = process1(data[0], h64);
        }
        29 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process8(read_u64(&data[16..]), h64);
            h64 = process4(read_u32(&data[24..]), h64);
            h64 = process1(data[28], h64);
        }
        21 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process4(read_u32(&data[16..]), h64);
            h64 = process1(data[20], h64);
        }
        13 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process4(read_u32(&data[8..]), h64);
            h64 = process1(data[12], h64);
        }
        5 => {
            h64 = process4(read_u32(&data), h64);
            h64 = process1(data[4], h64);
        }
        26 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process8(read_u64(&data[16..]), h64);
            h64 = process1(data[24], h64);
            h64 = process1(data[25], h64);
        }
        18 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process1(data[16], h64);
            h64 = process1(data[17], h64);
        }
        10 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process1(data[8], h64);
            h64 = process1(data[9], h64);
        }
        2 => {
            h64 = process1(data[0], h64);
            h64 = process1(data[1], h64);
        }
        30 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process8(read_u64(&data[16..]), h64);
            h64 = process4(read_u32(&data[24..]), h64);
            h64 = process1(data[28], h64);
            h64 = process1(data[29], h64);
        }
        22 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process4(read_u32(&data[16..]), h64);
            h64 = process1(data[20], h64);
            h64 = process1(data[21], h64);
        }
        14 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process4(read_u32(&data[8..]), h64);
            h64 = process1(data[12], h64);
            h64 = process1(data[13], h64);
        }
        6 => {
            h64 = process4(read_u32(&data), h64);
            h64 = process1(data[4], h64);
            h64 = process1(data[5], h64);
        }
        27 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process8(read_u64(&data[16..]), h64);
            h64 = process1(data[24], h64);
            h64 = process1(data[25], h64);
            h64 = process1(data[26], h64);
        }
        19 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process1(data[16], h64);
            h64 = process1(data[17], h64);
            h64 = process1(data[18], h64);
        }
        11 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process1(data[8], h64);
            h64 = process1(data[9], h64);
            h64 = process1(data[10], h64);
        }
        3 => {
            h64 = process1(data[0], h64);
            h64 = process1(data[1], h64);
            h64 = process1(data[2], h64);
        }
        31 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process8(read_u64(&data[16..]), h64);
            h64 = process4(read_u32(&data[24..]), h64);
            h64 = process1(data[28], h64);
            h64 = process1(data[29], h64);
            h64 = process1(data[30], h64);
        }
        23 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process8(read_u64(&data[8..]), h64);
            h64 = process4(read_u32(&data[16..]), h64);
            h64 = process1(data[20], h64);
            h64 = process1(data[21], h64);
            h64 = process1(data[22], h64);
        }
        15 => {
            h64 = process8(read_u64(&data), h64);
            h64 = process4(read_u32(&data[8..]), h64);
            h64 = process1(data[12], h64);
            h64 = process1(data[13], h64);
            h64 = process1(data[14], h64);
        }
        7 => {
            h64 = process4(read_u32(&data), h64);
            h64 = process1(data[4], h64);
            h64 = process1(data[5], h64);
            h64 = process1(data[6], h64);
        }
        _ => {}
    }

    avalanche64(h64)
}

#[cfg(test)]
mod tests {
    extern crate twox_hash;
    use self::twox_hash::XxHash;
    use super::xxhash64;
    use std::hash::Hasher;

    fn build_test_cases() -> Vec<Vec<u8>> {
        let mut t: Vec<_> = Vec::new();
        for limit in 0..255 {
            t.push((0..limit).collect());
        }
        t
    }

    #[test]
    fn hash64() {
        let cases = build_test_cases();
        for case in cases {
            let mut hasher = XxHash::with_seed(0xae054331);
            hasher.write(case.as_slice());
            assert_eq!(hasher.finish(), xxhash64(case.as_slice(), 0xae054331));
        }
    }
}
