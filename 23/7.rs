use std::io::{self, BufRead};
use std::collections::HashMap;

const CARD_ORDER: &str = "23456789TJQKA";

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    _debug: String,
}

// some way to pattern match this would be more readable though
fn strength(hand: &Hand) -> i32 {
    let map = hand.cards.iter().copied().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1; acc
    });
    let max = map.iter().map(|(_, count)| *count).max().unwrap();
    let r = match map.len() {
        // five of a kind: XXXXX
        1 => 6,
        // four of a kind: XYYYY / XXXXY
        2 if max == 4 => 5,
        // full house: XXYYY / XXXYY
        2 if max == 3 => 4,
        // three of a kind: XXXYZ etc
        3 if max == 3 => 3,
        // two pair: XXYYZ
        3 if max == 2 => 2,
        // one pair: XXYZW
        4 => 1,
        // high card
        5 => 0,
        _ => panic!()
    };
    if false { println!("strength of {:?} is {}", hand, r); }
    r
}

impl Ord for Hand {
    fn cmp(&self, rhs: &Hand) -> std::cmp::Ordering {
        strength(self).cmp(&strength(rhs)).then(self.cards.cmp(&rhs.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, rhs: &Hand) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

fn total_winnings(mut games: Vec<(Hand, i32)>) -> i32 {
    games.sort();
    if false { println!("{:#?}", games); }
    games.iter().enumerate().map(|(i, g)| (1 + i as i32) * g.1).sum()
}

fn parse_bid(line: &str) -> (Hand, i32) {
    let mut sp = line.split(' ');
    let mut hand = Hand { cards: [0; 5], _debug: line.to_owned() };
    let values = sp.next()
        .unwrap()
        .chars()
        .map(|c| CARD_ORDER.chars().position(|x| x == c).unwrap() as u8);
    hand.cards.iter_mut().zip(values).for_each(|(hand, inp)| *hand = inp);
    let bid = sp.next().unwrap().parse().unwrap();

    (hand, bid)
}

fn main() {
    let games: Vec<(Hand, i32)> = io::stdin().lock().lines()
        .map(|line| parse_bid(&line.unwrap()))
        .collect();
    println!("{}", total_winnings(games));
}
