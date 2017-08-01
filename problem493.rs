// DONE:
//
// There's probably some nice closed form solution for this
// but lets use a Monte-Carlo solution instead.

extern crate rand;

use rand::Rng;

/* @return the number of unique elements drawn. */
fn simulate_single(bag: &mut [u8; 100]) -> u8 {
    // Select 20 elements at random.
    let mut found: [bool; 10] = [false; 10];
    for i in 0..20 {
        let last_pos = 99 - i;
        let pos = rand::thread_rng().gen_range(0, last_pos) as usize;

        let element = bag[pos];
        found[element as usize] = true;

        // Swap the element in place.
        bag[pos] = bag[last_pos];
        bag[last_pos] = element;
    }

    let mut count = 0;
    for i in 0..10 {
        if found[i] {
            count += 1;
        }
    }
    return count;
}

fn simulate() -> f64 {
    let mut bag: [u8; 100] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        6, 6, 6, 6, 6, 6, 6, 6, 6, 6,
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        9, 9, 9, 9, 9, 9, 9, 9, 9, 9
    ];

    let mut expected: f64 = 0.0;
    let mut checkpoint = expected;
    let mut i = 0.0;
    loop {
        // Note, there's no need tp shuffle between takes because we always
        // start picking randomly regardless.
        let elements = simulate_single(&mut bag) as f64;

        let old_sum = expected * ii ;
        expected = (old_sum + elements) / (i + 1.0);

        // Exit if we haven't really changed since the last checkpoint.
        if i as u64 % 100000 == 0 {
            if (expected - checkpoint).abs() < 0.0000000001 {
                break;
            }
            checkpoint = expected;

            println!("Checkpoint: {:.*}", 9, checkpoint);
        }

        i += 1.0;
    }
    return expected;
}

fn main() {
    println!("Expected number of distinct elements is {:.*}", 9, simulate());
}
