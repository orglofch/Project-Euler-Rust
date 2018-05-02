// DONE: 2772
//
// This looks like a job for good-old dynamic programming.
//
// Let C(r, c) be the number of rectangles that can be contained within
// a grid with r rows and c columns.
//
// C(r, c) = C(r - 1, c)
//     + C(r, c - 1)
//     - C(r - 1, c - 1) // The overlap
//     + r*c // Rectangles including the new corner.
//
// Note, the "rectangles including the new corner" calculation is just
// the number of distinct non-zero width heights that can be selected
// for the rectangle starting in the new corner.

const TARGET_RECTANGLES: u32 = 2_000_000;

fn grid_area() -> u32 {
    // We know r * c ~= 2,000,000
    let mut cache = [[0_u32; 200]; 200];

    let mut closest_area = 0;
    let mut closest_distance = std::u32::MAX;
    for r in 1..200 {
        for c in 1..200 {
            let area = (r * c) as u32;
            let rectangles = cache[r - 1][c]
                + cache[r][c - 1]
                - cache[r - 1][c - 1]
                + area;

            let distance = (rectangles as i32 - TARGET_RECTANGLES as i32).abs() as u32;

            // Record new closest rectangles.
            if distance < closest_distance {
                closest_distance = distance;
                closest_area = area;
            }

            // If the rectanges get further from here then break.
            if rectangles > TARGET_RECTANGLES {
                break;
            }
            cache[r][c] = rectangles;
        }
    }
    return closest_area;
}

fn main() {
    println!("The area of the max grid is {}", grid_area());
}
