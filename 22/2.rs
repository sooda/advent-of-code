use std::io::{self, BufRead};

type Rps = i32;

fn play_game(a: Rps, b: Rps) -> i32 {
    let shape = b + 1;
    // this could be branchless more modulo magic
    let outcome = match (b - a + 3) % 3 {
        2 => 0, // lost
        0 => 3, // equal
        1 => 6, // won
        _ => panic!()
    };
    shape + outcome
}

fn play_games_exactly(games: &[(Rps, Rps)]) -> i32 {
    games.iter().map(|&(x, y)| play_game(x, y)).sum()
}

// 0 for rock, 1 for paper, 2 for scissors
fn decode_rps(ch: u8) -> Rps {
    Rps::from(match ch {
        b'A'..=b'C' => ch - b'A',
        b'X'..=b'Z' => ch - b'X',
        _ => panic!()
    })
}

fn decode_strategy(l: &str) -> (Rps, Rps) {
    let abc = decode_rps(l.as_bytes()[0]);
    let xyz = decode_rps(l.as_bytes()[2]);
    (abc, xyz)
}

fn main() {
    let games: Vec<_> = io::stdin().lock().lines()
        .map(|line| decode_strategy(&line.unwrap()))
        .collect();
    let total_score = play_games_exactly(&games);
    println!("{}", total_score);
}
