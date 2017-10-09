// DONE: 55
//
// We are already given the primes which are less than 100 so we
// can skip those, which also lets use skip all numbers which have
// even numbers in them, since they will necessarily be multiples
// of 2 after some amount of rotation.
//
// This limits our number search to 5^6 (15,625) numbers which is
// significantly less than 1,000,000. However, in reality it's
// less than this since we're only counting prime numbers.
//
// We migtht as well preecompute a list of all primes less than
// 1,000,000 since that gives or primes checker a quick way of
// checking for whether a number is prime (based on ordering).
// Instead of checking against an incomplete set (in the case of
// only checking the numbers created with odd numbers).

use std::collections::HashSet;

/** Assumes all primes < num are already present in the cache. */
fn is_prime(num: u32, smaller_primes: &Vec<u32>) -> bool {
    let num_sqrt = (num as f32).sqrt() as u32;
    for prime in smaller_primes.iter() {
        if *prime > num_sqrt {
            break;
        }

        if num % prime == 0 {
            return false;
        }
    }
    return true;
}

fn is_all_odd_digits(num: u32) -> bool {
    let mut rem = num;
    while rem > 0 {
        let digit = rem % 10;
        if digit % 2 != 1 {
            return false;
        }
        rem /= 10;
    }
    return true;
}

fn is_circular_prime(prime: u32, prime_cache: &HashSet<u32>) -> bool {
    // Find the highest power in the number / number of digits.
    let mut highest_power = 1;
    let mut digits = 0;

    let mut rem = prime;
    while rem > 0 {
        highest_power *= 10;
        digits += 1;
        rem /= 10;
    }
    // Overcount by one.
    highest_power /= 10;

    // Note, we don't have to check the last rotation since that's
    // the initial rotation.
    let mut current = prime;
    for _ in 0..digits {
        // Rotate the number. We already know the original is prime.
        let smallest_digit = current % 10;
        current = current / 10 + smallest_digit * highest_power;

        if !prime_cache.contains(&current) {
            return false;
        }
    }
    return true;
}

fn circular_primes() -> u32 {
    let mut prime_cache: HashSet<u32> = HashSet::new();
    let mut smaller_primes: Vec<u32> = Vec::new();
    let mut odd_primes: Vec<u32> = Vec::new();

    // Precalculate primes.
    for i in 2..1_000_000 {
        if is_prime(i, &smaller_primes) {
            prime_cache.insert(i);
            smaller_primes.push(i);

            if is_all_odd_digits(i) {
                odd_primes.push(i);
            }
        }
    }

    let mut count = 1_u32;
    for prime in odd_primes.into_iter() {
        if is_circular_prime(prime, &prime_cache) {
            count += 1;
        }
    }
    return count;
}

fn main() {
    println!("The number of circular primes < 1,000,000 is {}",
             circular_primes());
}
