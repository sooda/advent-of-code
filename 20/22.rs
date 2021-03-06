use std::io::{self, BufRead};
use std::collections::HashSet;

fn deck_score(deck: &[u32]) -> u32 {
    deck.iter().rev().enumerate().map(|(multiplier, card)| {
        (multiplier as u32 + 1) * card
    }).sum()
}

fn winning_score(mut deck0: Vec<u32>, mut deck1: Vec<u32>) -> u32 {
    for _round in 1.. {
        let card0 = deck0.remove(0);
        let card1 = deck1.remove(0);
        assert!(card0 != card1);
        let (windeck, wincard, losecard) = if card0 > card1 {
            (&mut deck0, card0, card1)
        } else {
            (&mut deck1, card1, card0)
        };
        windeck.push(wincard);
        windeck.push(losecard);
        if deck0.is_empty() {
            return deck_score(&deck1);
        }
        if deck1.is_empty() {
            return deck_score(&deck0);
        }
    }
    unreachable!()
}

// ret: (score, winner is player 0)
fn recursive_combat(mut deck0: Vec<u32>, mut deck1: Vec<u32>) -> (u32, bool) {
    let mut previous_states: HashSet<(Vec<u32>, Vec<u32>)> = HashSet::new();
    for _round in 1.. {
        if !previous_states.insert((deck0.clone(), deck1.clone())) {
            return (deck_score(&deck0), true);
        }
        let card0 = deck0.remove(0);
        let card1 = deck1.remove(0);
        assert!(card0 != card1);
        let recurse = deck0.len() >= card0 as usize && deck1.len() >= card1 as usize;
        let win_condition = if recurse {
            recursive_combat(deck0[0..(card0 as usize)].to_vec(), deck1[0..(card1 as usize)].to_vec()).1
        } else {
            card0 > card1
        };

        let (windeck, wincard, losecard) = if win_condition {
            (&mut deck0, card0, card1)
        } else {
            (&mut deck1, card1, card0)
        };
        windeck.push(wincard);
        windeck.push(losecard);

        if deck0.is_empty() {
            return (deck_score(&deck1), false);
        }
        if deck1.is_empty() {
            return (deck_score(&deck0), true);
        }
    }
    unreachable!()
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let mut decks = lines.split(|x| x == "");
    let deck0: Vec<u32> = decks.next().unwrap().iter().skip(1).map(|x| x.parse().unwrap()).collect();
    let deck1: Vec<u32> = decks.next().unwrap().iter().skip(1).map(|x| x.parse().unwrap()).collect();
    println!("{}", winning_score(deck0.clone(), deck1.clone()));
    println!("{}", recursive_combat(deck0, deck1).0);
}
