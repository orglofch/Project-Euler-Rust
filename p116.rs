// Done: 20492570929
//  
// We can use our function from the former problems.

use std::collections::HashMap;

fn combinations(remaining: i64, block_size: i64, cache: &mut HashMap<i64, i64>) -> i64 {
  if remaining <= block_size - 1 {
    return 0_i64;
  } else if remaining == block_size {
    return 1_i64;
  }

  match cache.get(&remaining) {
   Some(&number) => return number,
   _ => {},
  }

  let mut num = 0_i64;
  for start in 0..(remaining - block_size + 1) {
    num = num + 1 + combinations(remaining - start - block_size, block_size, cache);
  }
  cache.insert(remaining, num);
  return num;
}

fn main() {
  let mut red_cache: HashMap<i64, i64> = HashMap::new();
  let mut green_cache: HashMap<i64, i64> = HashMap::new();
  let mut blue_cache: HashMap<i64, i64> = HashMap::new();

  let red_combos = combinations(50, 2, &mut red_cache);
  let green_combos = combinations(50, 3, &mut green_cache);
  let blue_combos = combinations(50, 4, &mut blue_cache);

  println!("Red: {}, Green: {}, Blue: {}", red_combos, green_combos, blue_combos);

  println!("Total combos: {}", red_combos + green_combos + blue_combos);
}
