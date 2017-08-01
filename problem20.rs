// DONE: 648
//
// The simplest solution would be to just write a large nubmer handler again.

use std::ops::Mul;

/** Max value for a single part. */
const MAX_VALUE: u32 = 1_000_000;

#[derive(Debug)]
struct LargeNumber {
    parts: Vec<u32>,
}

impl LargeNumber {
    fn of(num: u32) -> LargeNumber {
        let mut parts: Vec<u32> = Vec::new();
        parts.push(num);
        LargeNumber { parts: parts }
    }
}

impl Mul for LargeNumber {
    type Output = LargeNumber;

    fn mul(self: LargeNumber, other: LargeNumber) -> LargeNumber {
        let mut new_parts: Vec<u32> = Vec::new();

        if other.parts.len() != 1 {
            panic!("The other number should only have one part!");
        }

        let num: u32 = other.parts[0];

        let mut carry: u32 = 0;
        for part in self.parts.into_iter() {
            let mult = part * num + carry;

            new_parts.push(mult % MAX_VALUE);
            carry = mult / MAX_VALUE;
        }

        if carry > 0 {
            new_parts.push(carry);
        }

        LargeNumber { parts: new_parts }
    }
}

fn sum() -> u64 {
    let mut factorial: LargeNumber = LargeNumber::of(1);
    for i in 2..101 {
        factorial = factorial * LargeNumber::of(i);
    }

    let mut sum: u64 = 0;
    for part in factorial.parts.into_iter() {
        let mut rem = part;
        while rem != 0 {
            sum += (rem % 10) as u64;
            rem = rem / 10;
        }
    }
    return sum;
}

fn main() {
    println!("The sum is {}", sum());
}
