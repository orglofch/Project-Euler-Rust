// DONE: 100
//
// Super simple just iterate the numbers of options
// add them, and fine the GCD.
//
// Notably, 0 can never be added since it results
// in either numbers < 10 or the trivial excluded examples.

#[derive(Debug)]
struct Fract {
    num: u64,
    den: u64,
}

fn gcd(num: u64, den: u64) -> u64 {
    let mut a = num;
    let mut b = den;
    while b != 0 {
        let t = b;
        b = num % b;
        a = t;
    }
    return a;
}

fn digit_cancelling_fraction_product() -> u64 {
    let mut fractions: Vec<Fract> = Vec::new();
    for num in 1..9 {
        for den in 1..10 {
            let frac = num as f32 / den as f32;
            //println!("Frac {}", frac);
            for i in 1..10 {
                // Both behind.
                if num < den {
                    let new_num = num * 10 + i;
                    let new_den = den * 10 + i;
                    let new_frac = new_num as f32 / new_den as f32;
                    if new_frac == frac {
                        fractions.push(Fract { num: new_num, den: new_den });
                    }
                }
                // Both infront.
                if num < den {
                    let new_num = num + i * 10;
                    let new_den = den + i * 10;
                    let new_frac = new_num as f32 / new_den as f32;
                    if new_frac == frac {
                        fractions.push(Fract { num: new_num, den: new_den });
                    }
                }

                // Infront of first, behind second.
                if i <= den && i != num {
                    let new_num = num + i * 10;
                    let new_den = den * 10 + i;
                    let new_frac = new_num as f32 / new_den as f32;
                    if new_frac == frac {
                        fractions.push(Fract { num: new_num, den: new_den });
                    }
                }

                // Behind first, infront of second.
                if i > num && i != den {
                    let new_num = num * 10 + i;
                    let new_den = den + i * 10;
                    let new_frac = new_num as f32 / new_den as f32;
                    if new_frac == frac {
                        fractions.push(Fract { num: new_num, den: new_den });
                    }
                }
            }
        }
    }
    let mut num_prod = 1;
    let mut den_prod = 1;
    for i in 0..fractions.len() {
        num_prod *= fractions[i].num;
        den_prod *= fractions[i].den;
    }
    return den_prod / gcd(num_prod, den_prod);
}

fn main () {
    println!("Digit cancelling fraction product {}", digit_cancelling_fraction_product());
}
