// DONE: 9183
//
// We don't want to actually calculate the power and compare
// as that's definitely overkill.
//
// We can decompose numbers into their unique prime products
// and gather terms to uniquely identify the numbers since
// prime products are necessarily unique.
//
// For example, 4^2 = (2 * 2) ^2 = (2 * 2) * (2 * 2) = 2^4 etc.

use std::collections::HashSet;

// Note, we could store these as a set for faster checks on
// the number which are prime but it's not that worth it.
const PRIMES: [u32; 25] = [
    2,
    3,
    5,
    7,
    11,
    13,
    17,
    19,
    23,
    29,
    31,
    37,
    41,
    43,
    47,
    53,
    59,
    61,
    67,
    71,
    73,
    79,
    83,
    89,
    97,
];

fn unique_product(a: u32, b: u32) -> [u32; 25] {
    let mut products: [u32; 25] = [0; 25];

    // Find the unique product for a.
    let mut rem = a;
    while rem != 1 {
        for i in 0..PRIMES.len() {
            if rem % PRIMES[i] == 0 {
                products[i] += 1;
                rem = rem / PRIMES[i];
                break;
            }
        }
    }

    // Apply the ppwer to all decomposed factors.
    for i in products.iter_mut() {
        *i *= b;
    }

    return products;
}

fn distinct_powers() -> u32 {
    let mut distinct: HashSet<[u32; 25]> = HashSet::new();

    for a in 2..101 {
        for b in 2..101 {
            distinct.insert(unique_product(a, b));
        }
    }
    return distinct.len() as u32;
}

fn main() {
    println!("There are {} distinct power", distinct_powers());
}
