// DONE: 31875000

// TODO(orglofch): This could be optimized if we expand
// a^2 + b^2 = c^2 using the condition of a.

fn pythagorean_triplet_product() -> u32 {
  // TODO(orglofch): How do you do decreasing sequences?
  for c in 1..998 {
    for b in 1..(1000 - c) { 
      let a = 1000 - (b + c);
      let ab_sqr_sum = a*a + b*b;
      if ab_sqr_sum == c*c {
         return a * b * c;
      }
    }
  }
  return 0;
}

fn main() {
  println!("The pythagorean triplet product is: {}", pythagorean_triplet_product());
}
