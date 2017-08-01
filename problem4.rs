// TODO(orglofch): FINISH ME

#[allow(unused_variables)]
fn is_palindrome(n: u32) -> bool {
  // TODO(orglofch): How do you convert an int to a string?
  // TODO(orglofch): How do you take the log of a number?
  
  return false;
}

fn largest_palindrome() -> u32 {
 let mut largest = 0;
  for i in 999..100 {
    if i * i < largest {
      break;
    }
    for j in i..100 {
      let mult: u32 = i * j;
      if mult < largest {
        break;
      }

      if is_palindrome(mult) {
        largest = mult;
      }
    }
  }
  return largest;
}

fn main() {
  println!("Largest Palindrome: {}", largest_palindrome());  
}
