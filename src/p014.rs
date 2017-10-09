// DONE: 837799
//
// Just keep track of a hashmap of starts to
// chain lengths. If we encounter an etnry we
// can just use the hashed entry.
//
// We can also skip even number since those are
// going to converge to 1 the fastest.
//
// I'd be surprised if there was a better way
// to predict which element are going to produce
// the longest chains. We definitely can't work
// backwards.

use std::collections::HashMap;

fn collatz_chain_length(num: u64, cache: &mut HashMap<u64, u32>) -> u32 {
    let mut iter = num;
    let mut count = 0;

    while iter != 1 {
        match cache.get(&iter) {
            Some(&val) => {
                cache.insert(num, count + val);
                return count + val;
            },
            None => (),
        }

        if iter % 2 == 0 {
            iter = iter / 2;
        } else {
            iter = 3 * iter + 1;
        }
        count += 1;
    }

    cache.insert(num, count);
    return count;
}

fn longest_collatz_chain() -> u64 {
    let mut cache: HashMap<u64, u32> = HashMap::new();
    let mut max_length = 0;
    let mut max_val = 0;
    for i in 1..1_000_000 {
        if i % 2 == 0 {
            continue;
        }

        let length = collatz_chain_length(i, &mut cache);
        if length > max_length {
            max_length = length;
            max_val = i;
        }
    }
    return max_val;
}

fn main() {
    println!("The valu which produces the longest collatz chain is {}", longest_collatz_chain());
}
