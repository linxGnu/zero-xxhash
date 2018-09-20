const PRIME32_1: u32 = 2654435761;
const PRIME32_2: u32 = 2246822519;
const PRIME32_3: u32 = 3266489917;
const PRIME32_4: u32 = 668265263;
const PRIME32_5: u32 = 374761393;

use std::ptr;

#[inline(always)]
fn read_u32(data: &[u8]) -> u32 {
    unsafe { ptr::read(data.as_ptr() as *const u32) }
}

#[inline(always)]
fn avalanche32(mut inp: u32) -> u32 {
    inp ^= inp >> 15;
    inp = inp.wrapping_mul(PRIME32_2);
    inp ^= inp >> 13;
    inp = inp.wrapping_mul(PRIME32_3);
    inp ^= inp >> 16;
    inp
}

#[inline(always)]
fn xxh32_round(acc: u32, input: u32) -> u32 {
    acc.wrapping_add(input.wrapping_mul(PRIME32_2))
        .rotate_left(13)
        .wrapping_mul(PRIME32_1)
}

pub fn xxhash32(data: &[u8], seed: u32) -> u32 {
    let mut h32: u32;
    let n = data.len();
    let limit = n - (n & 15);

    let mut acc1;
    let mut acc2;
    let mut acc3;
    let mut acc4;

    let mut t = 0;

    if n >= 16 {
        acc1 = seed.wrapping_add(PRIME32_1).wrapping_add(PRIME32_2);
        acc2 = seed.wrapping_add(PRIME32_2);
        acc3 = seed;
        acc4 = seed.wrapping_sub(PRIME32_1);

        while t < limit {
            acc1 = xxh32_round(acc1, read_u32(&data[t..]));
            acc2 = xxh32_round(acc2, read_u32(&data[t + 4..]));
            acc3 = xxh32_round(acc3, read_u32(&data[t + 8..]));
            acc4 = xxh32_round(acc4, read_u32(&data[t + 12..]));
            t += 16;
        }

        h32 = acc1
            .rotate_left(1)
            .wrapping_add(acc2.rotate_left(7))
            .wrapping_add(acc3.rotate_left(12))
            .wrapping_add(acc4.rotate_left(18));
    } else {
        h32 = seed.wrapping_add(PRIME32_5);
    }

    h32 = h32.wrapping_add(n as u32);

    // finalize
    xxh32_finalize(&data[limit..], h32)
}

fn xxh32_finalize(data: &[u8], mut h32: u32) -> u32 {
    #[inline(always)]
    fn process4(v: u32, mut h: u32) -> u32 {
        h = h.wrapping_add(v.wrapping_mul(PRIME32_3));
        h = h.rotate_left(17);
        h.wrapping_mul(PRIME32_4)
    };

    #[inline(always)]
    fn process1(v: u8, mut h: u32) -> u32 {
        h = h.wrapping_add((v as u32).wrapping_mul(PRIME32_5));
        h = h.rotate_left(11);
        h.wrapping_mul(PRIME32_1)
    };

    match data.len() {
        12 => {
            h32 = process4(read_u32(data), h32);
            h32 = process4(read_u32(&data[4..]), h32);
            h32 = process4(read_u32(&data[8..]), h32);
        }
        8 => {
            h32 = process4(read_u32(data), h32);
            h32 = process4(read_u32(&data[4..]), h32);
        }
        4 => {
            h32 = process4(read_u32(data), h32);
        }
        13 => {
            h32 = process4(read_u32(data), h32);
            h32 = process4(read_u32(&data[4..]), h32);
            h32 = process4(read_u32(&data[8..]), h32);
            h32 = process1(data[12], h32);
        }
        9 => {
            h32 = process4(read_u32(data), h32);
            h32 = process4(read_u32(&data[4..]), h32);
            h32 = process1(data[8], h32);
        }
        5 => {
            h32 = process4(read_u32(data), h32);
            h32 = process1(data[4], h32);
        }
        1 => {
            h32 = process1(data[0], h32);
        }
        14 => {
            h32 = process4(read_u32(data), h32);
            h32 = process4(read_u32(&data[4..]), h32);
            h32 = process4(read_u32(&data[8..]), h32);
            h32 = process1(data[12], h32);
            h32 = process1(data[13], h32);
        }
        10 => {
            h32 = process4(read_u32(data), h32);
            h32 = process4(read_u32(&data[4..]), h32);
            h32 = process1(data[8], h32);
            h32 = process1(data[9], h32);
        }
        6 => {
            h32 = process4(read_u32(data), h32);
            h32 = process1(data[4], h32);
            h32 = process1(data[5], h32);
        }
        2 => {
            h32 = process1(data[0], h32);
            h32 = process1(data[1], h32);
        }
        15 => {
            h32 = process4(read_u32(data), h32);
            h32 = process4(read_u32(&data[4..]), h32);
            h32 = process4(read_u32(&data[8..]), h32);
            h32 = process1(data[12], h32);
            h32 = process1(data[13], h32);
            h32 = process1(data[14], h32);
        }
        11 => {
            h32 = process4(read_u32(data), h32);
            h32 = process4(read_u32(&data[4..]), h32);
            h32 = process1(data[8], h32);
            h32 = process1(data[9], h32);
            h32 = process1(data[10], h32);
        }
        7 => {
            h32 = process4(read_u32(data), h32);
            h32 = process1(data[4], h32);
            h32 = process1(data[5], h32);
            h32 = process1(data[6], h32);
        }
        3 => {
            h32 = process1(data[0], h32);
            h32 = process1(data[1], h32);
            h32 = process1(data[2], h32);
        }
        _ => {}
    }

    avalanche32(h32)
}

#[cfg(test)]
mod tests {
    extern crate twox_hash;
    use self::twox_hash::XxHash32;
    use super::xxhash32;
    use std::hash::Hasher;

    fn build_test_cases() -> Vec<Vec<u8>> {
        let mut t: Vec<_> = Vec::new();
        for limit in 0..255 {
            t.push((0..limit).collect());
        }
        t
    }

    #[test]
    fn hash32() {
        let cases = build_test_cases();
        for case in cases {
            let mut hasher = XxHash32::with_seed(0xae054331);
            hasher.write(case.as_slice());
            assert_eq!(
                hasher.finish(),
                xxhash32(case.as_slice(), 0xae054331) as u64
            );
        }
    }
}
