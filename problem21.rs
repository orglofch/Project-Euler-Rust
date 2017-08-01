// DONE: 31626
//
// Seems fairly straightforward, we know we can skip number's we've
// already computer so we can store an index of all the computed values
// etc.

use std::collections::HashMap;

fn proper_divisor_sum(num: u32) -> u32 {
    let sqrt_bound: u32 = (num as f32).sqrt() as u32;
    let mut sum = 1_u32;
    for i in 2..(sqrt_bound + 1) {
        if num % i != 0 {
            continue;
        }

        if i == sqrt_bound {
            sum += i;
        } else {
            sum += i + (num / i);
        }
    }

    return sum;
}

fn amicable_sum() -> u32 {
    let mut proper_divisor_sum_cache: HashMap<u32, u32> = HashMap::new();

    let mut amicable_sum = 0_u32;
    for i in 1..10000 {
        let divisor_sum = proper_divisor_sum(i);
        if divisor_sum >= 10000 {
            continue;
        }

        match proper_divisor_sum_cache.get(&divisor_sum) {
            Some(&sum) if sum == i => amicable_sum += i + divisor_sum,
            _ => (),
        }
        proper_divisor_sum_cache.insert(i, divisor_sum);
    }
    return amicable_sum;
}

fn main() {
    println!("The sum of amicable numbers < 10000 is {}", amicable_sum());
}
