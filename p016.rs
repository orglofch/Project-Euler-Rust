// DONE: 1366
//
// 2^1000
// = 2^32 * 2^(1000 - 32)

// Note, must fit into a u32.
// Determines how the number is divided.
const MAX_POWER: u32 = 100_000_000;

#[derive(Debug)]
struct LargeNumber {
    parts: Vec<u32>,
}

impl LargeNumber {
    fn new() -> LargeNumber {
        LargeNumber { parts: Vec::new() }
    }
}

fn power_digit_sum() -> u64 {
    let mut number: LargeNumber = LargeNumber::new();

    let start_num: u32 = 1 << 31;
    number.parts.push(start_num % MAX_POWER);
    number.parts.push(start_num / MAX_POWER);

    // Calculate the sum.
    for _ in 0..969 {
        let mut carry: u32 = 0;
        for part in number.parts.iter_mut() {
            let mult = *part * 2 + carry;
            *part = mult % MAX_POWER;
            carry = mult / MAX_POWER;
        }
        if carry > 0 {
            number.parts.push(carry);
        }
    }

    // Sum the digits.
    let mut sum: u64 = 0;
    for part in number.parts.into_iter() {
        let mut rem = part;
        while rem > 0 {
            sum += (rem % 10) as u64;
            rem /= 10;
        }
    }

    return sum;
}

fn main() {
    println!("The power digit sum is {}", power_digit_sum());
}
