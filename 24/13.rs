use std::io::{self, BufRead};

extern crate regex;
use regex::Regex;

type Pos = (i32, i32);

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn mul(a: i32, b: Pos) -> Pos {
    (a * b.0, a * b.1)
}

struct Game {
    button_a: Pos,
    button_b: Pos,
    prize: Pos
}

fn play(game: &Game) -> Option<usize> {
    let mut besttokens = None;
    for a in 1..=100 {
        for b in 1..=100 {
            let tokens = (3 * a + b) as usize;
            if add(mul(a, game.button_a), mul(b, game.button_b)) == game.prize {
                if tokens < besttokens.unwrap_or(std::usize::MAX) {
                    besttokens = Some(tokens);
                }
            }
        }
    }
    besttokens
}

fn fewest_tokens(games: &[Game]) -> usize {
    games.iter().filter_map(play).sum()
}

fn parse<'a>(specs: impl Iterator<Item=&'a [String]>) -> Vec<Game> {
    let re = Regex::new(r"(\d+),[^\d]+(\d+)").unwrap();
    let parse = |line| {
        let cap = re.captures(line).unwrap();
        (cap.get(1).unwrap().as_str().parse().unwrap(),
         cap.get(2).unwrap().as_str().parse().unwrap())
    };
    specs.map(|spec| {
            let button_a = parse(&spec[0]);
            let button_b = parse(&spec[1]);
            let prize = parse(&spec[2]);
            Game { button_a, button_b, prize }
        })
    .collect()
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let games = parse(lines.split(|l| l.is_empty()));
    println!("{}", fewest_tokens(&games));
}
