use std::io::{self, BufRead};

type Pos = (i64, i64);

fn area(a: Pos, b: Pos) -> i64 {
    ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1)
}

fn greatest_area(tiles: &[Pos]) -> i64 {
    tiles.iter()
        .enumerate()
        .flat_map(|(i, &a)| tiles.iter().skip(i + 1).map(move |&b| area(a, b)))
        .max().unwrap()
}

fn parse(line: &str) -> Pos {
    let ab = line.split_once(',').unwrap();
    (ab.0.parse().unwrap(), ab.1.parse().unwrap())
}

fn main() {
    let tiles = io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap()))
        .collect::<Vec<_>>();
    println!("{}", greatest_area(&tiles));
}

