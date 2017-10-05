// DONE: 161667
//
// We can calculate the set of all pythagorean triples
// accumulate them by their total circumference and
// count the total buckets with one entry.
//
// We know n < m since a = m^2 - n^2 and a > 0
//
// a + b + c = m^2 - n^2 + 2mn + m^2 + n^2
//           = 2m^2 + 2mn
//           = 2(m^2 + mn)
// 2(m^2 + mn) < 1,500,000
//         m^2 < 750,000 - mn
//         m^2 < 750,000
//           m < 750,000^(1/2)

use std::collections::HashMap;

fn gcd(num: u32, den: u32) -> u32 {
    let mut a = num;
    let mut b = den;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn single_integer_right_triangles() -> u32 {
    let mut triangles_by_length: HashMap<u32, u32> = HashMap::new();

    for m in 2..868 {
        for n in 1..m {
            // Ensure they're not both odd and are coprime.
            if (m + n) % 2 != 1 || gcd(n, m) != 1 {
                continue;
            }

            let length = 2 * m * (m + n);

            let mut accum_length = length;
            while accum_length <= 1_500_000 {
                *triangles_by_length.entry(accum_length).or_insert(0) += 1;
                accum_length += length;
            }
        }
    }

    return triangles_by_length.values().filter(|&&n| n == 1).count() as u32;
}

fn main() {
    println!(
        "There are {} single integer right triangles",
        single_integer_right_triangles()
    );
}
