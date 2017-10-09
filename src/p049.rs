// DONE: 296962999629
//
// Checking if primes are permutation of one another can be solved using
// a unique prime product generated from the digits.
//
// Once we have a list of all prime permutation we can group them
// into buckets and figure out if we can generate a sequence from
// their elements.

use std::collections::HashMap;

#[doc = /** Primes used in unique product generation. */]
const PRIMES: [u32; 10] = [1, 2, 3, 5, 7, 11, 13, 17, 19, 23];

fn is_prime(num: u32, lesser_primes: &Vec<u32>) -> bool {
    let sqrt = (num as f32).sqrt().ceil() as u32;
    return lesser_primes
        .iter()
        .take_while(|&&prime| prime <= sqrt)
        .all(|&prime| num % prime != 0);
}

fn get_primes() -> Vec<u32> {
    let mut lesser_primes: Vec<u32> = Vec::new();
    let mut primes: Vec<u32> = Vec::new();

    for i in 2..10_000 {
        if is_prime(i, &lesser_primes) {
            if i >= 1000 {
                primes.push(i);
            }

            lesser_primes.push(i);
        }
    }
    return primes;
}

fn unique_product(prime: u32) -> u32 {
    let mut product = 1;
    let mut rem = prime;
    while rem > 0 {
        let digit = rem % 10;
        product *= PRIMES[digit as usize];

        rem /= 10;
    }
    return product;
}

#[inline]
fn concat(min: u32, mid: u32, max: u32) -> u64 {
    return min as u64 * 1_0000_0000 + mid as u64 * 1_0000 + max as u64;
}

fn prime_permutation_concat() -> u64 {
    let primes = get_primes();

    let mut product_buckets: HashMap<u32, Vec<u32>> = HashMap::new();

    for &prime in primes.iter() {
        let product = unique_product(prime);
        product_buckets
            .entry(product)
            .or_insert(Vec::with_capacity(3))
            .push(prime);
    }

    for prime_bucket in product_buckets.iter_mut() {
        let count = prime_bucket.1.len();
        if count < 3 {
            continue;
        }

        // Skip the bucket we already know.
        if prime_bucket.1.contains(&1487) {
            continue;
        }

        prime_bucket.1.sort();

        for i in 0..(count - 2) {
            for j in (i + 1)..(count - 1) {
                for k in (j + 1)..count {
                    let min = prime_bucket.1[i];
                    let mid = prime_bucket.1[j];
                    let max = prime_bucket.1[k];

                    if mid - min == max - mid {
                        return concat(min, mid, max);
                    }
                }
            }
        }
    }
    return 0;
}

fn main() {
    println!(
        "The other prime permutation sum is {}",
        prime_permutation_concat()
    );
}
