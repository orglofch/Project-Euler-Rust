// TODO
//
// Fairly simply, especially if we already have a hard coded
// list of primes, but a cache with a calculator should suffice.
//
// We could probably add a hard limit to the number of primes
// based on the min and max values which can be used for a and b.
//
// Knowing the primes ahead of time has some value since the
// is_prime check only need to check against other primes.
//
// I'm guessing n < 79 otherwise the "amazing" formula
// n^2 - 79n + 1601 wouldn't be so amazing since there would
// be a number with small coefficients that outperforms it.
//
// Therefore, the largeest number we need to consider to
// generate our primes list is 79^2 + 1000 * 79 + 1000
// = 86241.
//
// n also has to include 0. Which implies that b itself
// need to be a positive prime number.
//
// If we really wanted to go overboard we could perform
// random hillclimbing (since I'm sure there are multiple
// local maxima) and use that to try to approximate it.
// Perhaps the function isn't smooth enough though for
// that to be an effective strategy.

use std::collections::HashSet;

fn is_prime(num: i32, cache: &HashSet<u32>) -> bool {
    if num < 0 {
        return false
    }

    if cache.contains(&(num as u32)) {
        return true;
    }

    for prime in cache.iter() {
        if (num as u32) % prime == 0 {
            return false;
        }
    }
    return true;
}

fn make_primes() -> HashSet<u32> {
    let mut primes: HashSet<u32> = HashSet::new();

    for i in 2..86241 {
        if is_prime(i as i32, &primes) {
            primes.insert(i);
        }
    }
    return primes;
}

fn quadratic(n: u32, a: i32, b: u32) -> i32 {
    let i32n: i32 = n as i32;
    return i32n*i32n + a * i32n + b as i32;
}

fn quadratic_prime_product() -> i32 {
    let primes = make_primes();

    let mut max_run: u32 = 0;
    let mut max_a: i32 = 0;
    let mut max_b: u32 = 0;

    for a in -1000..1001 {
        // TODO(orglofch): limit b to primes.
        for b in primes.iter() {
            if *b > 1000 {
                break;
            }

            let mut num: u32 = 0;
            let mut quad = quadratic(num, a, *b);
            while is_prime(quad, &primes) {
                num += 1;
                quad = quadratic(num, a, *b);
            }
            if num > max_run {
                max_run = num;
                max_a = a;
                max_b = *b;
            }
            println!("n^2 + {} * n + {} has run {}", a, b, num);
        }
    }
    return max_a * max_b as i32;
}

fn main() {
    println!("The quadratic prime product is {}", quadratic_prime_product());
}
