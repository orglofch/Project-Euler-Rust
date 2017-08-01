// DONE: 4613732

fn even_fib_number_sum() -> u32 {
  let mut sum = 0; 
  let mut prev = 0;
  let mut cur = 1;
  loop {
    let next = prev + cur;
    if next > 4_000_000 {
      break;
    } else if next % 2 == 0 {
      sum += next;
    }
   
    prev = cur;
    cur = next;
  }
  return sum;
}

fn main() {
  println!("Even Fibinacci number sum {}", even_fib_number_sum());
}
