// DONE: 24174150

fn sum_square_difference() -> u32 {
  let sum = (1..100).fold(0, |sum, n| sum + n);
  let sum_square: u32 = sum * sum;

  let square_sum: u32 = (1..100)
    .map(|n| n * n)
    .fold(0, |sum, n| sum + n);

  return sum_square - square_sum;
}

fn sum_square_difference_alt() -> u32 {
  let mut sum = 0_u32;
  for i in 1..100 {
    for j in (i + 1)..100 {
      if i != j {
        sum += 2 * i * j;
      }     
    }
  }
  return sum;
}

fn main() {
  println!("Sum square difference: {}", sum_square_difference());
  println!("Sum square difference alt: {}", sum_square_difference_alt());
}
