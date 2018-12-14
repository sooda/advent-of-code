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

#[derive(Debug)]
struct Link {
    value: usize,
    prev: usize,
    next: usize,
}

// Add an element at the back of the storage, never bother about deletions (indexes are immutable,
// deleted slots just stay there as garbage)
fn insert(ring: &mut Vec<Link>, left: usize, value: usize) -> usize {
    let right = ring[left].next;
    let pos = ring.len();

    ring.push(Link { value: value, prev: left, next: right });

    ring[left].next = pos;
    ring[right].prev = pos;

    pos
}

fn unlink(ring: &mut Vec<Link>, pos: usize) -> usize {
    let val = ring[pos].value;

    let prev = ring[pos].prev;
    let next = ring[pos].next;
    ring[prev].next = next;
    ring[next].prev = prev;

    val
}

fn play(players: usize, length: usize) -> usize {
    let mut ring = vec![Link { value: 0, prev: 0, next: 0 }];
    let mut current = 0;
    let mut score = vec![0; players];

    for round in 1..=length {
        let player = round % players;
        if round % 23 != 0 {
            let left_pos = ring[current].next;
            current = insert(&mut ring, left_pos, round);
        } else {
            score[player] += round;
            for _ in 0..6 {
                current = ring[current].prev;
            }
            let del = ring[current].prev;
            score[player] += unlink(&mut ring, del);
        }
    }

    score.into_iter().max().unwrap()
}

fn main() {
    let games = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_configuration(&x.unwrap())).collect::<Vec<_>>();

    for g in &games {
        let winning_score = play(g.0, g.1);
        println!("{:?} {}", g, winning_score);
    }
    let winning_score_100 = play(games[0].0, 100 * games[0].1);
    println!("{}", winning_score_100);
}
