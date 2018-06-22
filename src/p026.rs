// DONE: 983
//
// To convert a fraction to a decimal we:
// 1) Multiply the remainder by 10
// 2) Set digit = remainder / d
// 3) Set remainder = remainder % 10
// 4) Go-to 1
//
// We know if we see the same remainder at any point will result in
// the same sequence of number since the calculation is repeated
// deterministically (Note the digit doesn't imply that same thing since
// the same digit can be created by multiple remainder states).
//
// Once we detect a repeating remainder we can check that last time we
// saw that remainder and use that as the cycle length.

use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn cycle_length(d: u32) -> u32 {
    let mut encountered_by_remainder: HashMap<u32, u32> = HashMap::new();

    let mut i = 0;
    let mut remainder = 1;
    while remainder != 0 {
        match encountered_by_remainder.entry(remainder) {
            Entry::Occupied(entry) => return i - entry.get(),
            Entry::Vacant(entry) => entry.insert(i),
        };

        remainder = (remainder * 10) % d;

        i += 1;
    }

    return 0;
}

fn longest_reciprocal_cycle(d_limit: u32) -> u32 {
    let mut max_cycle = 0;
    let mut max_d = 0;
    for d in 1..d_limit {
        let cycle = cycle_length(d);

        if cycle > max_cycle {
            max_cycle = cycle;
            max_d = d;
        }
    }
    return max_d;
}

fn main() {
    println!(
        "The longest reciprocal cycle is for 1/{}",
        longest_reciprocal_cycle(1000)
    );
}
