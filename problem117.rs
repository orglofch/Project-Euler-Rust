// Done: 100808458960497
//
// We can use our function from the former problems.

use std::collections::HashMap;

fn combinations(remaining: i64, cache: &mut HashMap<i64, i64>) -> i64 {
  if remaining <= 1 {
    return 0_i64;
  } else if remaining == 2 {
    return 1_i64;
  }

  match cache.get(&remaining) {
   Some(&number) => return number,
   _ => {},
  }

  let mut num = 0_i64;
  for block_size in 2..5 {
    for start in 0..(remaining - block_size + 1) {
      num = num + 1 + combinations(remaining - start - block_size, cache);
    }
  }
  cache.insert(remaining, num);
  return num;
}

fn main() {
  let mut cache: HashMap<i64, i64> = HashMap::new();

  let combos = combinations(50, &mut cache) + 1;

  println!("Total combos: {}", combos);
}
