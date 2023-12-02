use std::io::{self, BufRead};

extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct Game {
    id: u32,
    ok: bool,
}

fn parse_game(inp: &str) -> Game {
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    let re = Regex::new(r"Game (\d+): (.*)").unwrap();
    let re_reveals = Regex::new(r"([^;]+)(; |$)").unwrap();
    let re_reveal = Regex::new(r"(\d+) (red|green|blue)").unwrap();
    let cap = re.captures(inp).unwrap();
    let id = cap.get(1).unwrap().as_str().parse().unwrap();

    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for reveal in re_reveals.captures_iter(cap.get(2).unwrap().as_str()) {
        for color in re_reveal.captures_iter(reveal.get(1).unwrap().as_str()) {
            let n: u32 = color.get(1).unwrap().as_str().parse().unwrap();
            match color.get(2).unwrap().as_str() {
                "red" => r = r.max(n),
                "green" => g = g.max(n),
                "blue" => b = b.max(n),
                _ => panic!()
            }
        }
    }

    Game { id, ok: r <= 12 && g <= 13 && b <= 14 }
}

fn main() {
    let games: Vec<Game> = io::stdin().lock().lines()
        .map(|line| parse_game(&line.unwrap()))
        .collect();
    println!("{}", games.iter().filter(|x| x.ok).map(|x| x.id).sum::<u32>());
}

