// DONE: 162
//
// This is fairly simple, we can just read all the words in,
// compute their values, and then check to see if those values
// are triangle numbers by solving the quadratic formula.
//
// the formula is -1/2 + (0.25 + 2 * n)^0.5 = natural number.

use std::fs::File;
use std::io::{BufRead, BufReader};

const NAMES: &'static str = "p042_words.txt";

fn is_triangle(num: u64) -> bool {
    return ((2 * num) as f64 + 0.25).sqrt().fract() == 0.5;
}

fn word_score(chars: &[u8]) -> u64 {
    let mut score: u64 = 0;
    for c in chars {
        score += *c as u64 - 64;
    }
    return score;
}

fn triangle_words() -> u64 {
    let file = BufReader::new(File::open(NAMES).unwrap());

    let mut total = 0;
    for line in file.lines() {
        for name in line.unwrap().split(",") {
            let bytes = name.as_bytes();
            // Exclude the quotes.
            let score = word_score(&bytes[1..bytes.len() - 1]);
            if is_triangle(score) {
                total += 1;
            }
        }
    }
    return total;
}

fn main() {
    println!("The total number of triangle words is {}", triangle_words());
}
