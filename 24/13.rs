use std::io::{self, BufRead};

extern crate regex;
use regex::Regex;

type Pos = (i64, i64);

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn mul(a: i64, b: Pos) -> Pos {
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

/*
 * q * a + w * b = p | minimize 3 * q + w
 *
 * q * a.x + w * b.x = p.x
 * q * a.y + w * b.y = p.y
 *
 * q = (p.x - w * b.x) / a.x
 *   = (p.y - w * b.y) / a.y
 *
 * (p.x - w * b.x) / a.x = (p.y - w * b.y) / a.y
 * (p.x - w * b.x) = (p.y - w * b.y) * a.x / a.y
 * - w * b.x = (p.y - w * b.y) * a.x / a.y - p.x
 *   w * b.x = p.x - (p.y - w * b.y) * a.x / a.y
 *   w * b.x + (p.y - w * b.y) * a.x / a.y = p.x
 *   w * b.x + p.y * a.x / a.y - w * b.y * a.x / a.y = p.x
 *   w * b.x - w * b.y * a.x / a.y = p.x - p.y * a.x / a.y
 *   w * (b.x - b.y * a.x / a.y) = p.x - p.y * a.x / a.y
 *   w = (p.x - p.y * a.x / a.y) / (b.x - b.y * a.x / a.y)
 *     = (p.x * a.y / a.y - p.y * a.x / a.y) / (b.x * a.y / a.y - b.y * a.x / a.y)
 *     = ((p.x * a.y - p.y * a.x) / a.y) / ((b.x * a.y - b.y * a.x) / a.y)
 *     = (p.x * a.y - p.y * a.x) / (b.x * a.y - b.y * a.x)
 *
 * The "smallest number of tokens" appears to be just a diversion; it only applies for collinear
 * button vectors and those don't exist (in my input, anyway).
 */
fn play_fast(game: &Game) -> Option<usize> {
    let (a, b, p) = (game.button_a, game.button_b, game.prize);
    let w_top = p.0 * a.1 - p.1 * a.0;
    let w_bot = b.0 * a.1 - b.1 * a.0;
    if w_top % w_bot == 0 {
        let w = w_top / w_bot;
        let q1 = (p.0 - w * b.0) / a.0;
        let q2 = (p.1 - w * b.1) / a.1;
        assert_eq!(q1, q2);
        return Some((3 * q1 + w) as usize);
    } else {
        None
    }
}

fn fewest_tokens(games: &[Game]) -> usize {
    games.iter().filter_map(play).sum()
}

fn fewest_tokens_fast(games: &[Game]) -> usize {
    games.iter().filter_map(play_fast).sum()
}

fn repair(games: &[Game]) -> Vec<Game> {
    games.into_iter()
        .map(|g| {
            Game {
                button_a: g.button_a,
                button_b: g.button_b,
                prize: add(g.prize, (10000000000000, 10000000000000))
            }
        })
    .collect()
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
    println!("{}", fewest_tokens_fast(&games));
    println!("{}", fewest_tokens_fast(&repair(&games)));
}
