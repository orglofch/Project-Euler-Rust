// TODO(orgloch): FINISH ME

fn is_prime(n: u64) -> bool {
  println!("Is prime: {}", n);
  return !(2..n).any(|f| n % f == 0);
}

fn main() {
  let num = 600_851_475_143_u64;

  // TODO(orglofch): How do you do sqrt?
  let mut largest_prime_factor = 1_u64;
  let mut factor = num - 2; // odd.
  loop {
    if factor == 2 {
      break;
    }

    if num % factor == 0 && is_prime(factor) {
      largest_prime_factor = factor;
      break; 
    }

    // TODO(orglofch): Is there i++ or ++i?
    factor = factor - 2; // odd.
  }

  println!("{:?}", largest_prime_factor);
}
