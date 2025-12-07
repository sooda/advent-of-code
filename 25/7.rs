use std::io::{self, BufRead};
use std::collections::HashSet;
use std::collections::HashMap;

type Map = Vec<Vec<char>>;

fn splits(input: &Map, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> usize {
    if y == input.len() {
        0
    } else if visited.contains(&(x, y)) {
        0
    } else if input[y][x] == '.' {
        splits(input, x, y + 1, visited)
    } else if input[y][x] == '^' {
        visited.insert((x, y));
        // safe, the input does not have splitters at the edges
        1 + splits(input, x - 1, y + 1, visited) + splits(input, x + 1, y + 1, visited)
    } else {
        panic!("invalid input")
    }
}

fn total_splits(input: &Map) -> usize {
    let start = input[0].iter().position(|&a| a == 'S').unwrap();
    splits(&input, start, 1, &mut HashSet::new())
}

fn timelines(input: &Map, x: usize, y: usize, visited: &mut HashMap<(usize, usize), usize>) -> usize {
    if y == input.len() {
        1
    } else if let Some(&x) = visited.get(&(x, y)) {
        x
    } else if input[y][x] == '.' {
        timelines(input, x, y + 1, visited)
    } else if input[y][x] == '^' {
        // safe, the input does not have splitters at the edges
        let n = timelines(input, x - 1, y + 1, visited) + timelines(input, x + 1, y + 1, visited);
        visited.insert((x, y), n);
        n
    } else {
        panic!("invalid input")
    }
}

fn total_timelines(input: &Map) -> usize {
    let start = input[0].iter().position(|&a| a == 'S').unwrap();
    timelines(&input, start, 1, &mut HashMap::new())
}

fn main() {
    let input = io::stdin().lock().lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{}", total_splits(&input));
    println!("{}", total_timelines(&input));
}
