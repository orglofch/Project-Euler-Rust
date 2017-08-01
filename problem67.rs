// DONE:
//
// Same as problem 18 with file reading.

use std::io::{BufRead, BufReader};
use std::fs::File;

const TRIANGLE_FILE: &'static str = "p067_triangle.txt";

fn load_file() -> [[u32; 100]; 100] {
  let file = BufReader::new(File::open(TRIANGLE_FILE).unwrap());

  let mut triangle = [[0; 100]; 100];
 
  // preallocate the array and read the data into it
  for (i, line) in file.lines().enumerate() {
    for (j, number) in line.unwrap().split(char::is_whitespace).enumerate() {
      triangle[i][j] = number.trim().parse().unwrap();
    }
  }
  return triangle;
}

#[allow(unused_variables)]
fn main() {
  let triangle: [[u32; 100]; 100] = load_file();
  let mut row: [u32; 100] = triangle[99];
  for r in (0..(triangle.len() - 1)).rev() {
    let mut next_row: [u32; 100] = [0; 100];
    for c in 0..(r + 1) {
      next_row[c] = triangle[r][c] + std::cmp::max(row[c], row[c + 1]);
    }
    row = next_row;
  }
  println!("The maximum path is: {}", row[0]);
}
