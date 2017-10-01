// DONE: 76576500
//
// We know the formula for triangle numbers
// is 1/2 n * (n + 1) so we can only check those.
//
// The number we want is most likely even since even
// numbers generally have more divisors.


fn divisors(num: u64) -> u32 {
    let mut divisors = 2; // Including 1 and itself.
    for i in 2..(num as f64).sqrt() as u64 {
        if num % i == 0 {
            divisors += 2;
        }
    }

    // Account for the square root case as well.
    if (num as f64).sqrt().fract() == 0.0 {
        divisors += 1;
    }

    return divisors;
}

fn first_500_divisor_triangle() -> u64 {
    let mut i = 1;
    loop {
        let triangle_num = i * (i + 1) / 2;
        if triangle_num % 2 == 1 {
            // If the next number is odd, it's still going to be odd.
            if i % 2 == 1 {
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }

        let divisors = divisors(triangle_num);
        if divisors >= 500 {
            return triangle_num;
        }
        i += 1;
    }
}

fn main() {
    println!("The first triangle number with over 500 divisors is {}", first_500_divisor_triangle());
}
