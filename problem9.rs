// DONE: 31875000
//
// This could be optimized if we expand
// a^2 + b^2 = c^2 using the condition of a.

use std::cmp;

fn pythagorean_triplet_product() -> u32 {
    for c in 1..998 {
        for b in (1..cmp::min(c - 1, 1000 - c)).rev() {
            let a = 1000 - (b + c);

            if a > b {
                break;
            }

            let ab_sqr_sum = a * a + b * b;
            if ab_sqr_sum == c * c {
                return a * b * c;
            }
        }
    }
    return 0;
}

fn main() {
    println!("The pythagorean triplet product is: {}",
             pythagorean_triplet_product());
}
