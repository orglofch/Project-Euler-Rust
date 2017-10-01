// DONE: 997651
//
// Seems like we can do this fairly easily with dynamic programming to
// find all prime runs once we have a list of all primes.
//
// We know our run is going to be larger than 21 elements.
// Therefore, the largest start of a sequence which is still less than
// 1,000,000 must be < 45,455 since 45,455 * 22 > 1,000,000.
//
// Instead of finding all primes < 1,000,000 we'll only find those
// which create sequences. Then we'll perform a more expensive check
// of the sequences we find.

fn is_prime(num: u32, lesser_primes: &Vec<u32>) -> bool {
    let sqrt = (num as f32).sqrt().ceil() as u32;
    return lesser_primes
        .iter()
        .take_while(|&&prime| prime <= sqrt)
        .all(|&prime| num % prime != 0);
}

/** Determine if a number is prime given an incomplete set of lesser primes. */
fn is_prime_full(num: u32, primes: &Vec<u32>) -> bool {
    let sqrt = (num as f32).sqrt().ceil() as u32;
    let divisible_by_known_primes = primes
        .iter()
        .take_while(|&&prime| prime <= sqrt)
        .any(|&prime| num % prime == 0);
    if divisible_by_known_primes {
        return false;
    }

    // Otherwise try all other intermediate numbers.
    let last_prime = primes[primes.len() - 1];
    for i in last_prime..sqrt {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

fn get_primes() -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::new();
    primes.push(2);

    let mut num = 3;
    while num < 45_455 {
        if is_prime(num, &primes) {
            primes.push(num);
        }
        num += 2;
    }
    return primes;
}

#[inline]
fn index2d(y: usize, x: usize, width: usize) -> usize {
    return y * width + x;
}

fn consecutive_prime_sum() -> u32 {
    let primes = get_primes();

    let num_primes = primes.len();
    let mut runs = vec![0; num_primes * num_primes];

    // Note, length is indexed as length 1 == 0.

    for start in 0..num_primes {
        let i = index2d(start, 0, num_primes);
        runs[i] = primes[start];
    }
    for start in 0..num_primes {
        for length in 1..(num_primes - start) {
            let cur_i = index2d(start, length, num_primes);
            let prev_i = index2d(start, length - 1, num_primes);

            let run = runs[prev_i] + primes[start + length];

            runs[cur_i] = run;
        }
    }

    for length in (0..num_primes).rev() {
        for start in 0..num_primes {
            let i = index2d(start, length, num_primes);
            if runs[i] == 0 || runs[i] > 1_000_000 {
                continue;
            }

            if is_prime_full(runs[i], &primes) {
                return runs[i];
            }
        }
    }
    return 0;
}

fn main() {
    println!("The largest consecutive prime sum is {}",
             consecutive_prime_sum());
}
