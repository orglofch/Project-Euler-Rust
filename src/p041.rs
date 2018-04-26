// DONE: 7652413
//
// We know the pandigital needs to be prime, so the smallest digit
// needs to be odd and can't be a 5.
//
// Using the prime number theorem we can approximate the number of prime numbers:
// Prime numbers <= 987654321 ~= 987654321 / (log(987654321) - 1)
//                            ~= 50,000,000
//
// The number of pandigital numbers = 9! = 362,880.
//
// Thus, we should iterate over the set of pandigital numbers rather than the
// set of primes since it's significantly smaller. However, without a
// precomputed set of primes, checking to see if a number is prime will
// become more expensive.
//
// We also know n >= 4 since the question already gives us a 4 digit
// pandigital prime.
//
// Because pandigital nubmers of the same length always have the same digit sum
// we can rule out numbers which follow summation divisor rules.
//
// 9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1 = 45 (divisible by 3)
// 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1 = 36 (divisible by 3)
// 7 + 6 + 5 + 4 + 3 + 2 + 1 = 28 (not divisible by 3)
//
// Therefore we can ignore the 9 and 8 length pandigitals.

/// Convert a digit vector into a number.
#[inline]
fn as_number(digits: &Vec<u8>) -> u32 {
    let mut num = 0_u32;
    for digit in digits.iter() {
        num = num * 10 + *digit as u32;
    }
    return num;
}

fn is_prime(num: u32) -> bool {
    let sqrt = (num as f32).sqrt().ceil() as u32;
    return (2..(sqrt + 1)).all(|other| num % other != 0);
}

fn largest_pandigital_prime_of_length(n: usize) -> Option<u32> {
    assert!(n > 0, "n should be > 0");
    assert!(n <= 9, "Pandigital numbers can't have lengths > 9");

    // Generate the initial permutation.
    let mut permutation: Vec<u8> = Vec::new();
    for i in (1..(n + 1)).rev() {
        permutation.push(i as u8);
    }

    loop {
        let num = as_number(&permutation);

        if is_prime(num) {
            return Some(num);
        }

        // Permutate the previous number in place.
        // Note, we use descending ordering since we want to check
        // the highest values first.
        let mut k = n - 1;
        for i in 0..(n - 1) {
            if permutation[i] > permutation[i + 1] {
                k = i;
            }
        }
        if k == n - 1 {
            break;
        }

        let mut l = n;
        for i in (k + 1)..n {
            if permutation[k] > permutation[i] {
                l = i;
            }
        }
        if l == n {
            break;
        }

        permutation.swap(k, l);
        permutation[(k + 1)..].reverse();
    }

    return None;
}

fn largest_pandigital_prime() -> u32 {
    for n in (4..8).rev() {
        let lpp = largest_pandigital_prime_of_length(n);
        if lpp.is_some() {
            return lpp.unwrap();
        }
    }
    panic!("Function should terminate in the inner loop");
}

fn main() {
    println!(
        "The largest pandigital prime is {}",
        largest_pandigital_prime()
    );
}
