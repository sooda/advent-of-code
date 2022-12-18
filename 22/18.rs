use std::io::{self, BufRead};
use std::collections::HashSet;

type Cube = (i32, i32, i32);

fn sides_exposed(cubes: &HashSet<Cube>, this: Cube) -> usize {
    let dpos = &[
        (-1,  0,  0),
        ( 1,  0,  0),
        ( 0, -1,  0),
        ( 0,  1,  0),
        ( 0,  0, -1),
        ( 0,  0,  1),
    ];
    dpos.iter().filter(|d| {
        !cubes.contains(&(this.0 + d.0, this.1 + d.1, this.2 + d.2))
    }).count()
}

fn surface_area(cubes: &[Cube]) -> usize {
    let cubes: HashSet<_> = cubes.iter().copied().collect();
    cubes.iter().map(|&c| sides_exposed(&cubes, c)).sum()
}

fn parse_cube(line: &str) -> Cube {
    let mut sp = line.split(',').map(|a| a.parse().unwrap());
    (sp.next().unwrap(), sp.next().unwrap(), sp.next().unwrap())
}

fn main() {
    let cubes: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_cube(&line.unwrap()))
        .collect();
    println!("{}", surface_area(&cubes));
}

