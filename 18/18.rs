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

fn play(map: &mut Vec<Vec<char>>) -> usize {
    let w = map[0].len();
    let h = map.len();

    let mut next = vec![vec!['?'; w]; h];

    for i in 1..=10 {
        magic(map, &mut next);
        println!("After {} minutes:", i);
        for (ro, ri) in map.iter_mut().zip(next.iter()) {
            ro.copy_from_slice(ri);
        }
        dunp(map);
    }

    let trees = map.iter().flat_map(|row| row.iter().filter(|&&tile| tile == TREE)).count();
    let lumberyards = map.iter().flat_map(|row| row.iter().filter(|&&tile| tile == YARD)).count();

    println!("{} * {}", trees, lumberyards);

    trees * lumberyards
}

fn main() {
    let mut map = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|l| parse_line(&l.unwrap())).collect::<Vec<_>>();
    dunp(&map);
    println!("{}", play(&mut map));
}
