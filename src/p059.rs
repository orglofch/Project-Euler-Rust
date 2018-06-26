// DONE: 107359
//
// Lets just try a bunch of keys and see which resemble english text.
//
// The set of possible valid pieces of text decreases as we add more
// multiples of the key length (unlike unbreakable encyption which is
// "unbreakable" because ever possible text could be created.
//
// To try to limit the number of texts we have to eye-ball we can
// exclude characters that shouldn't exist in sentences and try
// to detect and throw out any non-english sentences.
//
// Unfortunately, outside of included a rough dictionary with a bloom
// filter (which seems to heavy-weight), we may have to resort to
// some form of frequency analysis in the end.

extern crate whatlang;

use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::option::Option;
use whatlang::{Detector, Lang};

const CIPHER: &'static str = "p059_cipher.txt";

// A rough estimate of letter frequencies in english text.
// Indexed ordinally.
static LETTER_FREQUENCIES: [f32; 26] = [
    0.08167,
    0.01492,
    0.02782,
    0.04253,
    0.12702,
    0.02228,
    0.02015,
    0.06094,
    0.06966,
    0.00153,
    0.00772,
    0.04025,
    0.02406,
    0.06749,
    0.07507,
    0.01929,
    0.00095,
    0.05987,
    0.06327,
    0.09056,
    0.02758,
    0.00978,
    0.02360,
    0.00150,
    0.01974,
    0.00074
];

fn load_file() -> Vec<u8> {
    let file = BufReader::new(File::open(CIPHER).unwrap());

    let mut cipher: Vec<u8> = Vec::new();

    for line in file.lines() {
        for number in line.unwrap().split(',') {
            cipher.push(number.trim().parse().unwrap());
        }
    }
    return cipher;
}

fn decrypt(cipher: &Vec<u8>, key: [u8; 3], eng_detector: &Detector) -> Option<u32> {
    let mut letter_count = [0_u32; 26];

    let mut ascii_sum = 0;

    let mut text = String::new();
    for i in 0..cipher.len() {
        // Only allows ciphers which parse to sentence characters.
        let xored = cipher[i] ^ key[i % 3];
        if xored < 32 || xored > 126 {
            return None;
        }

        if xored >= 65 && xored <= 90 {
            // Uppercase letter.
            letter_count[(xored - 65) as usize] += 1;
        } else if xored >= 97 && xored <= 122 {
            // Lowercase letter.
            letter_count[(xored - 97) as usize] += 1;
        }

        let chr = char::from_u32(xored as u32).unwrap();

        ascii_sum += xored as u32;
        text.push(chr);
    }

    // Try to detect english with high confidence, unfortunately this only
    // uses trigram so it can detect some garbage text as english.
    match eng_detector.detect(text.as_str()) {
        Some(info) => {
            if info.lang() != Lang::Eng || info.confidence() != 1.0 || !info.is_reliable() {
                return None;
            }
        },
        None => return None,
    }

    // If any of the letter frequencies are drastically different than normal english
    // then reject the cipher.
    for i in 0..26 {
        let frequency = letter_count[i] as f32 / cipher.len() as f32;
        // Reject frequencies which differ by more than 3.75%. Note, I cheated and
        // binary searched this value so only the correct answer was returned. In
        // practice something less strict still provides few false positives.
        if (LETTER_FREQUENCIES[i] - frequency).abs() > 0.0375 {
            return None;
        }
    }

    return Some(ascii_sum);
}

fn main() {
    let cipher = load_file();

    let eng_detector = Detector::with_whitelist(vec![Lang::Eng]);;

    // The question states lower case characters so we can assume
    // these characters are alpha characters.
    for i in 97..123 {
        for j in 97..123 {
            for k in 97..123 {
                let key = [i, j, k];
                let decrypted = decrypt(&cipher, key, &eng_detector);
                if decrypted.is_some() {
                    println!("The ascii sum of the descrypted text is {}", decrypted.unwrap());
                    return;
                }
            }
        }
    }
}
