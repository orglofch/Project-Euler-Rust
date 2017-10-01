// DONE: -59231
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
// 1 + a + b (n = 1) needs to be a prime and since b is
// necessarily odd (prime) a must be odd, otherwise
// a + 1 is odd and two odds form an even.
//
// If we really wanted to go overboard we could perform
// random hillclimbing (since I'm sure there are multiple
// local maxima) and use that to try to approximate it.
// Perhaps the function isn't smooth enough though for
// that to be an effective strategy.
//
// For a slight speedup we can also jump right to
// n = our_longest_sequence + 1 to early exit on some
// small subsequences.

use std::collections::HashSet;

fn is_prime(num: i32, lesser_primes: &Vec<u32>) -> bool {
    for prime in lesser_primes.iter() {
        if (num as u32) % prime == 0 {
            return false;
        }
    }
    return true;
}

fn make_primes() -> (HashSet<u32>, Vec<u32>) {
    let mut primes: HashSet<u32> = HashSet::new();
    let mut lesser_primes: Vec<u32> = Vec::new();

    for i in 2..86241 {
        if is_prime(i as i32, &lesser_primes) {
            lesser_primes.push(i);
            primes.insert(i);
        }
    }
    return (primes, lesser_primes);
}

#[inline]
fn quadratic(n: u32, a: i32, b: u32) -> i32 {
    let i32n: i32 = n as i32;
    return i32n * i32n + a * i32n + b as i32;
}

fn quadratic_prime_product() -> i32 {
    let (prime_cache, prime_list) = make_primes();

    let mut max_run: u32 = 0;
    let mut max_a: i32 = 0;
    let mut max_b: u32 = 0;

    for a in (-999..1000).filter(|&n| n % 2 != 0) {
        for b in prime_list.iter().take_while(|&&n| n < 1000) {
            // Skip sequences that are non-zero but smaller than our
            // max so far.
            let mut quad = quadratic(max_run + 1, a, *b);
            if quad < 0 || !prime_cache.contains(&(quad as u32)) {
                continue;
            }

            // Start from the beginning again to verify it's a full sequence.
            let mut num: u32 = 0;
            quad = quadratic(num, a, *b);
            while quad > 0 && prime_cache.contains(&(quad as u32)) {
                num += 1;
                quad = quadratic(num, a, *b);
            }
            if num > max_run {
                max_run = num;
                max_a = a;
                max_b = *b;
            }
        }
    }
    return max_a * max_b as i32;
}

fn main() {
    println!("The quadratic prime product is {}",
             quadratic_prime_product());
}
