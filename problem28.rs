// DONE: 669171001
//
// This problem is fairly simple, so much so that's it's not really
// even worth our time to optimize.
//
// The pattern for diagonals is:
// 1, 3, 5, 7, 9, 13, 17, 21, 25 ...
// We see that each "ring" consists of 4 diagonals which are each n
// apart (e.g. 3 + 2 = 5, 5 + 2 = 7, 7 + 2 = 9)
// Which can be written as
//   (1 + 2) + (1 + 2 + 2) + (1 + 2 + 2 + 2) + (1 + 2 + 2 + 2 + 2) for ring 1
//   Which is 4 * 1 + 10 * 2
//
//   (9 + 4) + (9 + 4 + 4) + (9 + 4 + 4 + 4) + (9 + 4 + 4 + 4 + 4) for ring 2
//   Which is 4 * 9 + 10 * 4
//
// For a generic ring this can be written as:
//   prev * 4 + 10 * ring * 2
//
// There are 500 rings in the 1001 x 1001 rectangle.
//
// Obviously this could be simplied further and likely has a closed form
// solution.

const RECTANGLE_DIMEN: u64 = 1001;

fn spiral_sum() -> u64 {
    let rings: u64 = (RECTANGLE_DIMEN - 1) / 2;
    let mut sum = 1;
    let mut prev = 1;
    for ring in 1..(rings + 1) {
        let inc = ring * 2;
        sum += prev * 4 + 10 * inc;
        prev = prev + 4 * inc;
    }
    return sum;
}

fn main() {
    println!("The spiral diagonal sum is {}", spiral_sum());
}
