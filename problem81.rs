// TODO
//
// Another trivial dynamic programming problem.
//
// We can start at the end and work backwards due to the contraints
// of the problem which allows us to generate the minimal path sum.

use std::io::{BufRead, BufReader};
use std::fs::File;

const MATRIX_FILE: &'static str = "p081_matrix.txt";

fn load_file() -> [[u32; 5]; 5] {
  let file = BufReader::new(File::open(MATRIX_FILE).unwrap());

  let mut matrix = [[0; 5]; 5];
 
  for (i, line) in file.lines().enumerate() {
    for (j, number) in line.unwrap().split(char::is_whitespace).enumerate() {
      matrix[i][j] = number.trim().parse().unwrap();
    }
  }
  return matrix;
}

fn main() {
  let matrix = load_file();
  
  let mut min_sum_matrix = [[0; 5]; 5];
  
  min_sum_matrix[4][4] = matrix[4][4];

  for r in (0..5).rev() {
    for c in (0..5).rev() {
      min_sum_matrix[r][c] = match (r, c) {
        (4, 4) => matrix[4][4],
        (4, _) => matrix[r][c] + min_sum_matrix[r][c + 1],
        (_, 4) => matrix[r][c] + min_sum_matrix[r + 1][c],
        _ => matrix[r][c] + std::cmp::min(min_sum_matrix[r][c + 1], min_sum_matrix[r + 1][c]), 
      };
    }
  }

  println!("Min path sum: {}", min_sum_matrix[0][0]);
}
