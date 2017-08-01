// DONE: 872187
//
// This problem is very simple and efficient enough
// that brute forcing it really suffices.
//
// We can basically just convert the numbers to
// strings and do basic string palindroms checks.
//
// However, we can be much much faster if we just
// use the number palindromes less than 1 million
// and just check their binary version.

use std::fmt;

fn is_palindrome(string: String) -> bool {
    let bytes = string.as_bytes();
    let len = bytes.len();

    for i in 0..len / 2 {
        if bytes[i] != bytes[len - i - 1] {
            return false;
        }
    }
    return true;
}

fn is_binary_palindrome(num: u64) -> bool {
    let mut output = String::new();
    fmt::write(&mut output, format_args!("{:b}", num)).unwrap();
    return is_palindrome(output);
}

fn double_base_palindrome_sum() -> u64 {
    let mut sum: u64 = 25;
    let mut sub_palindromes: Vec<String> = vec!(
        "".to_owned(),
        "0".to_owned(),
        "1".to_owned(),
        "2".to_owned(),
        "3".to_owned(),
        "4".to_owned(),
        "5".to_owned(),
        "6".to_owned(),
        "7".to_owned(),
        "8".to_owned(),
        "9".to_owned());
    for _ in 0..3 {
        let mut new: Vec<String> = Vec::new();
        for sub in sub_palindromes.iter() {
            for n in 0..10 {
                let str: String = n.to_string() + &sub + &n.to_string();

                if n != 0 {
                    let as_num = str.parse::<u64>().unwrap();
                    if as_num > 1_000_000 {
                        continue;
                    }
                    if is_binary_palindrome(as_num) {
                        sum += as_num;
                    }
                }
                new.push(str);
            }
        }
        sub_palindromes = new;
    }
    return sum;
}

fn main() {
    println!("Double base palindrome sum {}", double_base_palindrome_sum());
}
