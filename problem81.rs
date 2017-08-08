// DONE: 427337
//
// Another dynamic programming problem.
//
// We can start at the end and work backwards due to the contraints
// of the problem which allows us to generate the minimal path sum.

use std::io::{BufRead, BufReader};
use std::fs::File;

const MATRIX_FILE: &'static str = "p081_matrix.txt";

fn load_file() -> [[u32; 80]; 80] {
    let file = BufReader::new(File::open(MATRIX_FILE).unwrap());

    let mut matrix = [[0; 80]; 80];

    for (i, line) in file.lines().enumerate() {
        for (j, number) in line.unwrap().split(char::is_whitespace).enumerate() {
            matrix[i][j] = number.trim().parse().unwrap();
        }
    }
    return matrix;
}

fn min_path_sum() -> u32 {
    let mut matrix = load_file();

    for r in (0..80).rev() {
        for c in (0..80).rev() {
            matrix[r][c] = match (r, c) {
                (79, 79) => matrix[r][c],
                (79, _) => matrix[r][c] + matrix[r][c + 1],
                (_, 79) => matrix[r][c] + matrix[r + 1][c],
                _ => {
                    matrix[r][c] + std::cmp::min(matrix[r][c + 1], matrix[r + 1][c])
                }
            };
        }
    }
    return matrix[0][0];
}

fn main() {
    println!("Min path sum: {}", min_path_sum());
}
