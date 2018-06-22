// DONE: 210
//
// There are:
// 10 one digit numbers covering the range of 10 digits.
// 90 two digit numbers cocering the range of 180 digits.
// 900 three digit numbers covering the range of 2700 digits.
// 9000 four digit numbers covering the range of 36000 digits.
// ...
//
// We can just check if the digit we're looking for falls into
// each range and extract the relvant number it corresponds to
// and the digit within that number to find our result.

/// Return the nth digit in the number n.
fn nth_digit(num: u32, n: u32) -> u32 {
    let digits = (num as f32).log(10.0).floor() as u32;

    assert!(n <= digits + 1);

    let mut remainder = num;
    for _ in 0..(digits - n) {
        remainder = remainder / 10;
    }
    return remainder % 10;
}

fn champ_digit(digit: u32) -> u32 {
    let mut d = digit;

    // Special case the 1 digit numbers.
    if d < 10 {
        return d;
    }
    d -= 10;

    let mut start_number = 10;
    let mut digits = 2;
    let mut numbers = 90;

    loop {
        let range = digits * numbers;

        if d < range {
            let number_index = d / digits;
            let number_digit = d % digits;

            let number = start_number + number_index;

            return nth_digit(number, number_digit);
        }
        d -= range;

        start_number += numbers;
        digits += 1;
        numbers *= 10;
    }
}

fn champernownes_constant_sum() -> u32 {
    return champ_digit(1)
        * champ_digit(10)
        * champ_digit(100)
        * champ_digit(1000)
        * champ_digit(10000)
        * champ_digit(100000);
}

fn main() {
    println!("d_1 × d_10 × d_100 × d_1000 × d_10000 × d_100000 × d_1000000 = {}", champernownes_constant_sum());
}
