// DONE: 137846528820
// 
// No need to program to do anything beyond the math, 
// it's a simple choose problem.
// 
// You need to take 40 steps total to get to the end.
// 20 of those steps need to be down, 20 need to be right.
// so of the 40 steps we can choose 20 to be down and the
// rest will be right.
//
// Therefore, the total number of unique ways to choose
// a unique sequence of downs is the total number of
// unique patterns! = 40 C 20.

// TODO(orglofch): How do I make the computation stable if I wanted to?
fn main() {
  // TODO(orglofch): How do you do reduce instead of fold?
  let lattices = (21..40)
    .fold(1, |product, n| product * n) / 2;
  println!("The number of lattices is {}", lattices);
}
