// Done: 168
//  
// We can use our function from the former problem.
//
// TODO(orglofch): For some of these "Find the first that ..." problems
// we might be able to use some form of binary search to traverse
// the space of possibilities to make it slightly master.

use std::collections::HashMap;

fn combinations(remaining: i64, min_block_size: i64, cache: &mut HashMap<i64, i64>) -> i64 {
  if remaining <= min_block_size - 1 {
    return 0_i64;
  } else if remaining == min_block_size {
    return 1_i64;
  }

  match cache.get(&remaining) {
   Some(&number) => return number,
   _ => {},
  }

  let mut num = 0_i64;
  for b_size in min_block_size..(remaining + 1) {
    let mut num_for_block = 0_i64;
    for start in 0..(remaining - b_size + 1) {
      num_for_block = num_for_block + 1 + combinations(remaining - start - b_size - 1, min_block_size, cache);
    }
    num = num + num_for_block;
  }
  cache.insert(remaining, num);
  return num;
}

fn main() {
  let mut cache: HashMap<i64, i64> = HashMap::new();
  for n in 100..1000 {
    let combos = combinations(n, 50, &mut cache) + 1;
    if combos > 1_000_000 {
      println!("Row: {}, combos: {}", n, combos);
      break;
    } 
  }
}
