// DONE: 709
//
// We can simply commpare logs instead of powers to make the
// calculation simpler since logs still preserve inequalities.

use std::io::{BufRead, BufReader};
use std::fs::File;

const EXPONENTIAL_FILE: &'static str = "p099_base_exp.txt";

fn largest_exponential_line() -> u32 {
    let file = BufReader::new(File::open(EXPONENTIAL_FILE).unwrap());

    let mut max_exponential_index = 0;
    let mut max_exponential_log = 0.0;
    for (i, line) in file.lines().enumerate() {
        let mut columns = match line {
            Ok(ref line) => line[..].split(','),
            Err(e) => panic!("Failed to read line due to {}", e),
        };

        let num = columns.next().unwrap().parse::<u32>().unwrap();
        let exp = columns.next().unwrap().parse::<u32>().unwrap();

        let log_value = exp as f64 * (num as f64).ln();

        if log_value > max_exponential_log {
            max_exponential_log = log_value;
            max_exponential_index = i;
        }
    }
    return (max_exponential_index + 1) as u32;
}

fn main() {
    println!("The line with the largest exponential is {}", largest_exponential_line());
}
