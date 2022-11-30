use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

const OPEN: char = '.';
const TREE: char = '|';
const YARD: char = '#';

fn parse_line(input: &str) -> Vec<char> {
    /*
     * .#.#...|#.
     * .....#|##|
     */
    input.chars().collect()
}

fn dunp(map: &[Vec<char>]) {
    for row in map {
        for tile in row {
            print!("{}", tile);
        }
        println!("");
    }
    println!("");
}


fn neighbourhood(map: &[Vec<char>], x: usize, y: usize) -> (usize, usize) {
    let mut trees = 0;
    let mut yards = 0;
    let default_row = Vec::new();
    let default_yard = '.';
    for &(xi, yi) in &[
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y    ),             (x + 1, y    ),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)
    ] {
        match *map.get(yi).unwrap_or_else(|| &default_row).get(xi).unwrap_or_else(|| &default_yard) {
            TREE => trees += 1,
            YARD => yards += 1,
            OPEN => (),
            _ => unreachable!()
        }
    }
    (trees, yards)
}

fn change(map: &[Vec<char>], x: usize, y: usize) -> char {
    let (trees, yards) = neighbourhood(map, x, y);
    match map[y][x] {
        OPEN => if trees >= 3 { TREE } else { OPEN },
        TREE => if yards >= 3 { YARD } else { TREE },
        YARD => if yards >= 1 && trees >= 1 { YARD } else { OPEN },
        _ => unreachable!()
    }
}

fn magic(map: &[Vec<char>], next: &mut [Vec<char>]) {
    for (y, row) in next.iter_mut().enumerate() {
        for (x, tile) in row.iter_mut().enumerate() {
            *tile = change(map, x, y);
        }
    }
}

fn score(map: &[Vec<char>]) -> usize {
    let trees = map.iter().flat_map(|row| row.iter().filter(|&&tile| tile == TREE)).count();
    let lumberyards = map.iter().flat_map(|row| row.iter().filter(|&&tile| tile == YARD)).count();

    //println!("{} * {} = {}", trees, lumberyards, trees * lumberyards);
    //println!("");

    trees * lumberyards
}

fn animate(map: &mut Vec<Vec<char>>, n: usize) -> usize {
    let w = map[0].len();
    let h = map.len();

    let mut next = vec![vec!['?'; w]; h];

    for i in 1..=n {
        magic(map, &mut next);
        for (ro, ri) in map.iter_mut().zip(next.iter()) {
            ro.copy_from_slice(ri);
        }
        println!("After {} minutes:", i);
        dunp(map);
        println!("");
    }

    score(map)
}

fn cycle_len(v: &Vec<usize>) -> usize {
    for len in 1.. {
        let a = v.iter().rev();
        let b = v.iter().rev().skip(len);
        if a.zip(b).take(len).all(|(a, b)| a == b) {
            return len;
        }
    }
    unreachable!()
}

fn cycledetect(map: &mut Vec<Vec<char>>, n: usize) -> usize {
    let w = map[0].len();
    let h = map.len();
    let stabilize_time = 1000;
    let mut history = Vec::new();

    let mut next = vec![vec!['?'; w]; h];

    for _ in 0..=stabilize_time {
        magic(map, &mut next);
        for (ro, ri) in map.iter_mut().zip(next.iter()) {
            ro.copy_from_slice(ri);
        }
        history.push(score(map));
    }
    let cycle = cycle_len(&history);
    println!("c: {}", cycle);
    // After big enough a, any f(a) + f(a + b * C) for integer b.
    // For big n = a + b * C, find reasonable b to get 1000 - C <= n + b * C < 1000
    let b = (n - stabilize_time + cycle-1) / cycle; // round down to go safely under 1000 if exact match
    let a = n - b * cycle - 1;
    history[a]
}

fn main() {
    let mut map = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|l| parse_line(&l.unwrap())).collect::<Vec<_>>();
    dunp(&map);
    println!("{}", animate(&mut map.clone(), 10));
    println!("{}", cycledetect(&mut map, 1000000000));
}
