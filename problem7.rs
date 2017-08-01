fn is_prime(n: u64) -> bool {
  for f in 2..n {
    if n % f != 0 {
      return false;
    }
  }
  return true;
}

fn prime_10001() -> u64 {
  let mut count = 0;
  let mut num = 1;
  loop {
    if is_prime(num) {
      count = count + 1;
      if count == 10001 {
        return num;
      }
    }
    num = num + 2;
  }
}

fn main() {
  println!("The 1,0001 prime is: {}", prime_10001());
}
