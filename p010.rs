// DONE: 142913828922 

extern crate prime;

fn main() {
  let mut sum = 2_u64; // We skip 2 in is_prime.
  let mut lesser_primes: Vec<u64> = Vec::new(); 
  for n in 2..2_000_000 {
    if prime::is_prime(n, &mut lesser_primes) {
      sum = sum + n;
    }
  }
  println!("The sum of the primes is {}", sum);
}
