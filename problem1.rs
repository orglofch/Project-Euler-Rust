// DONE: 233168

fn sum_multiples_of_3_or_5() -> u32 {
    return (0..1000)
               .filter(|&n| n % 5 == 0 || n % 3 == 0)
               .fold(0, |sum, n| sum + n);
}

fn main() {
    println!("Sum multiples for 3 or 5: {}", sum_multiples_of_3_or_5());
}
