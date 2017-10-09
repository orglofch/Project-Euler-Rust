// TODO
//
// At first glance it appears that a good approach would be to just
// iterate over the set of all primes and create some unique index
// which represents that number (minus some values). Whenever,
// we discover a new prime we can split it out into these indexes
// and check if any of the buckets have 8 values in them.
//
// Bucketing by length is an obvious first step, once we step into
// a larger bucket length we can toss out the previous bucket.
//
// The second step would be two mask out each digit in each number.
// E.g. 4121 produces mask indexes:
// -121, 4-2-, 41-1
//
// We could us 0's instead of dashes but that messes with out ability
// to distinguish real zeros in the number which makes it more difficult
// to recreate the smallest number.

use std::collections::HashMap;

fn is_prime(num: u32, lesser_primes: &Vec<u32>) -> bool {
    let sqrt = (num as f32).sqrt().ceil() as u32;
    return lesser_primes
        .iter()
        .take_while(|&&prime| prime <= sqrt)
        .all(|&prime| num % prime != 0);
}

fn prime_digit_replacement() -> u32 {
    let mut lesser_primes: Vec<u32> = Vec::new();
    let mut index: HashMap<u32, Vec<u32>> = HashMap::new();

    let mut digits = 1;

    let mut n = 2;
    loop {
        if !is_prime(n, &lesser_primes) {
            n += 1;
            continue;
        }

        if (n as f32).log(10.0) > digits as f32 {
            index.clear();
            digits += 1;
            n += 1;
            continue;
        }



        lesser_primes.push(n);
        n += 1;
    }

    return 0;
}

fn main() {
    println!("Prime digit replacement is {}", prime_digit_replacement());
}
