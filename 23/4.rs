use std::io::{self, BufRead};
use std::collections::HashSet;

struct Card {
    winning: HashSet<u32>,
    hand: HashSet<u32>,
}

fn parse_list(s: &str) -> HashSet<u32> {
    // filter_map because "" isn't a number, so skip them
    s.split(' ').filter_map(|n| n.parse::<u32>().ok()).collect()
}

fn parse_card(card_spec: &str) -> Card {
    let mut sp = card_spec.split(" | ");
    let left = sp.next().unwrap();
    let hand_str = sp.next().unwrap();
    let mut sp = left.split(": ");
    let winning_str = sp.nth(1).unwrap();
    let winning = parse_list(winning_str);
    let hand = parse_list(hand_str);

    Card { winning, hand }
}

fn points(card: &Card) -> u32 {
    let n = card.hand.intersection(&card.winning).count();

    if n == 0 { 0 } else { 1 << (n - 1) }
}

fn won_cards(cards: &[Card], play: usize) -> u32 {
    let card = &cards[play];
    let n = card.hand.intersection(&card.winning).count();

    1 + (1..=n).map(|i| won_cards(cards, play + i)).sum::<u32>()
}

fn game(cards: &[Card]) -> u32 {
    (0..cards.len()).map(|i| won_cards(cards, i)).sum::<u32>()
}

fn main() {
    let cards: Vec<Card> = io::stdin().lock().lines()
        .map(|line| parse_card(&line.unwrap()))
        .collect();
    println!("{}", cards.iter().map(points).sum::<u32>());
    println!("{}", game(&cards));
}
