// DONE: 840
//
// Basically solve for the p such that
// 1) a + b + c = p => = c^2 = (p - a - b)^2
// 2) a^2 + b^2 = c^2
// has the most solutions.
//
// 2) -> 1) => (p - a - b)^2 = a^2 + b^2
//          => p^2 - 2pa - 2pb + a^2 + 2ab + b^2 = a^2 + b^2
//          => p^2 - 2pa - 2pb + 2ab = 0
//          => p^2 - 2pa - 2pb + 2ab = 0
//
// Which has solutions p - b != 0, a = p(p - 2b) / 2(p - b)
//
// Which should give us a relative small amount of p's to check.

fn perimeter_with_most_right_triangles() -> u32 {
    let mut max_triangles_perimeter = 0;
    let mut max_valid_triangles = 0;
    for p in 1..1001 {
        let mut valid_triangles = 0;

        // We only need to check a up to p / 4, otherwise we start
        // checking symmetric cases.
        for b in 1..(p / 4 + 1) {
            let numer = (p * (p - 2 * b)) as f64;
            let denom = (2 * (p - b)) as f64;
            let a = numer / denom;

            if a.fract() == 0.0 {
                valid_triangles += 1;
            }
        }
        if valid_triangles > max_valid_triangles {
            max_valid_triangles = valid_triangles;
            max_triangles_perimeter = p;
        }
    }
    return max_triangles_perimeter;
}

fn main() {
    println!("The perimeter {} has the most right triangle solutions", perimeter_with_most_right_triangles());
}
