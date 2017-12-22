use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashMap;

fn infected_bursts(source_map: &[String], bursts: usize) -> usize {
    // don't know how big the map will grow, so mark only the infected positions
    let mut map: HashMap<(i32, i32), char> = source_map.iter().enumerate()
        .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c)))
        .collect();

    // odd size truncates to center
    let mut x = (source_map[0].len() / 2) as i32;
    let mut y = (source_map.len() / 2) as i32;
    let mut dx = 0;
    let mut dy = -1;

    let mut infected = 0;

    for _ in 0..bursts {
        // new ones behind the initial view area come up as clean
        let v = map.entry((x, y)).or_insert('.');

        if *v == '#' {
            // right
            // (dx, dy) = (-dy, dx);
            std::mem::swap(&mut dx, &mut dy);
            dx = -dx;
            *v = '.';
        } else if *v == '.' {
            // left
            // (dx, dy) = (dy, -dx);
            std::mem::swap(&mut dx, &mut dy);
            dy = -dy;
            *v = '#';
            infected += 1;
        } else {
            unreachable!()
        }

        x += dx;
        y += dy;
    }

    infected
}

fn infected_evolved_bursts(source_map: &[String], bursts: usize) -> usize {
    // don't know how big the map will grow, so mark only the infected positions
    let mut map: HashMap<(i32, i32), char> = source_map.iter().enumerate()
        .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c)))
        .collect();

    // odd size truncates to center
    let mut x = (source_map[0].len() / 2) as i32;
    let mut y = (source_map.len() / 2) as i32;
    let mut dx = 0;
    let mut dy = -1;

    let mut infected = 0;

    for _ in 0..bursts {
        // new ones behind the initial view area come up as clean
        let v = map.entry((x, y)).or_insert('.');

        if *v == '.' {
            // left
            // (dx, dy) = (dy, -dx);
            std::mem::swap(&mut dx, &mut dy);
            dy = -dy;
            *v = 'W';
        } else if *v == 'W' {
            *v = '#';
            infected += 1;
        } else if *v == '#' {
            // right
            // (dx, dy) = (-dy, dx);
            std::mem::swap(&mut dx, &mut dy);
            dx = -dx;
            *v = 'F';
        } else if *v == 'F' {
            dx = -dx;
            dy = -dy;
            *v = '.';
        } else {
            unreachable!()
        }

        x += dx;
        y += dy;
    }

    infected
}

fn main() {
    let map = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    println!("{:?}", infected_bursts(&map, 10000));
    println!("{:?}", infected_evolved_bursts(&map, 10000000));
}
