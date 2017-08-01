// DONE: 21124
//
// This is fairly straightforward, the number of different
// special cases is fairly simple.
//
// Note, there's some patterns that we could use but it's
// somewhat unnecessary.

const ONES_PLACES: [&'static str; 9] = ["ONE", "TWO", "THREE", "FOUR", "FIVE", "SIX", "SEVEN",
                                        "EIGHT", "NINE"];

const TEENS: [&'static str; 10] = ["TEN",
                                   "ELEVEN",
                                   "TWELVE",
                                   "THIRTEEN",
                                   "FOURTEEN",
                                   "FIFTEEN",
                                   "SIXTEEN",
                                   "SEVENTEEN",
                                   "EIGHTEEN",
                                   "NINETEEN"];

const TENS_PLACES: [&'static str; 8] = ["TWENTY", "THIRTY", "FORTY", "FIFTY", "SIXTY", "SEVENTY",
                                        "EIGHTY", "NINETY"];

fn stringify(num: u32) -> String {
    if num < 10 {
        return ONES_PLACES[(num - 1) as usize].to_string();
    }

    if num < 100 {
        if num < 20 {
            return TEENS[(num % 10) as usize].to_string();
        }

        let tens_place = (num / 10) as usize;
        let ones_place = (num % 10) as usize;

        if ones_place == 0 {
            return TENS_PLACES[tens_place - 2].to_string();
        } else {
            return format!("{}{}",
                           TENS_PLACES[tens_place - 2],
                           ONES_PLACES[ones_place - 1]);
        }
    }

    if num < 1000 {
        let hundreds_place = (num / 100) as usize;
        let tens_place = num % 100;

        if tens_place == 0 {
            return format!("{}HUNDRED", ONES_PLACES[hundreds_place - 1]);
        }

        return format!("{}HUNDREDAND{}",
                       ONES_PLACES[hundreds_place - 1],
                       stringify(tens_place));
    }

    return "ONETHOUSAND".to_string();
}

fn number_letter_count() -> u64 {
    let mut count: u64 = 0;
    for i in 1..1001 {
        let as_str = stringify(i);
        count += as_str.len() as u64;
    }
    return count;
}

fn main() {
    println!("The number letter count is {}", number_letter_count());
}
