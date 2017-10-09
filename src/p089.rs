// TODO
//
// Which can simply create an encoding script that runs back to front
// (since it's assumed that lesser values always appear at the end)
// and compress the text until it's no longer compressible.

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::string::String;

const ROMAN_FILE: &'static str = "p089_roman.txt";

fn load_file() -> [str; 6] {
  let file = BufReader::new(File::open(ROMAN_FILE).unwrap());

  let mut numerals = [""; 6];
 
  for (i, line) in file.lines().enumerate() {
    numerals[i] = line.unwrap().trim();
  }
  return numerals;
}

fn main() {
  let numerals = load_file();
  println!("Numerals {:?}", numerals);
}
