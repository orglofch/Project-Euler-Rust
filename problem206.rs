// DONE: 1389019170
//
// There are more possibility when iterating over all numbers
// that could fill in the blank (9^9) than there are numbers
// which could be powered and checked to see if they match
// the correct sequence (~300,000,000) so we iterate over the
// second instead.
//
// Since the smallest number in the sequence is a 0 we also
// know that they number we're looking for must start with a 0.
// Which means it's square must have the last 3 digits as 900.
//
// So we know the last two digits of our number is 30 or 70.

const MIN_SQR: u64 = 1010101010;
const MAX_SQR: u64 = 1389026623;

fn correct_sequence(num: u64) -> bool {
    let mut sequence_num = num / 10000;
    for i in 0..8 {
        if sequence_num % 10 != (8 - i) {
            return false;
        }
        sequence_num /= 100;
    }
    return true;
}

fn concealed_square() -> u64 {
    for num in MIN_SQR..MAX_SQR {
        let lower_2 = num % 100;
        if lower_2 != 30 && lower_2 != 70 {
            continue;
        }
        if correct_sequence(num * num) {
            return num;
        }
    }
    return 0;
}

fn main() {
    println!("The concealed square is: {}", concealed_square());
}
