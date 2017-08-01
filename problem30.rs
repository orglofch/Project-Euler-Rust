// DONE: 443839
//
// The check itself is simple, we may just need a way
// of deciding what the treshold at which we can stop checking
// numbers is, so we'll print them out and see if we can see
// the logical stopping point.
//
// We know that the largest digit is 9^5 = 59049.
// A 7 digit number can have max value 413343 which is only 6 digits.
// A 6 digit number can have max value 354294 which is 6 digits.
// Therefore, we should still check 6 digit numbers. There might be
// a tighter bound than that though...
//
// The fact that all the ^4 sums are 4 digits isn't a fact
// we should actually be able to take advantage of.

fn sum_of_digit_powers(num: u32) -> u32 {
    let mut rem = num;
    let mut sum = 0;
    while rem > 0 {
        let digit = rem % 10;
        sum += digit.pow(5);
        rem = rem / 10;
    }
    return sum;
}

fn sum_of_fifth_powers() -> u32 {
    let mut sum = 0;
    // Stop at some sufficiently large number.
    for i in 2..1_000_000 {
        let digit_sum = sum_of_digit_powers(i);
        if i == digit_sum {
            sum += i;
        }
    }
    return sum;
}

fn main() {
    println!("Sum of 5th powers is {}", sum_of_fifth_powers());
}
