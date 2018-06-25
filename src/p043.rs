// DONE: 16695334890
//
// This seems like a pretty straightforward constraint satisfactions problem with the
// following constraints base on various divisibility rules we know:
// 1) d4 is even since d2d3d4 needs to be divisible by 2.
// 2) d3 + d4 + d5 is divisible by 3 since d3d4d5 is divisible by 3.
// 3) d6 is a 0 or a 5 since d4d5d6 needs to be divisible by 5.
// 4) d5d6 - 2 * d7 is divisible by 7 since d5d6d7 is divisible by 7.
// 5) d5d6 + 5 * d7 is divisible by 7 since d5d6f6 is divisible by 7.
// 6) 3 * d5 + d6 is divisible by 7 since d5d6d7 is divisible by 7.
// 7) 2 * d5 + d6d7 is divisble by 7 since d5d6d7 is divisible by 7.
// 8) 2 * d5 + 3 * d6 + d7 is divisible by 7 since d5d6d7 is divisble by 7.
// 9) d7 + 3 * d5d6 is divisible by 7 since d5d6d7 is divisible by 7.
// 10) 3 * d7 + 2 * d5d6 is divisible by 7 since d5d6d7 is divisible by 7.
// 11) d6 - d7 + d8 is divisible by 11 since d6d7d8 is divisible by 11.
// 12) d7d8 + d6 is divisible by 11 since d6d7d8 is divisible by 11.
// 13) d6d7 - d8 is divisible by 11 since d6d7d8 is divisible by 11.
// 14) d6d7 + 10 * d8 is divisible by 11 since d6d7d8 is divisible by 11.
// 15) d7 - d6 - d8 is divisible by 11 since d6d7d8 is divisible by 11.
// 16) 4 * d9 + d7d8 is divisible by 13 since d7d8d9 is divisible by 13.
// 17) 4 * d7 - d8d9 is divisible by 13 since d7d8d9 is divisible by 13.
// 18) d7d8 - 9 * d9 is divisible by 13 since d7d8d9 is divisible by 13.
// 19) d8d9 - 5 * d10 is divisible by 17 since d8d9d10 is divisible by 17.
// 20) 2 * d8 - d9d10 is divisible by 17 since d8d9d10 is divisible by 17.
//
// However, we'll see how fast it is through enumeration alone and go from
// there.

fn is_valid_pandigital(pandigital: &Vec<i32>) -> bool {
    // d2d3d4 is divisible by 2.
    if pandigital[3] % 2 != 0 {
        return false;
    }

    // d3d4d5 is divisible by 3.
    if (pandigital[2] + pandigital[3] + pandigital[4]) % 3 != 0 {
        return false;
    }

    // d4d5d6 is divisible by 5.
    if pandigital[5] != 0 && pandigital[5] != 5 {
        return false;
    }

    // d5d6d7 is divisible by 7.
    if (2 * pandigital[4] + 3 * pandigital[5] + pandigital[6]) % 7 != 0 {
        return false;
    }

    // d6d7d8 is divisible by 11.
    if (pandigital[5] - pandigital[6] + pandigital[7]) % 11 != 0 {
        return false;
    }

    // d7d8d9 is divisible by 13.
    if (4 * pandigital[6] - (10 * pandigital[7] + pandigital[8])) % 13 != 0 {
        return false;
    }

    // d8d9d10 is divisible by 17.
    if (2 * pandigital[7] - (10 * pandigital[8] + pandigital[9])) % 17 != 0 {
        return false;
    }

    return true;
}

fn as_number(pandigital: &Vec<i32>) -> u64 {
    let mut num = 0_u64;
    for digit in pandigital.iter() {
        num = num * 10 + *digit as u64;
    }
    return num;
}

fn substring_pandigital_sum() -> u64 {
    // Note, we exclude permutations with leading 0's.
    let mut permutation: Vec<i32> = vec![1, 0, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut sum = 0_u64;

    let digits = permutation.len();

    loop {
        if is_valid_pandigital(&permutation) {
            sum += as_number(&permutation);
        }

        // Permutate the previous number in place.
        let mut k = digits - 1;
        for i in 0..(digits - 1) {
            if permutation[i] < permutation[i + 1] {
                k = i;
            }
        }
        if k == (digits - 1) {
            break;
        }

        let mut l = digits;
        for i in (k + 1)..digits {
            if permutation[k] < permutation[i] {
                l = i;
            }
        }
        if l == digits {
            break;
        }

        permutation.swap(k, l);
        permutation[(k + 1)..].reverse();
    }

    return sum;
}

fn main() {
    println!(
        "The substring pandigital sum is {}",
        substring_pandigital_sum()
    );
}
