// DONE: 1533776805
//
// Maybe there's a nice way of defining the equation which generates a number
// which satisfies these 3 conditions. However, I don't know them so lets
// just iterate over all numbers in one of the sequences and and see
// where they match up. Maybe that will reveal the closed form equation
// regardless.

fn is_pentagonal(num: u64) -> bool {
    let quad = ((0.25 + (6 * num) as f64).sqrt() + 0.5) / 3.0;
    if quad.fract() == 0.0 {
        return true;
    }
    return false;
}

fn is_hexagonal(num: u64) -> bool {
    let quad = (((1 + 8 * num) as f64).sqrt() + 1.0) / 4.0;
    if quad.fract() == 0.0 {
        return true;
    }
    return false;
}

fn next_tri_pent_hexa() -> u64 {
    let mut i = 286;
    loop {
        let num = i * (i + 1) / 2;
        if is_pentagonal(num) && is_hexagonal(num) {
            return num;
        }

        i += 1;
    }
}

fn main() {
    println!("Next tri/pent/hexa number is {}", next_tri_pent_hexa());
}
