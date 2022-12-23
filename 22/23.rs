use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap};

type CoordT = i32;
type Coord = (CoordT, CoordT);
type Elves = HashSet<Coord>;

fn sum(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1)
}

fn move_proposition(elves: &Elves, elf: Coord, round: usize) -> Coord {
    // (0, 0) in top left
    let neighs = &[
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];
    if neighs.iter().all(|n| !elves.contains(&sum(elf, *n))) {
        elf
    } else {
        for dir in (round..round+4).map(|r| r % 4) {
            // just the closure is different though, but they're of different types.
            // FIXME: avoid repetition
            match dir {
                // north
                0 => if neighs.iter().filter(|n| n.1 == -1).all(|n| !elves.contains(&sum(elf, *n))) {
                    return (elf.0, elf.1 - 1);
                },
                // south
                1 => if neighs.iter().filter(|n| n.1 == 1).all(|n| !elves.contains(&sum(elf, *n))) {
                    return (elf.0, elf.1 + 1);
                },
                // west
                2 => if neighs.iter().filter(|n| n.0 == -1).all(|n| !elves.contains(&sum(elf, *n))) {
                    return (elf.0 - 1, elf.1);
                },
                // east
                3 => if neighs.iter().filter(|n| n.0 == 1).all(|n| !elves.contains(&sum(elf, *n))) {
                    return (elf.0 + 1, elf.1);
                },
                _ => panic!("but remainder"),
            }
        }

        elf
    }
}

fn one_round(elves: Elves, round: usize) -> Elves {
    let plan: Vec<(Coord, Coord)> = elves.iter()
        .map(|&e| (e, move_proposition(&elves, e, round))).collect();
    let dest_counts = plan.iter()
        .fold(HashMap::with_capacity(elves.len()), |mut counts, &(_, next)| {
            *counts.entry(next).or_insert(0) += 1;
            counts
        });

    plan.into_iter().map(|(elf, next)| {
        if *dest_counts.get(&next).unwrap() > 1 {
            elf
        } else {
            next
        }
    }).collect()
}

fn print_elves(elves: &Elves) {
    let minx = elves.iter().min_by_key(|&e| e.0).unwrap().0;
    let maxx = elves.iter().max_by_key(|&e| e.0).unwrap().0;
    let miny = elves.iter().min_by_key(|&e| e.1).unwrap().1;
    let maxy = elves.iter().max_by_key(|&e| e.1).unwrap().1;
    for y in miny..=maxy {
        for x in minx..=maxx {
            print!("{}", if elves.contains(&(x, y)) {
                '#'
            } else {
                '.'
            });
        }
        println!();
    }
}

fn play_rounds(mut elves: Elves, rounds: usize, steady_stop: bool) -> (Elves, usize) {
    for r in 0..rounds {
        let next_elves = one_round(elves.clone(), r);
        if steady_stop && elves == next_elves {
            return (elves, 1 + r);
        }
        elves = next_elves;
        if false {
            println!("end of round {}", 1 + r);
            print_elves(&elves);
        }
    }
    (elves, rounds)
}

fn empty_files_after(elves: Elves, rounds: usize) -> usize {
    let elves = play_rounds(elves, rounds, false).0;

    let minx = elves.iter().min_by_key(|&e| e.0).unwrap().0;
    let maxx = elves.iter().max_by_key(|&e| e.0).unwrap().0;
    let miny = elves.iter().min_by_key(|&e| e.1).unwrap().1;
    let maxy = elves.iter().max_by_key(|&e| e.1).unwrap().1;
    let w = maxx - minx + 1;
    let h = maxy - miny + 1;

    (w * h) as usize - elves.len()
}

fn rounds_needed(elves: Elves) -> usize {
    play_rounds(elves, std::usize::MAX, true).1
}

fn parse_elves(lines: &[String]) -> Elves {
    lines.iter().enumerate()
        .flat_map(|(y, line)| {
            line.chars().into_iter().enumerate().map(move |(x, ch)| (x, y, ch))
        })
        .filter(|&(_, _, ch)| ch == '#')
        .map(|(x, y, _)| (x as CoordT, y as CoordT))
        .collect()
}

fn main() {
    let lines: Vec<_> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();
    let elves = parse_elves(&lines);
    println!("{}", empty_files_after(elves.clone(), 10));
    println!("{}", rounds_needed(elves));
}
