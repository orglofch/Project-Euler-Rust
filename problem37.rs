// DONE: 748317
//
// We can exclude all numbers that have a even number in them
// aside from a 2 at the beginning since that can still technically be prime.
//
// We know our number needs to start with one of [2, 3, 5, 7]
// and end with one of [3, 5, 7] for the final truncation iteration to be prime.
//
// Truncatable primes must also start with a truncatable prime we've already found.

use std::collections::HashSet;
use std::iter::FromIterator;

fn is_prime(num: u32, lesser_primes: &Vec<u32>) -> bool {
    return lesser_primes.iter().all(|&n| num % n != 0);
}

/**
 * Note, we only pass the cache here, all truncation primes should be in this cache since
 * the truncation are always smaller numbers than the larger prime.
 */
fn is_truncatable_prime(prime: u32, lesser_truncatable_primes: &HashSet<u32>, cache: &HashSet<u32>) -> bool {
    let digits = (prime as f32).log(10_f32) as u32;

    let high = prime / 10_u32.pow(digits);
    if high != 2 && high != 3 && high != 5 && high != 7 {
        return false;
    }

    let low = prime % 10;
    if low != 3 && low != 5 && low != 7 {
        return false;
    }

    // TODO(orglofch): Use lesser trunctable primes.

    let mut forward = prime;
    let mut backward = low;
    for i in 0..digits {
        forward /= 10;

        if !cache.contains(&forward) {
            return false;
        }

        let power = 10_u32.pow(i + 1);
        backward += ((prime / power) % 10) * power;

        if !cache.contains(&backward) {
            return false;
        }
    }
    return true;
}

fn truncatable_prime_sum() -> u32 {
    let mut cache: HashSet<u32> = HashSet::new();

    // Add the basic numbers which don't count a truncatable.
    cache.insert(2);
    cache.insert(3);
    cache.insert(5);
    cache.insert(7);

    let mut lesser_primes: Vec<u32> = Vec::from_iter(cache.iter().cloned());
    let mut lesser_truncatable_primes: HashSet<u32> = HashSet::from_iter(cache.iter().cloned());

    let mut num = 11;
    while lesser_truncatable_primes.len() < 15 {
        if is_prime(num, &lesser_primes) {
            cache.insert(num);
            lesser_primes.push(num);

            if is_truncatable_prime(num, &lesser_truncatable_primes, &cache) {
                println!("{} is truncatable prime", num);
                lesser_truncatable_primes.insert(num);
            }
        }
        num += 2;
    }

    let sum: u32 = lesser_truncatable_primes.into_iter().sum();
    // Remove the 1 digit values.
    return sum - 17;
}

fn main() {
    println!("Truncatable prime sum is {}", truncatable_prime_sum());
}
