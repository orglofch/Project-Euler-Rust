// Done: 16475640049
//
// This problem should be solvable by some form of dynamic programming approach.
// For example,
// The number of ways to place blocks of size >=3 on a line of length 8 is based
// (atleast partially) on the number of ways to place blocks of size >= 3 on a
// line of length 4 after a 3 block has been placed on positions [0, 3] (with
// an extra spacer placed as well).
//
// The also means that the combinations are somewhat symmetric.
// 
// Some potential base cases are:
// 1) Block size <= 2 => 0
// 1) Block size == 3 => 1
// 2) Block size == 4 -> 3
//
// If we start on the left (moving right)
// and never allow for blocks to be placed to the left of the moving piece
// then we also prevent duplication (and account for all possibilities).
//
// For example,
// For block size 4:
// We can place a single 4 block.
// Or 2 times placing a 3 block which starts at the front.

use std::collections::HashMap;

const UNITS: i64 = 50;

fn combinations(remaining: i64, cache: &mut HashMap<i64, i64>) -> i64 {
  if remaining <= 2 {
    return 0_i64;
  } else if remaining == 3 {
    return 1_i64;
  }

  match cache.get(&remaining) {
    Some(&number) => return number,
     _ => {},
  }
  
  let mut num = 0_i64;
  for b_size in 3..(remaining + 1) {
    let mut num_for_block = 0_i64;
    for start in 0..(remaining - b_size + 1) {
      num_for_block = num_for_block + 1 + combinations(remaining - start - b_size - 1, cache);
    }
    num = num + num_for_block;
  }
  cache.insert(remaining, num);
  return num;  
}

fn main() {
  let mut cache: HashMap<i64, i64> = HashMap::new();
  println!("There are {} combinations", combinations(UNITS, &mut cache) + 1); 
}
