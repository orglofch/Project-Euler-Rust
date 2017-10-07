// TODO
//
// P(n) = n * (3n - 1) / 2
//      = (3n^2 - n) / 2
// So we can check if a number is pentagonal via
// 0 = 3n^2 - n - 2 * P(n)
//
// Via quadratic equation:
// n = (-(-1) +- sqrt((-1)^2 - 4*(3)(-2 * P(n)))) / 2 * 3
// n = (1 +- sqrt(1 + 24 * P(n))) / 6
// n = (1 + sqrt(1 + 24 * P(n))) / 6

fn is_pentagonal(num: u64) -> bool {
    return ((1.0 + (1.0 + 24.0 * (num as f64)).sqrt()) / 6.0).fract() == 0.0;
}

fn pentagonal(i: u64) -> u64 {
    return i * (3 * i - 1) / 2;
}

fn smallest_pentagonal_difference() -> u64 {
    let mut smallest_difference = u64::max_value();

    let mut previous = 0;

    let mut i = 1;
    loop {
        let pent1 = pentagonal(i);
        if pent1 - previous > smallest_difference {
            break;
        }

        for j in (1..i).rev() {
            let pent2 = pentagonal(j);
            let difference = pent1 - pent2;
            if difference >= smallest_difference {
                break;
            }

            if !is_pentagonal(difference) {
                continue;
            }

            let addition = pent1 + pent2;
            if is_pentagonal(addition) {
                smallest_difference = difference;
            }
        }

        previous = pent1;
        i += 1;
    }
    return smallest_difference;
}

fn main() {
    println!(
        "The smallest pentagonal difference is {}",
        smallest_pentagonal_difference()
    );
}
