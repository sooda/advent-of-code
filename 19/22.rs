use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Technique {
    NewStack,
    Cut(i32),
    Increment(i32),
}
use Technique::*;

#[derive(Debug)]
struct ParseTechError {}

impl FromStr for Technique {
    type Err = ParseTechError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        Ok(match words.next().unwrap() {
            "deal" => match words.next().unwrap() {
                "into" => NewStack,
                "with" => {
                    assert!(words.next().unwrap() == "increment");
                    Increment(words.next().unwrap().parse().unwrap())
                },
                _ => panic!("bad deal"),
            },
            "cut" => Cut(words.next().unwrap().parse().unwrap()),
            _ => panic!("bad input"),
        })
    }
}

fn newstack(deck: &mut [u32]) {
    deck.reverse();
}

fn cut(deck: &mut Vec<u32>, n: i32) {
    let n = if n >= 0 { n } else { deck.len() as i32 + n } as usize;
    let top: Vec<_> = deck.drain(0..n).collect();
    deck.extend(top);
}

fn increment(deck: &mut Vec<u32>, n: i32) {
    let n = n as usize;
    let len = deck.len();
    let mut next = vec![0; len];
    let mut pos = 0;
    // bleh, iter_mut() doesn't cycle()
    for src in deck.into_iter() {
        next[pos] = *src;
        pos = (pos + n) % len;
    }
    *deck = next;
}

fn shuffle_deck(deck: &mut Vec<u32>, tech: Technique) {
    match tech {
        NewStack => newstack(deck),
        Cut(n) => cut(deck, n),
        Increment(n) => increment(deck, n),
    };
}

fn shuffle_all<'a>(deck: &'a mut Vec<u32>, steps: &[Technique]) -> &'a Vec<u32> {
    for &tech in steps {
        shuffle_deck(deck, tech);
    }
    deck
}

fn factory_order(n: usize) -> Vec<u32> {
    (0..(n as u32)).collect()
}

fn sampledeck(tech: Technique) -> Vec<u32> {
    let mut deck = factory_order(10);
    shuffle_deck(&mut deck, tech);
    deck
}

fn where_2019(steps: &[Technique]) -> usize {
    let mut deck = factory_order(10007);
    shuffle_all(&mut deck, steps);
    deck.into_iter().position(|card| card == 2019).unwrap()
}

fn main() {
    assert_eq!(sampledeck(NewStack), &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    assert_eq!(sampledeck(Cut(3)), &[3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    assert_eq!(sampledeck(Cut(-4)), &[6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    assert_eq!(sampledeck(Increment(3)), &[0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    assert_eq!(shuffle_all(&mut factory_order(10),
            &[Increment(7), NewStack, NewStack]),
        &[0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    assert_eq!(shuffle_all(&mut factory_order(10),
            &[Cut(6), Increment(7), NewStack]),
        &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    assert_eq!(shuffle_all(&mut factory_order(10),
            &[Increment(7), Increment(9), Cut(-2)]),
        &[6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    assert_eq!(shuffle_all(&mut factory_order(10),
            &[NewStack, Cut(-2), Increment(7), Cut(8), Cut(-4), Increment(7),
            Cut(3), Increment(9), Increment(3), Cut(-1)]),
        &[9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);

    let steps: Vec<Technique> = io::stdin().lock().lines()
        .map(|l| l.unwrap().parse().unwrap()).collect();

    println!("{}", where_2019(&steps));
}
