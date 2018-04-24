// DONE: 9110846700
//
// We can just expand powers into multiplication
// and truncate the higher order bits whenever we pass 10 digits.
//
// E.g. 999^999 = 999 * 999 * 999 ...
//              = (998,001)^998
//              = (997,002,999)^997
//              = (996,005,996,001)^996
//              = (990,000,000,000 + 6,005,996,001)^996
// And we know the multiplication of 990,000,000,000 with a whole
// number can't affect the lower 10 digits.
// The same applies for addition.

const TRUNCATION_POINT: u64 = 10_000_000_000;
const HIGHEST_POWER: u16 = 1000;

#[inline]
fn truncate(val: u64) -> u64 {
    val % TRUNCATION_POINT
}

fn last_ten_digits() -> u64 {
    let mut sum = 0_u64;
    for i in 1..(HIGHEST_POWER as u64 + 1) {
        let mut product = i;
        for _ in 1..(i as u64) {
            // It's faster to perform the truncation on each step rather than
            // check if it needs to be truncated.
            product = truncate(product * i);
        }
        // We need to truncate the sum as well so it doesn't grow unbounded.
        sum = truncate(sum + product);
    }
    return sum;
}

fn main() {
    println!("The last ten digits of the sequence 1^2 + 2^2 + ... + 1000^1000 are {:010}", last_ten_digits());
}
