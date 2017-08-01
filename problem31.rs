// Done: 73682
//
// Simple dynamic programming using a cache.

const DENOMINATIONS: [u64; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

use std::collections::HashMap;

fn get_ways(value: u64, cache: &mut HashMap<(u64, usize), u64>, denom: &[u64]) -> u64 {
  if value == 0 {
    return 1;
  }

  match cache.get(&(value, denom.len())) {
   Some(&ways) => return ways,
   _ => {},
  }

  let mut ways = 0_u64;
  for i in (0..denom.len()).rev() {
    let mut remaining = value;
    let d = denom[i];

    loop {
      remaining = remaining - d;
      ways = ways + get_ways(remaining, cache, &denom[0 .. i]);
      if d > remaining {
        break;
      }
    }
  }
  cache.insert((value, denom.len()), ways);
  return ways;
}

fn main() {
  let mut cache: HashMap<(u64, usize), u64> = HashMap::new(); 
  println!("There are {} ways to make 200", get_ways(200, &mut cache, &DENOMINATIONS));
}
