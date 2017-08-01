// DONE: 171
//
// Simple as an iterative process. Faster if we group several unchanging
// month details together, but it's fast enough as is.
//
// The month days are: 31, 28/29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31.
// 1900 is divisible by 4 but not by 400 so February has 28 this year.
//
// In 1901 the first day is a Tuesday.

const MONTH_DAYS: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn sundays() -> u32 {
    let mut count = 0;

    let mut day = 0_u64;
    for year in 1901..2001 {
        for month in 0..12 {
            // Is a sunday, relative to a Tuesday (0).
            if day % 7 == 5 {
                count += 1;
            }

            if month == 1 && year % 4 == 0 {
                day += 29;
            } else {
                day += MONTH_DAYS[month];
            }
        }
    }
    return count;
}

fn main() {
    println!("There have been {} sundays on the first of the month", sundays());
}
