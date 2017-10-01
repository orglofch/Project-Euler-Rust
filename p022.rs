// DONE: 871198282
//
// This problem is fairly simple. We should be able to process 46k elements
// without too much trouble. However, we can be smarter and use a trie which
// implicitely provices the sort-order (in less time than a regular sort) and
// also uses less memory since common overlaps between name prefixes use the
// same memory.
//
// In order to actually save memory we don't store anything at the leaves of
// the trie, we just need to traverse in the right order to maintain a count.
// and know when something is a leaf.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::option::Option;

const FILE: &'static str = "p022_names.txt";

struct Trie {
    is_leaf: bool,
    elements: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Trie {
        Trie {
            is_leaf: false,
            elements: Default::default(),
        }
    }

    fn insert(&mut self, chars: &[u8]) {
        if chars.len() == 0 {
            self.is_leaf = true;
            return;
        }

        let element = (chars[0] - 65) as usize;
        match self.elements[element] {
            Some(ref mut trie) => trie.insert(&chars[1..]),
            None => {
                self.elements[element] = {
                    let mut trie = Trie::new();
                    trie.insert(&chars[1..]);
                    Some(Box::new(trie))
                }
            }
        }
    }

    fn score(&self, prefix_score: u64, last_order: u64) -> (u64, u64) {
        let mut total: u64 = 0;
        let mut order = last_order;
        if self.is_leaf {
            total += prefix_score * order;
            order += 1;
        }

        for i in 0..self.elements.len() {
            match self.elements[i] {
                Some(ref trie) => {
                    let (value, count) = trie.score(prefix_score + i as u64 + 1, order);
                    total += value;
                    order = count;
                }
                None => (),
            }
        }
        return (total, order);
    }
}

fn read_names_as_trie() -> Box<Trie> {
    let file = BufReader::new(File::open(FILE).unwrap());

    let mut trie = Trie::new();

    for line in file.lines() {
        for name in line.unwrap().split(",") {
            let bytes = name.as_bytes();
            // Exclude the quotes.
            trie.insert(&bytes[1..bytes.len() - 1]);
        }
    }
    return Box::new(trie);
}

fn name_scores() -> u64 {
    let trie = read_names_as_trie();
    return trie.score(0, 1).0;
}

fn main() {
    println!("The total name score is {}", name_scores());
}
