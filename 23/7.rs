use std::io::{self, BufRead};
use std::collections::HashMap;

const CARD_ORDER: &str = "23456789TJQKA";
const CARD_ORDER_JOKERY: &str = "J23456789TQKA";

#[derive(Clone, Debug, PartialEq, Eq)]
struct Hand {
    cards: [u8; 5],
    cards_jokery: [u8; 5],
    _debug: String,
}

// some way to pattern match this would be more readable though
fn strength(hand: &Hand) -> i32 {
    let map = hand.cards.iter().copied().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1; acc
    });
    let max = map.iter().map(|(_, count)| *count).max().unwrap();
    match map.len() {
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
    }
}

/*
 * one joker: five of a kind or four of a kind or full house or three of a kind or one pair
 *   AAAA -> AAAAA
 *   AAAB -> AAAAB
 *   AABB -> AAABB
 *   AABC -> AABBC
 *   ABCD -> AABCD or ABBCD or ABCCD or ABCDD
 * dual joker, five of a kind or four of a kind or three of a kind:
 *   AAA -> AAAAA
 *   AAB -> AAAAB
 *   ABC -> AAABC
 *   ABBBC or ABCCC
 * triple joker, five of a kind or four of a kind:
 *   AA -> AAAAA
 *   AB -> AAAAB or ABBBB
 * quad joker, always five of a kind:
 *   A -> AAAAA
 * and all jokers become five of a kind.
 */
fn strength_jokery(hand: &Hand) -> i32 {
    let mut map = hand.cards_jokery.iter().copied().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1; acc
    });
    let n_jokers = map.remove(&0).unwrap_or(0);
    let rehand = |card_map: HashMap<_, _>, jokered: u8| {
        let mut next = [jokered; 5];
        let mut nexti = next.iter_mut();
        for (c, n) in card_map {
            for _ in 0..n {
                *nexti.next().unwrap() = c;
            }
        }
        let mut h = hand.clone();
        h.cards = next;
        h
    };
    if n_jokers == 5 {
        // trivial, all jokers, no other cards to look at
        return 6;
    }
    let highest = map.iter().map(|(&c, &count)| (count, c)).max().unwrap().1;
    strength(&rehand(map, highest))
}

fn total_winnings(mut games: Vec<(Hand, i32)>) -> i32 {
    games.sort_by(|(a, _), (b, _)| {
        strength(a).cmp(&strength(b)).then(a.cards.cmp(&b.cards))
    });
    games.iter().enumerate().map(|(i, g)| (1 + i as i32) * g.1).sum()
}

fn total_winnings_jokery(mut games: Vec<(Hand, i32)>) -> i32 {
    games.sort_by(|(a, _), (b, _)| {
        strength_jokery(a).cmp(&strength_jokery(b)).then(a.cards_jokery.cmp(&b.cards_jokery))
    });
    games.iter().enumerate().map(|(i, g)| (1 + i as i32) * g.1).sum()
}

fn parse_bid(line: &str) -> (Hand, i32) {
    let mut sp = line.split(' ');
    let mut hand = Hand { cards: [0; 5], cards_jokery: [0; 5], _debug: line.to_owned() };
    let values = sp.clone().next()
        .unwrap()
        .chars()
        .map(|c| CARD_ORDER.chars().position(|x| x == c).unwrap() as u8);
    let values_jokery = sp.next()
        .unwrap()
        .chars()
        .map(|c| CARD_ORDER_JOKERY.chars().position(|x| x == c).unwrap() as u8);
    hand.cards.iter_mut().zip(values).for_each(|(hand, inp)| *hand = inp);
    hand.cards_jokery.iter_mut().zip(values_jokery).for_each(|(hand, inp)| *hand = inp);
    let bid = sp.next().unwrap().parse().unwrap();

    (hand, bid)
}

fn main() {
    let games: Vec<(Hand, i32)> = io::stdin().lock().lines()
        .map(|line| parse_bid(&line.unwrap()))
        .collect();
    println!("{}", total_winnings(games.clone()));
    println!("{}", total_winnings_jokery(games));
}
