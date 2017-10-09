// DONE: 40730
//
// 9! * 7 (2,540,160) <<< 9,999,999 and adding digits is never going to make that better
// so we know we at least only need to check numbers < 10,000,000.
//
// Hard-code the factorials since we only need them for digits 0..9.

const FACTORIALS: [u32; 10] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

fn digit_factorial_sum(num: u32) -> u32 {
    let mut sum = 0;

    let mut rem = num;
    while rem > 0 {
        let digit = rem % 10;
        sum += FACTORIALS[digit as usize];
        rem /= 10;
    }
    return sum;
}

fn digit_factorials_total() -> u32 {
    let mut sum = 0;
    for i in 3..10_000_000 {
        let digit_sum = digit_factorial_sum(i);
        if digit_sum == i {
            sum += i;
        }
    }
    return sum;
}

fn main() {
    println!("The digit factorial sum is {}", digit_factorials_total());
}
