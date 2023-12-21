use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque, HashSet};

type Coord = (i32, i32);
type Map = HashMap<Coord, bool>;

fn sum(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1)
}

fn search(map: &Map, spos: Coord, steps: usize) -> usize {
    let mut fifo = VecDeque::new();
    fifo.push_back((spos, steps));
    let mut reached = 0;
    let move_deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut visited = HashSet::new();
    while let Some((pos, steps_remaining)) = fifo.pop_front() {
        if !visited.insert((pos, steps_remaining)) {
            continue;
        }
        if steps_remaining > 0 {
            for nextpos in move_deltas.iter().map(|&d| sum(pos, d)) {
                if let Some(false) = map.get(&nextpos) {
                    fifo.push_back((nextpos, steps_remaining - 1));
                }
            }
        } else {
            //println!("{:?}", pos);
            reached += 1;
        }
    }
    reached
}

fn parse(lines: &[String]) -> (Map, Coord) {
    let map = lines.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as i32, y as i32), ch == '#'))
        })
    .collect();
    let spos = lines.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as i32, y as i32), ch == 'S'))
        })
    .find(|p| p.1).unwrap();

    (map, spos.0)
}

fn main() {
    let lines = io::stdin().lock().lines()
        .map(|row| row.unwrap())
        .collect::<Vec<_>>();
    let (map, start) = parse(&lines);

    println!("{}", search(&map, start, 6));
    println!("{}", search(&map, start, 64));
}
