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

fn count_matching_sides<F: Fn(Cube) -> bool>(this: Cube, f: F) -> usize {
    CUBE_NEIGHBOR_COORDS.iter().filter(|&d| {
        f((this.0 + d.0, this.1 + d.1, this.2 + d.2))
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
    if pos.0 < extents.0.0 || pos.1 < extents.0.1 || pos.2 < extents.0.2 {
        return;
    }
    if pos.0 > extents.1.0 || pos.1 > extents.1.1 || pos.2 > extents.1.2 {
        return;
    }

    exterior.insert(pos);
    for d in CUBE_NEIGHBOR_COORDS {
        flood_fill_one(cubes, exterior, extents, (pos.0 + d.0, pos.1 + d.1, pos.2 + d.2));
    }
}

fn flood_fill(cubes: &HashSet<Cube>) -> HashSet<Cube> {
    let minx = cubes.iter().map(|(x, _, _)| x).min().unwrap();
    let maxx = cubes.iter().map(|(x, _, _)| x).max().unwrap();
    let miny = cubes.iter().map(|(_, y, _)| y).min().unwrap();
    let maxy = cubes.iter().map(|(_, y, _)| y).max().unwrap();
    let minz = cubes.iter().map(|(_, _, z)| z).min().unwrap();
    let maxz = cubes.iter().map(|(_, _, z)| z).max().unwrap();
    let extents = ((minx - 1, miny - 1, minz - 1), (maxx + 1, maxy + 1, maxz + 1));
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

