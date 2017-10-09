// TODO:
//
//

use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::option::Option;

const CIPHER: &'static str = "p059_cipher.txt";

fn load_file() -> Vec<u32> {
  let file = BufReader::new(File::open(CIPHER).unwrap());

  let mut cipher: Vec<u32> = Vec::new();
 
  for line in file.lines() {
    for number in line.unwrap().split(',') {
      cipher.push(number.trim().parse().unwrap());
    }
  }
  return cipher;
}

fn decrypt(cipher: &Vec<u32>, key: [u32; 3]) -> Option<String> {
  let mut text = String::new();  
  for i in 0..cipher.len() {
    let xored = cipher[i] ^ key[i % 3];
    if !((xored >= 65 && xored <= 90) || (xored >= 97 && xored <= 122) || xored == 32) {
      return None;
    }
    text.push(char::from_u32(xored).unwrap());
  }
  return Some(text);
}

fn cipher_min_max(cipher: &Vec<u32>) -> ([u32; 3], [u32; 3]) {
  let mut min = [999; 3];
  let mut max = [0; 3]; 
  for i in 0..cipher.len() {
    if cipher[i] > max[i % 3] {
      max[i % 3] = cipher[i];
    }
    if cipher[i] < min[i % 3] { 
      min[i % 3] = cipher[i];
    }
  }
  return (min, max);
}

fn main() {
  let cipher = load_file();
  let (min, max) = cipher_min_max(&cipher);

  println!("Min: {:?}", min);
  println!("Max: {:?}", max);

  for i in 0..123 {
    for j in 0..123 {
      for k in 0..123 {
        let key = [i, j, k];
        let decrypted = decrypt(&cipher, key);
        if decrypted.is_some() {
          println!("Decrypt using {:?}: {}", key, decrypted.unwrap());
        }
      }
    }
  }
}
