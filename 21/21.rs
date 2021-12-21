use std::io::{self, BufRead};

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

fn main() {
    let positions: Vec<i32> = io::stdin().lock().lines()
        .map(|line| line.unwrap().split("position: ").skip(1).next().unwrap().parse().unwrap())
        .collect();
    println!("{:?}", play_game(positions[0], positions[1]));
}
