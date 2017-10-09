// DONE: 376
//
// Just read the hands into memory and check.
// We can check things from highest to lowest.
// and we only need to check the seconds players hand
// if it's worse than the first players.

use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::option::Option;

const FILE: &'static str = "p054_poker.txt";

#[derive(Copy, Clone, Debug)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, Debug)]
enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Copy, Clone, Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum HandRank {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

impl Card {
    fn from(card_str: String) -> Card {
        let mut chars = card_str.chars();
        let rank = match chars.nth(0).unwrap() {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            'J' => Rank::Jack,
            'Q' => Rank::Queen,
            'K' => Rank::King,
            'A' => Rank::Ace,
            other => panic!("Unhandled suit {}", other),
        };
        let suit = match chars.nth(0).unwrap() {
            'S' => Suit::Spades,
            'H' => Suit::Hearts,
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            other => panic!("Unhandled suit {}", other),
        };
        Card { rank: rank, suit: suit }
    }
}

#[derive(Debug)]
struct Hand(Card, Card, Card, Card, Card);

impl Hand {
    fn score(&self) -> (HandRank, usize) {
        let mut suit_counter: [u8; 4] = [0; 4];
        suit_counter[self.0.suit as usize] += 1;
        suit_counter[self.1.suit as usize] += 1;
        suit_counter[self.2.suit as usize] += 1;
        suit_counter[self.3.suit as usize] += 1;
        suit_counter[self.4.suit as usize] += 1;

        let mut all_same_suit = false;
        for i in 0..4 {
            if suit_counter[i] == 5 {
                all_same_suit = true;
                break;
            }
        }

        let mut rank_counter: [u8; 13] = [0; 13];
        rank_counter[self.0.rank as usize] += 1;
        rank_counter[self.1.rank as usize] += 1;
        rank_counter[self.2.rank as usize] += 1;
        rank_counter[self.3.rank as usize] += 1;
        rank_counter[self.4.rank as usize] += 1;

        // Royal flush.
        if all_same_suit
            && rank_counter[Rank::Ten as usize] == 1
            && rank_counter[Rank::Jack as usize] == 1
            && rank_counter[Rank::Queen as usize] == 1
            && rank_counter[Rank::King as usize] == 1
            && rank_counter[Rank::Ace as usize] == 1 {
                return (HandRank::RoyalFlush, 12);
            }

        let mut is_straight = false;
        let mut running = 0;
        let mut straight_end = 0;
        for i in 0..13 {
            if rank_counter[i] == 1 {
                if running == 4 {
                    is_straight = true;
                    straight_end = i;
                    break;
                }
                running += 1;
            } else if rank_counter[i] == 0 {
                running = 0;
            } else {
                break;
            }
        }

        // Straight flush.
        if is_straight && all_same_suit {
            return (HandRank::StraightFlush, straight_end);
        }

        // Four of a kind.
        for i in 0..13 {
            if rank_counter[i] == 4 {
                return (HandRank::FourOfAKind, i);
            }
        }

        let mut three_of_a_kind: Option<usize> = None;
        for i in 0..13 {
            if rank_counter[i] == 3 {
                three_of_a_kind = Some(i);
                break;
            }
        }

        let mut pair: Option<usize> = None;
        for i in 0..13 {
            if rank_counter[i] == 2 {
                pair = Some(i);
                break;
            }
        }

        // Full house.
        if three_of_a_kind.is_some() && pair.is_some() {
            return (HandRank::FullHouse, cmp::max(three_of_a_kind.unwrap(), pair.unwrap()));
        }

        // Flush.
        if all_same_suit {
            for i in (0..13).rev() {
                if rank_counter[i] > 0 {
                    return (HandRank::Flush, i);
                }
            }
        }

        // Straight.
        if is_straight {
            return (HandRank::Straight, straight_end);
        }

        // Three of a kind.
        if three_of_a_kind.is_some() {
            return (HandRank::ThreeOfAKind, three_of_a_kind.unwrap());
        }

        // Two pairs.
        if pair.is_some() {
            for i in 0..13 {
                if i == pair.unwrap() as u8 {
                    continue;
                }
                if rank_counter[i as usize] == 2 {
                    return (HandRank::TwoPair, cmp::max(pair.unwrap(), i as usize));
                }
            }
        }

        // One pair.
        if pair.is_some() {
            return (HandRank::Pair, pair.unwrap());
        }

        // High Card.
        for i in (0..13).rev() {
            if rank_counter[i] > 0 {
                return (HandRank::HighCard, i);
            }
        }
        panic!("No hand rank found");
    }
}

fn wins() -> u32 {
    let file = BufReader::new(File::open(FILE).unwrap());

    let mut total = 0;
    for line in file.lines() {
        let mut cards: Vec<Card> = Vec::new();
        for card_str in line.unwrap().split(" ") {
            cards.push(Card::from(card_str.to_string()));
        }

        let p2hand = Hand(cards.pop().unwrap(),
                          cards.pop().unwrap(),
                          cards.pop().unwrap(),
                          cards.pop().unwrap(),
                          cards.pop().unwrap());
        let p1hand = Hand(cards.pop().unwrap(),
                          cards.pop().unwrap(),
                          cards.pop().unwrap(),
                          cards.pop().unwrap(),
                          cards.pop().unwrap());

        let (p1score, p1max) = p1hand.score();
        let (p2score, p2max) = p2hand.score();

        if p1score as u8 > p2score as u8 {
            total += 1;
        } else if p1score as u8 == p2score as u8 && p1max > p2max {
            total += 1;
        }
    }
    return total;
}

fn main() {
    println!("Player 1 won {} hands", wins());
}
