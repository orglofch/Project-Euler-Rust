// TODO
//
// This seems like a pretty straightforward constraint satisfactions problem.
//
// We know that d4 is an even number since d2d3d4 needs to be divisible by 2.
//
// We know that d6 is a 0 or a 5 since d4d5d6 needs to be divisible by 5.
//
// The difference between d7 and d8 = d6.
//
// There are probably more constraints but this should suffice. We can
// also iterate over a constraint since that simplifies things considerably.
// Might as will iterate over the hardest constraint at well.

fn substring_pandigital_sum() -> u64 {
    let mut d8d9d10 = 17;

    while d8d9d10 < 999 {
        let d8 = d8d9d10 / 100;
        let d9 = d8d9d10 % 100 / 10;
        let d10 = d8d9d10 % 10;

        // Might be pandigital.
        if d8 == d9 || d8 == d10 || d9 == d10 {
            continue;
        }

        let mut available: [bool; 10] = [true; 10];
        available[d8] = false;
        available[d9] = false;
        available[d10] = false;

        // d6 is next most constrained.
        if available[0] == false && available[5] = false {
            continue;
        }

        d8d9d10 += 17;
    }
}

fn main() {
    println!("The substring pandigital sum is {}", substring_pandigital_sum());
}
