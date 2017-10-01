// DONE: 932718654
//
// Iterating over the set of all pandigital numbers is easier
// than iterating over the set of all numbers that might
// generate a pandigital number. The number of pandigital
// numbers is 8! since we can assume it starts with a 9
// since the question gave us a number starting with a 9.
//
// For each pandigital number we can check if it follows
// the multiple pattern. Since the first number needs to be 1
// we only need to check prefixes to make sure they follow the
// correct pattern.

fn follows_pattern(num: u64) -> bool {
    let as_str = num.to_string();

    let mut chars = as_str.chars();

    let mut prefix = String::new();

    // Larger prefixes necessarily can't create a 9 digit number.
    for _ in 0..(as_str.len() / 2) {
        prefix.push(chars.next().unwrap());

        let prefix_num = prefix.parse::<u32>().unwrap();

        let mut block_num = 2;
        let mut expected_next = (prefix_num * block_num).to_string();
        let mut slice = &as_str[prefix.len()..as_str.len()];

        while slice.starts_with(&expected_next) {
            block_num += 1;
            expected_next = (prefix_num * block_num).to_string();
            slice = &slice[expected_next.len()..slice.len()];
        }
        if slice.is_empty() {
            return true;
        }
    }
    return false;
}

fn max_iterate_pandigitals(val: u64, choices: Vec<u8>) -> Option<u64> {
    if choices.len() == 0 {
        if follows_pattern(val) {
            return Some(val);
        }
        return None;
    }

    // Iterate high to low ensures we find the largest number first.
    let base = val * 10;
    for i in 0..choices.len() {
        let mut remaining = choices.clone();
        let choice = remaining.swap_remove(i);
        let sub_result = max_iterate_pandigitals(base + choice as u64, remaining);
        if sub_result.is_some() {
            return sub_result;
        }
    }
    return None;
}

fn largest_pandigital_multiple() -> u64 {
    return max_iterate_pandigitals(9, vec!(8, 7, 6, 5, 4, 3, 2, 1)).unwrap();
}

fn main() {
    println!("Largest pandigital multiple number {}", largest_pandigital_multiple());
}
