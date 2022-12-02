use std::io::{self, BufRead};

type Rps = i32;

fn play_game_exactly(a: Rps, b: Rps) -> i32 {
    let shape_score = b + 1;
    if false {
        let _outcome = match (b - a + 3) % 3 {
            2 => 0, // lost
            0 => 3, // equal
            1 => 6, // won
            _ => panic!()
        };
    }
    // rotate the above by 1
    let outcome = (b - a + 1 + 3) % 3;
    let outcome_score = 3 * outcome;
    shape_score + outcome_score
}

fn play_games_exactly(games: &[(Rps, Rps)]) -> i32 {
    games.iter().map(|&(x, y)| play_game_exactly(x, y)).sum()
}

fn play_game_by_plan(a: Rps, b: Rps) -> i32 {
    let outcome_score = 3 * b;
    if false {
        let (_outcome, _my_shape) = match b {
            0 => (0, (a + 2) % 3), // lost
            1 => (3, a), // equal
            2 => (6, (a + 1) % 3), // won
            _ => panic!()
        };
    }
    // rotate the above by 2
    let my_shape = (a + b + 2 + 3) % 3;
    let shape_score = my_shape + 1;
    shape_score + outcome_score
}

fn play_games_by_plan(games: &[(Rps, Rps)]) -> i32 {
    games.iter().map(|&(x, y)| play_game_by_plan(x, y)).sum()
}

// 0 for rock, 1 for paper, 2 for scissors
// or 0 for lose, 1 for draw, 2 for win
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
    println!("{}", play_games_exactly(&games));
    println!("{}", play_games_by_plan(&games));
}
