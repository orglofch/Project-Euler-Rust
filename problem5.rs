// TODO(orglofch): FINISH ME

fn divisible_by_1_through_20(n: u32) -> bool {
  // TODO(orglofch): How do you do decreasing sequences?
  for i in 2..20 {
    if n % i != 0 {
      return false;
    }
  }
  return true;
}

fn smallest_divisible_by_1_through_20() -> u32 {
  // TODO(orglofch): How do you do 'find first'?
  for n in 380.. {
    if divisible_by_1_through_20(n) {
      return n;
    }
  }
  return 0;
}

fn main() {
 println!("Smallest number divisible by 1 through 20: {}", smallest_divisible_by_1_through_20()); 
}
