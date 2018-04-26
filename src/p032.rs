// DONE: 45228
//
// The simplest way that comes to mind it just iterating over the set of
// pandigitals, splitting them up and checking if the product matches.
//
// a * b = c
//
// Note, the number of digits in a * b can't be larger than
// 5 otherwise it's impossible for c = a * b given the remaining digits.

use std::collections::HashSet;

/// Converts a digit vec into a number.
#[inline]
fn as_number(digits: &[u8]) -> u32 {
    let mut num = 0_u32;
    for digit in digits {
        num = num * 10 + *digit as u32;
    }
    return num;
}

fn permutation_products(permutation: &Vec<u8>, products: &mut HashSet<u32>) {
    for i in 0..4 {
        let a = as_number(&permutation[0..(i + 1)]);
        if a == 1 {
            break;
        }

        for j in (i + 1)..5 {
            let b = as_number(&permutation[(i + 1)..(j + 1)]);
            let c = as_number(&permutation[(j + 1)..]);

            if a * b == c {
                products.insert(c);
            }
        }
    }
}

fn pandigital_products_sum() -> u32 {
    let mut permutation: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut products: HashSet<u32> = HashSet::new();

    loop {
        // Generate the valid products.
        permutation_products(&permutation, &mut products);

        // Permutate the previous number in place.
        let mut k = 8;
        for i in 0..8 {
            if permutation[i] < permutation[i + 1] {
                k = i;
            }
        }
        if k == 8 {
            break;
        }

        let mut l = 9;
        for i in (k + 1)..9 {
            if permutation[k] < permutation[i] {
                l = i;
            }
        }
        if l == 9 {
            break;
        }

        permutation.swap(k, l);
        permutation[(k + 1)..].reverse();
    }

    return products.iter().sum();
}

fn main() {
    println!("The sum of pandigital products is {}", pandigital_products_sum());
}
