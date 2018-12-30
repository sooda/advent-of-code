use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;

use std::collections::BinaryHeap;

#[derive(Debug)]
struct Cave {
    depth: usize,
    tx: usize, // target
    ty: usize,
    w: usize,
    h: usize
}

fn geologic_index(x: usize, y: usize, c: &Cave, map: &Vec<usize>) -> usize {
    if x == 0 && y == 0 {
        0
    } else if x == c.tx && y == c.ty {
        0
    } else if y == 0 {
        x * 16807
    } else if x == 0 {
        y * 48271
    } else {
        let a = map[(y - 1) * c.w + x];
        let b = map[y * c.w + x - 1];
        a * b
    }
}

fn expand_map(cave: &Cave) -> Vec<usize> {
    let mut v = Vec::new();
    for y in 0..cave.h {
        for x in 0..cave.w {
            let index = geologic_index(x, y, cave, &mut v);
            let erosion_level = (index + cave.depth) % 20183;
            v.push(erosion_level);
        }
    }
    v
}

fn total_risk(cave: &Cave) -> usize {
    let levels = expand_map(cave);
    levels.iter().map(|&erosion_level| erosion_level % 3).sum()
}

#[derive(Clone, Copy)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}
use RegionType::*;

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord)]
enum Equipped {
    Gear,
    Torch,
    Neither,
}
use Equipped::*;

fn equipment_fits(rt: RegionType, eq: Equipped) -> bool {
    match rt {
        Rocky => eq != Neither,
        Wet => eq != Torch,
        Narrow => eq != Gear,
    }
}

fn eq2id(eq: Equipped) -> usize {
    match eq {
        Gear => 0,
        Torch => 1,
        Neither => 2,
    }
}

fn pathfind(map: &[RegionType], w: usize, h: usize) -> Vec<Option<usize>> {
    // negate the dist for max to be better so the heap works without a custom ord for the state
    let mut heap: BinaryHeap<(i64, usize, usize, Equipped)> = BinaryHeap::new(); // -dist, x, y, equip
    let mut distances = vec![None; w * h * 3];

    let m_idx = |x, y| {
        y * w + x
    };

    let d_idx = |x, y, eq| {
        (w * h) * eq2id(eq) + m_idx(x, y)
    };

    distances[d_idx(0, 0, Torch)] = Some(0);
    heap.push((0, 0, 0, Torch));

    while let Some(current) = heap.pop() {
        let (dist, xi, yi, eqi) = current;
        let dist = (-dist) as usize;

        if dist > distances[d_idx(xi, yi, eqi)].unwrap_or_else(|| std::usize::MAX) {
            continue;
        }

        // state changing edge
        for &eqj in &[Gear, Torch, Neither] {
            let dpos = d_idx(xi, yi, eqj);
            let dist_new = dist + 7;

            let should_visit = dist_new < distances[dpos].unwrap_or_else(|| std::usize::MAX);
            let proper_eq = equipment_fits(map[m_idx(xi, yi)], eqj);

            if should_visit && proper_eq {
                heap.push((-(dist_new as i64), xi, yi, eqj));
                distances[dpos] = Some(dist_new);
            }
        }

        // walking edge
        for &(xj, yj) in &[(xi, yi), (xi - 1, yi), (xi + 1, yi), (xi, yi - 1), (xi, yi + 1)] {
            let in_range = xj < w && yj < h;
            if !in_range {
                continue;
            }

            let dpos = d_idx(xj, yj, eqi);
            let dist_new = dist + 1;

            let should_visit = dist_new < distances[dpos].unwrap_or_else(|| std::usize::MAX);
            let proper_eq = equipment_fits(map[m_idx(xj, yj)], eqi);

            if should_visit && proper_eq {
                heap.push((-(dist_new as i64), xj, yj, eqi));
                distances[dpos] = Some(dist_new);
            }
        }
    }

    if false {
        print!("{: >5} ", "");
        for x in 0..w {
            print!("xx{:x>5} ", x);
        }
        println!("");
        for y in 0..h {
            for &eq in &[Gear, Torch, Neither] {
                print!("{:y>5} ", y);
                for x in 0..w {
                    match map[y * w + x] {
                        Rocky => print!("."),
                        Wet => print!("="),
                        Narrow => print!("|")
                    }
                    match eq {
                        Gear => print!("g"),
                        Torch => print!("t"),
                        Neither => print!("n"),
                    }
                    match distances[d_idx(x, y, eq)] {
                        Some(d) => print!("{:_>5} ", d),
                        None => print!(",,,,, "),
                    }
                }
                println!("");
            }
            println!("");
        }
    }

    distances
}

fn parse_type(erosion_level: &usize) -> RegionType {
    match erosion_level % 3 {
        0 => Rocky,
        1 => Wet,
        2 => Narrow,
        _ => unreachable!()
    }
}

fn shortest(cave: &Cave) -> usize {
    // reserve "some" extra space for cave exploration outside the known area
    let cave = &Cave {
        depth: cave.depth,
        tx: cave.tx,
        ty: cave.ty,
        // this should be enough, the size is about 10 by 700
        w: 5 * cave.w,
        h: 2 * cave.h
    };

    let levels = expand_map(cave);
    let region_types: Vec<_> = levels.iter().map(parse_type).collect();
    let dists = pathfind(&region_types, cave.w, cave.h);

    let m_idx = |x, y| {
        y * cave.w + x
    };

    let d_idx = |x, y, eq| {
        (cave.w * cave.h) * eq2id(eq) + m_idx(x, y)
    };

    dists[d_idx(cave.tx, cave.ty, Torch)].unwrap()
}

fn parse_cave(input: &mut Lines<BufReader<File>>) -> Cave {
    /*
     * depth: 11739
     * target: 11,718
     */
    let depline = input.next().unwrap().unwrap();
    let depth = depline.split(" ").nth(1).unwrap().parse().unwrap();
    let tarline = input.next().unwrap().unwrap();
    let coords = tarline.split(" ").nth(1).unwrap();
    let mut xy = coords.split(",");
    let x = xy.next().unwrap().parse().unwrap();
    let y = xy.next().unwrap().parse().unwrap();
    Cave { depth: depth, tx: x, ty: y, w: x + 1, h: y + 1 }
}

fn main() {
    let mut input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines();
    let cave = parse_cave(&mut input);
    println!("{:?}", cave);
    println!("{}", total_risk(&cave));
    println!("{}", shortest(&cave));
}
