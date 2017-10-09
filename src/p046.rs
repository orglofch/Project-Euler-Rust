// DONE: 5777
//
// We can just iterate over all the compisite numbers and subtract
// them against prime numbers to see if we can find a square.

fn is_square(num: u64) -> bool {
    return (num as f64).sqrt().fract() == 0.0;
}

fn is_prime(num: u64, lesser_primes: &Vec<u64>) -> bool {
    let sqrt = (num as f64).sqrt().ceil() as u64;
    return lesser_primes
        .iter()
        .take_while(|&&prime| prime <= sqrt)
        .all(|&prime| num % prime != 0);
}

fn passes_goldbach(num: u64, lesser_primes: &Vec<u64>) -> bool {
    return lesser_primes
        .iter()
        .any(|&prime| is_square((num - prime) / 2));
}

fn fails_goldbach_conjecture() -> u64 {
    let mut lesser_primes: Vec<u64> = Vec::new();

    let mut n = 2;
    loop {
        if is_prime(n, &lesser_primes) {
            lesser_primes.push(n);
        } else if !passes_goldbach(n, &lesser_primes) {
            break;
        }

        n += 1;
    }

    return n;
}

fn main() {
    println!("The smallest failing composite is {}", fails_goldbach_conjecture());
}
