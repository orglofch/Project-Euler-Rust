// DONE: 13938
//
// The travelling salesman problem can be reinterpreted as this problem
// where (r, c) selection define edges in the path. Therefore, the problem
// is NP-Complete.
//
// A common solution to NP-Complete problems in simulated annealing so
// we'll use that. Note, that this converges but it still technically an
// approximation.
//
// An alternate would be to formulate this problem as a graph search problem
// and use A* search.

extern crate rand;

use rand::Rng;

const MATRIX: [[u32; 15]; 15] =
    [[7, 53, 183, 439, 863, 497, 383, 563, 79, 973, 287, 63, 343, 169, 583],
     [627, 343, 773, 959, 943, 767, 473, 103, 699, 303, 957, 703, 583, 639, 913],
     [447, 283, 463, 29, 23, 487, 463, 993, 119, 883, 327, 493, 423, 159, 743],
     [217, 623, 3, 399, 853, 407, 103, 983, 89, 463, 290, 516, 212, 462, 350],
     [960, 376, 682, 962, 300, 780, 486, 502, 912, 800, 250, 346, 172, 812, 350],
     [870, 456, 192, 162, 593, 473, 915, 45, 989, 873, 823, 965, 425, 329, 803],
     [973, 965, 905, 919, 133, 673, 665, 235, 509, 613, 673, 815, 165, 992, 326],
     [322, 148, 972, 962, 286, 255, 941, 541, 265, 323, 925, 281, 601, 95, 973],
     [445, 721, 11, 525, 473, 65, 511, 164, 138, 672, 18, 428, 154, 448, 848],
     [414, 456, 310, 312, 798, 104, 566, 520, 302, 248, 694, 976, 430, 392, 198],
     [184, 829, 373, 181, 631, 101, 969, 613, 840, 740, 778, 458, 284, 760, 390],
     [821, 461, 843, 513, 17, 901, 711, 993, 293, 157, 274, 94, 192, 156, 574],
     [34, 124, 4, 878, 450, 476, 712, 914, 838, 669, 875, 299, 823, 329, 699],
     [815, 559, 813, 459, 522, 788, 168, 586, 966, 232, 308, 833, 251, 631, 107],
     [813, 883, 451, 509, 615, 77, 281, 613, 459, 205, 380, 274, 302, 35, 805]];

fn boltzmann(current_cost: u32, new_cost: u32, temperature: f32) -> f32 {
    let diff: f32 = (new_cost as i32 - current_cost as i32) as f32;
    return (diff / temperature).exp();
}

fn anneal(start_temperature: f32, entropy: f32) -> u32 {
    let mut temperature = start_temperature;

    // Start on the diagonal.
    let mut sum = 0;
    // Swapping columns is essentially swapping rows so just keep track of rows.
    let mut row_to_column: [u8; 15] = [0; 15];
    for i in 0..15 {
        sum += MATRIX[i][i];
        row_to_column[i] = i as u8;
    }

    while temperature > 0.00001 {
        // Choose two rows to swap.
        let row1 = rand::thread_rng().gen_range(0, 15) as usize;
        let row2 = rand::thread_rng().gen_range(0, 15) as usize;
        let col1 = row_to_column[row1] as usize;
        let col2 = row_to_column[row2] as usize;

        let new_sum = sum
            - MATRIX[row1][col1]
            - MATRIX[row2][col2]
            + MATRIX[row1][col2]
            + MATRIX[row2][col1];

        let prob = rand::thread_rng().gen_range(0.0, 1.0);
        if prob < boltzmann(sum, new_sum, temperature) {
            sum = new_sum;
            row_to_column[row1] = col2 as u8;
            row_to_column[row2] = col1 as u8;
        }

        temperature *= entropy;
    }
    return sum;
}

fn main() {
    println!("The max matrix sum is {}", anneal(10000.0, 0.99999));
}
