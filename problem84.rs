// TODO
//
// Seems like this should be solvable via a Monte Carlo method
// where we take random actions and calculate the visiting pattern at the end,
// Each step we'll just simulate a turn and record where we end up.
//
// Given enough steps the solution should converge to an approximation of
// the real solution.
//
// TODO(orglofch): We might be able to early exit when the probagilities
// stop fluctuating beyond some threshold.

const GO: u32 = 0;
const JAIL: u32 = 10;
const C1: u32 = 11;
const E3: u32 = 24;
const H2: u32 = 39;

enum SpecialAction {
  None,
  GTJ,  // Go to Jail.
  ATG,  // Advance to GO.
  GTC1, // Go to C1. 
  GTE3, // Go to E3.
  GTH2, // Go to H2.
}

/**
 * Roll two consequetive die.
 *
 * @return a tuple of the sum of the rolls and whether the rolls
 *         were a double.
 */
fn roll() -> (u32, bool) {
  (0, true) 
}

fn select_cc_card() -> SpecialAction {
  SpecialAction::None
}

fn select_ch_card() -> SpecialAction {
  SpecialAction::None
}

fn run_simulation(iterations: u32, print_iter: u32) {
  let mut position = 0_u32; 
  let mut consequetive_doubles = 0_u32; 

  let mut frequencies: [u32; 40] = [0; 40];

  for i in 0..iterations {
    let (sum, was_double) = roll();

    if was_double {
      consequetive_doubles = consequetive_doubles + 1;
    }

    if consequetive_doubles == 3 {
      position = JAIL;  
    } else {
      position = (position + sum) % 40;
    }
  
    // Perform special actions if the player lands on certain positions.
    let special_action = match position {
      // Community Chest.
      2 | 17 | 33 => select_cc_card(),
      // Chance.
      7 | 22 | 36 => select_ch_card(),
      // Go to Jail.
      30 => SpecialAction::GTJ,
      _ => SpecialAction::None,
    };

    // Perform special action.
    match special_action {
      GTJ => position = JAIL,
      ATG => position = GO,
      GTCL => position = C1,
      GTE3 => position = E3,
      GTH2 => position = H2,
      _ => {},
    }

    // Record the place they ended up.
    frequencies[position] = frequencies[position + 1];

    if i % print_iter == 0 {
      println!("Frequencies: {:?}", frequencies);
    }
  }

  // Print out the final frequencies.
  println!("Frequencies: {:?}", frequencies);
}

fn main() {
  run_simulation(100, 10);
}
