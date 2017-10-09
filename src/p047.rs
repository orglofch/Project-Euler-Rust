// DONE: 134043
//
// Should be fairly simple, we'll just accumulate primes
// and use those to find prime factors and we'll remember
// the factors between iterations to check uniqueness.
//
// Oh wait, this problem is slightly poorly worded and
// doesn't care if factors between number are unique, even better.

use std::collections::HashSet;

fn is_prime(num: u32, lesser_primes: &Vec<u32>) -> bool {
    let sqrt = (num as f32).sqrt().ceil() as u32;
    return lesser_primes
        .iter()
        .take_while(|&&prime| prime <= sqrt)
        .all(|&prime| num % prime != 0);
}


fn prime_factors(num: u32, lesser_primes: &Vec<u32>) -> HashSet<u32> {
    let mut factors: HashSet<u32> = HashSet::new();

    let mut rem = num;
    while rem != 1 {
        for prime in lesser_primes.iter() {
            if rem % prime == 0 {
                factors.insert(*prime);
                rem = rem / prime;
                break;
            }
        }
    }
    return factors;
}

fn distinct_prime_factors() -> (u32, u32, u32, u32) {
    let mut lesser_primes: Vec<u32> = Vec::new();

    let mut consequetive = 0;

    let mut n = 2;
    loop {
        if is_prime(n, &lesser_primes) {
            lesser_primes.push(n);
            consequetive = 0;
            n += 1;
            continue;
        }

        let factors = prime_factors(n, &lesser_primes);

        if factors.len() != 4 {
            consequetive = 0;
        } else {
            consequetive += 1;
        }

        if consequetive == 4 {
            break;
        }

        n += 1;
    }
    return (n - 3, n - 2, n - 1, n);
}

fn main() {
    println!("The first four consequetive numbers to have distinct prime factors are {:?}", distinct_prime_factors());
}
