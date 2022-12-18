use std::io::{self, BufRead};
use std::collections::HashSet;

type Cube = (i32, i32, i32);

const CUBE_NEIGHBOR_COORDS: &[Cube; 6] = &[
    (-1,  0,  0),
    ( 1,  0,  0),
    ( 0, -1,  0),
    ( 0,  1,  0),
    ( 0,  0, -1),
    ( 0,  0,  1),
];

fn sum(a: Cube, b: Cube) -> Cube {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn less(a: Cube, b: Cube) -> bool {
    a.0 < b.0 || a.1 < b.1 || a.2 < b.2
}

fn count_matching_sides<F: Fn(Cube) -> bool>(this: Cube, f: F) -> usize {
    CUBE_NEIGHBOR_COORDS.iter().filter(|&&d| {
        f(sum(this, d))
    }).count()
}

fn surface_area(cubes: &HashSet<Cube>) -> usize {
    cubes.iter().map(|&c| {
        count_matching_sides(c, |d| !cubes.contains(&d))
    }).sum()
}

fn flood_fill_one(cubes: &HashSet<Cube>, exterior: &mut HashSet<Cube>, extents: (Cube, Cube), pos: Cube) {
    if exterior.contains(&pos) {
        return;
    }
    if cubes.contains(&pos) {
        return;
    }
    if less(pos, extents.0) {
        return;
    }
    if less(extents.1, pos) {
        return;
    }

    exterior.insert(pos);
    for &d in CUBE_NEIGHBOR_COORDS {
        flood_fill_one(cubes, exterior, extents, sum(pos, d));
    }
}

fn flood_fill(cubes: &HashSet<Cube>) -> HashSet<Cube> {
    let min = (
        cubes.iter().map(|&(x, _, _)| x).min().unwrap(),
        cubes.iter().map(|&(_, y, _)| y).min().unwrap(),
        cubes.iter().map(|&(_, _, z)| z).min().unwrap(),
    );
    let max = (
        cubes.iter().map(|&(x, _, _)| x).max().unwrap(),
        cubes.iter().map(|&(_, y, _)| y).max().unwrap(),
        cubes.iter().map(|&(_, _, z)| z).max().unwrap(),
    );
    let extents = (sum(min, (-1, -1, -1)), sum(max, (1, 1, 1)));
        let mut exterior = HashSet::new();
    flood_fill_one(cubes, &mut exterior, extents, extents.0);
    exterior
}

fn exterior_surface_area(cubes: &HashSet<Cube>) -> usize {
    let gas = flood_fill(cubes);
    cubes.iter().map(|&c| {
        count_matching_sides(c, |d| gas.contains(&d))
    }).sum()
}

fn parse_cube(line: &str) -> Cube {
    let mut sp = line.split(',').map(|a| a.parse().unwrap());
    (sp.next().unwrap(), sp.next().unwrap(), sp.next().unwrap())
}

fn main() {
    let cubes: HashSet<_> = io::stdin().lock().lines()
        .map(|line| parse_cube(&line.unwrap()))
        .collect();
    println!("{}", surface_area(&cubes));
    println!("{}", exterior_surface_area(&cubes));
}

