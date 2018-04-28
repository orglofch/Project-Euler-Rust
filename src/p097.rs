// DONE: 8739992577
//
// Simple to calculate using continual truncation
// of the digits outside the 10's place.
//
// We could make it cheaper using modular exponentiation tricks
// but it runs fast enough as is.

#[inline]
fn truncate(num: u64) -> u64 {
    num % 10_000_000_000
}

fn last_10_digits() -> u64 {
    let mut num = 2;
    for _ in 0..7830456 {
        num = truncate(num * 2)
    }
    num = num * 28433 + 1;
    return truncate(num);
}

fn main() {
    println!("The last 10 digits of the prime are {}", last_10_digits());
}
