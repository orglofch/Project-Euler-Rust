// DONE: 4179871
//
// Well firt we know we don't need to check that many numbers
// since the problem states it needs to be < 28123.
//
// Once we have all abundant numbers we can find the unique
// sums of all those numbers less than the threshold. Then take
// all number's not in that set and add them together.

use std::collections::HashSet;

const MAX: u32 = 28123;

fn is_abundant(num: u32) -> bool {
    let mut factors: HashSet<u32> = HashSet::new();
    factors.insert(1);

    let sqrt_bound = (num as f32).sqrt().floor() as u32;
    for i in 2..(sqrt_bound + 1) {
        if num % i == 0 {
            factors.insert(i);
            factors.insert((num / i) as u32);
        }
    }

    let sum: u32 = factors.into_iter().sum();
    return sum > num;
}

fn abundant_numbers() -> Vec<u32> {
    let mut already_seen: HashSet<u32> = HashSet::new();
    let mut result: Vec<u32> = Vec::new();
    for i in 1..MAX {
        if !already_seen.contains(&i) && is_abundant(i) {
            already_seen.insert(i);
            result.push(i);
        }
    }
    return result;
}

fn nonabundant_sum() -> u64 {
    let abundant_numbers = abundant_numbers();
    let mut abundant_sums: HashSet<u32> = HashSet::new();
    for i in 0..abundant_numbers.len() {
        for j in i..abundant_numbers.len() {
            let sum = abundant_numbers[i] + abundant_numbers[j];
            if sum <= MAX {
                abundant_sums.insert(sum);
            }
        }
    }

    let mut sum: u64 = 0;
    for i in 0..MAX {
        if !abundant_sums.contains(&i) {
            sum += i as u64;
        }
    }
    return sum;
}

fn main() {
    println!("The non-abundant sum is {}", nonabundant_sum());
}
