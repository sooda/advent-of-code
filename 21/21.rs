use std::io::{self, BufRead};
use std::collections::HashMap;

fn play_game(mut apos: i32, mut bpos: i32) -> i32 {
    // move to zero-based indexing
    apos -= 1;
    bpos -= 1;

    let mut ascore = 0;
    let mut bscore = 0;
    let mut random_dice = -1;
    let mut roll = || { random_dice += 1; (random_dice % 100) + 1 };

    for round in 1.. {
        apos += roll();
        apos += roll();
        apos += roll();
        apos %= 10;
        ascore += apos + 1;
        if ascore >= 1000 {
            return bscore * (6 * (round - 1) + 3);
        }
        bpos += roll();
        bpos += roll();
        bpos += roll();
        bpos %= 10;
        bscore += bpos + 1;
        if bscore >= 1000 {
            return ascore * 6 * round;
        }
    }
    unreachable!()
}

// positions and scores and player one turn
type GameState = (i32, i32, i32, i32, bool);
type GameStates = HashMap::<GameState, (usize, usize)>;

fn dirac_recurse(start_state: GameState, states: &mut GameStates) -> (usize, usize) {
    if let Some(x) = states.get(&start_state) {
        return *x;
    }
    let mut wins_a = 0;
    let mut wins_b = 0;
    for d0 in 1..=3 {
        for d1 in 1..=3 {
            for d2 in 1..=3 {
                let mut state = start_state;
                let die = d0 + d1 + d2;
                if state.4 {
                    let pos = (state.0 + die) % 10;
                    let score = state.2 + 1 + pos;
                    if score >= 21 {
                        wins_a += 1;
                    } else {
                        state.0 = pos;
                        state.2 = score;
                        state.4 = !state.4;
                        let subwins = dirac_recurse(state, states);
                        wins_a += subwins.0;
                        wins_b += subwins.1;
                    }
                } else {
                    let pos = (state.1 + die) % 10;
                    let score = state.3 + 1 + pos;
                    if score >= 21 {
                        wins_b += 1;
                    } else {
                        state.1 = pos;
                        state.3 = score;
                        state.4 = !state.4;
                        let subwins = dirac_recurse(state, states);
                        wins_a += subwins.0;
                        wins_b += subwins.1;
                    }
                }
            }
        }
    }
    states.insert(start_state, (wins_a, wins_b));
    (wins_a, wins_b)
}


fn play_dirac_game(apos: i32, bpos: i32) -> usize {
    // (apos, bpos, ascore, bscore, aturn)
    // max lut size of 10 * 10 * 21 * 21 = 44100 elements
    let mut mem = GameStates::new();
    let wins = dirac_recurse((apos - 1, bpos - 1, 0, 0, true), &mut mem);
    wins.0.max(wins.1)
}

fn main() {
    let positions: Vec<i32> = io::stdin().lock().lines()
        .map(|line| line.unwrap().split("position: ").skip(1).next().unwrap().parse().unwrap())
        .collect();
    println!("{:?}", play_game(positions[0], positions[1]));
    println!("{:?}", play_dirac_game(positions[0], positions[1]));
}
