use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// "10 players; last marble is worth 25 points"
fn parse_configuration(line: &str) -> (usize, usize) {
    let mut words = line.split(" ");
    let players = words.next().unwrap().parse().unwrap();
    let length = words.nth(5).unwrap().parse().unwrap();
    (players, length)
}

fn play(players: usize, length: usize) -> usize {
    let mut ring = vec![0];
    let mut current = 0;
    let mut score = vec![0; players];

    for round in 1..=length {
        let player = round % players;
        if round % 23 != 0 {
            current = (current + 1) % ring.len() + 1;
            ring.insert(current, round);
        } else {
            score[player] += round;
            current = (current + ring.len() - 7) % ring.len();
            score[player] += ring.remove(current);
        }
    }

    score.into_iter().max().unwrap()
}

fn main() {
    let games = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_configuration(&x.unwrap())).collect::<Vec<_>>();

    for g in games {
        let winning_score = play(g.0, g.1);
        println!("{:?} {}", g, winning_score);
    }
}
