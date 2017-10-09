// DONE: 8581146
//
// Just keep track of two sets so we can build
// an ever increasing determine whether we converge to 89
// or 1.
//
// We can take advantage of permutations as well for a potential
// speed up since those will have equal sums.
//
// Permutations can be stored using a unique prime product.

use std::collections::HashSet;

// Note, although 1 isn't a prime we count it as contributing a net zero
// to the unique product.
const PRIMES: [u64; 10] = [1, 2, 3, 5, 7, 11, 13, 17, 19, 23];

fn digit_sqr_sum(num: u64) -> u64 {
    let mut rem = num;
    let mut sqr_sum = 0;
    while rem > 0 {
        let r = rem % 10;
        sqr_sum += r*r;
        rem = rem / 10;
    }
    return sqr_sum;
}

fn unique_prod(num: u64) -> u64 {
    let mut rem = num;
    let mut unique_prod: u64 = 1;
    while rem > 0 {
        unique_prod *= PRIMES[(rem % 10) as usize];
        rem = rem / 10;
    }
    return unique_prod;
}

fn converge_to_89_chains() -> u64 {
    let mut chain_1: HashSet<u64> = HashSet::with_capacity(1000);
    let mut chain_89: HashSet<u64> = HashSet::with_capacity(1000);

    chain_1.insert(unique_prod(1));
    chain_89.insert(unique_prod(89));

    let mut count = 0;
    for i in 1..10_000_000 {
        let mut num = i;
        let mut chain: Vec<u64> = Vec::new();
        loop {
            let prime_prod = unique_prod(num);
            if chain_1.contains(&prime_prod) {
                for &n in chain.iter() {
                    chain_1.insert(n);
                }
                break;
            }
            if chain_89.contains(&prime_prod) {
                for &n in chain.iter() {
                    chain_89.insert(n);
                }
                count += 1;
                break;
            }
            chain.push(prime_prod);
            num = digit_sqr_sum(num);
        }
    }
    return count;
}

fn main() {
    println!("{} numbers converge to 89 on the chain", converge_to_89_chains());
}
