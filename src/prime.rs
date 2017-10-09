pub fn is_prime(num: u64, lesser_primes: &mut Vec<u64>) -> bool {
  if num % 2 == 0 {
    return false;
  }
  
  let sqrt = (num as f64).sqrt().ceil() as u64;
  let is_prime = lesser_primes.iter()
    .take_while(|&prime| prime <= &sqrt)
    .all(|&prime| num % prime != 0);
  if is_prime {
    lesser_primes.push(num);
  }
  return is_prime;
}
