// DONE: 101524
//
// Seems like this should be solvable via a Monte Carlo method
// where we take random actions and calculate the visiting pattern at the end,
// Each step we'll just simulate a turn and record where we end up.
//
// Given enough steps the solution should converge to an approximation of
// the real solution.

extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

const GO: u32 = 0;
const JAIL: u32 = 10;
const C1: u32 = 11;
const E3: u32 = 24;
const H2: u32 = 39;
const R1: u32 = 5;
const U1: u32 = 12;
const U2: u32 = 28;

#[derive(Clone, Copy, Debug)]
enum CcCard {
    NA, // Generic other card.
    ATG, // Advance to Go.
    GTJ, // Go to Jail.
}

#[derive(Clone, Copy, Debug)]
enum ChanceCard {
    NA, // Generic other card.
    ATG, // Advance to Go.
    GTJ, // Go to Jail.
    GTC1, // Go to C1.
    GTE3, // Go to E3.
    GTH2, // Go to H2.
    GTR1, // Go to R1.
    GTNR, // Go to next railway.
    GTNU, // Go to next utility company.
    GB3S, // Go back 3 squares.
}

/**
 * Roll two consequetive die.
 *
 * @return a tuple of the sum of the rolls and whether the rolls
 *         were a double.
 */
fn roll<R: Rng>(random_die_roll: &Range<u32>, rng: &mut R) -> (u32, bool) {
    let roll1 = random_die_roll.ind_sample(rng);
    let roll2 = random_die_roll.ind_sample(rng);
    return (roll1 + roll2, roll1 == roll2);
}

/** @returns the new player position. */
fn cc_card(cur_pos: u32, card_pos: usize, cc_stack: &mut [CcCard; 16]) -> u32 {
    let card = cc_stack[card_pos];
    return match card {
        CcCard::NA => cur_pos,
        CcCard::ATG => GO,
        CcCard::GTJ => JAIL,
    };
}

/** @returns the new player position and card_position. */
fn ch_card(cur_pos: u32, card_pos: usize, ch_stack: &mut [ChanceCard; 16]) -> u32 {
    let card = ch_stack[card_pos];
    return match card {
        ChanceCard::NA => cur_pos,
        ChanceCard::ATG => GO,
        ChanceCard::GTJ => JAIL,
        ChanceCard::GTC1 => C1,
        ChanceCard::GTE3 => E3,
        ChanceCard::GTH2 => H2,
        ChanceCard::GTR1 => R1,
        ChanceCard::GTNR => match cur_pos {
            7 => 15,
            22 => 25,
            36 => 5,
            _ => panic!("Unhandled chance position"),
        },
        ChanceCard::GTNU => match cur_pos {
            7 | 36 => U1,
            22 => U2,
            _ => panic!("Unhandled chance position"),
        },
        ChanceCard::GB3S => (cur_pos + 40 - 3) % 40,
    }
}

fn run_simulation(iterations: u32) -> [f32; 40] {
    let mut cc_stack: [CcCard; 16] = [
        CcCard::ATG,
        CcCard::GTJ,
        CcCard::NA,
        CcCard::NA,
        CcCard::NA,
        CcCard::NA,
        CcCard::NA,
        CcCard::NA,
        CcCard::NA,
        CcCard::NA,
        CcCard::NA,
        CcCard::NA,
        CcCard::NA,
        CcCard::NA,
        CcCard::NA,
        CcCard::NA,
    ];

    let mut ch_stack: [ChanceCard; 16] = [
        ChanceCard::ATG,
        ChanceCard::GTJ,
        ChanceCard::GTC1,
        ChanceCard::GTE3,
        ChanceCard::GTH2,
        ChanceCard::GTR1,
        ChanceCard::GTNR,
        ChanceCard::GTNR,
        ChanceCard::GTNU,
        ChanceCard::GB3S,
        ChanceCard::NA,
        ChanceCard::NA,
        ChanceCard::NA,
        ChanceCard::NA,
        ChanceCard::NA,
        ChanceCard::NA,
    ];

    let mut rng = rand::thread_rng();
    let random_die_roll = Range::new(1, 5);

    rng.shuffle(&mut cc_stack);
    rng.shuffle(&mut ch_stack);

    let mut position = GO;
    let mut cc_card_pos = 0;
    let mut ch_card_pos = 0;
    let mut consecutive_doubles = 0_u32;

    let mut counts: [u32; 40] = [0; 40];

    for _ in 0..iterations {
        let (sum, was_double) = roll(&random_die_roll, &mut rng);

        consecutive_doubles = if was_double { consecutive_doubles + 1 } else { 0 };

        if consecutive_doubles == 3 {
            position = JAIL;
        } else {
            position = (position + sum) % 40;
        }

        // Perform special actions if the player lands on certain positions.
        match position {
            // Community Chest.
            2 | 17 | 33 => {
                position = cc_card(position, cc_card_pos, &mut cc_stack);
                cc_card_pos = (cc_card_pos + 1) % 12;
            }
            // Chance.
            7 | 22 | 36 => {
                position = ch_card(position, ch_card_pos, &mut ch_stack);
                ch_card_pos = (ch_card_pos + 1) % 16;
            }
            // Go to Jail.
            30 => position = JAIL,
            _ => (),
        };

        counts[position as usize] += 1;
    }

    let mut frequencies: [f32; 40] = [0.0; 40];
    for i in 0..40 {
        frequencies[i] = counts[i] as f32 / iterations as f32;
    }
    return frequencies;
}

fn main() {
    let mut frequencies: [f32; 40] = [0.0; 40];
    for _ in 0..100 {
        let new_frequencies = run_simulation(10_000);
        for i in 0..40 {
            frequencies[i] += new_frequencies[i];
        }
    }

    // Just select the top 3. We could use select-k but the time
    // this uses is not going to be anywhere close to the simulation.
    let mut top_3: [u32; 3] = [0; 3];
    for i in 0..3 {
        let mut max: f32 = -1.0;
        let mut max_j: i32 = -1;
        for j in 0..40 {
            if frequencies[j] > max {
                max = frequencies[j];
                max_j = j as i32;
            }
        }
        frequencies[max_j as usize] = -1.0;
        top_3[i] = max_j as u32;
    }

    println!("{:02}{:02}{:02}", top_3[0], top_3[1], top_3[2]);
}
