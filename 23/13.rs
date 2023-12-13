use std::io::{self, Read};
use std::collections::HashMap;

type Pattern = HashMap<(i32, i32), bool>;

// vertical
fn try_reflect_x(pat: &Pattern, mirror_x: i32) -> bool {
    let _maxx = *pat.keys().map(|(x, _)| x).max().unwrap();
    let maxy = *pat.keys().map(|(_, y)| y).max().unwrap();
    for xi in 0..=mirror_x {
        let dx = mirror_x - xi;
        let xj = mirror_x + 1 + dx;
        for yi in 0..=maxy {
            let cell = pat.get(&(xi, yi)).unwrap();
            if pat.get(&(xj, yi)).unwrap_or(cell) != cell {
                return false;
            }
        }
    }
    true
}

// horizontal, but TODO: rotating 90 degrees would have avoided this copypasta
// TODO: or do generic get(), abstract the axes away
fn try_reflect_y(pat: &Pattern, mirror_y: i32) -> bool {
    let maxx = *pat.keys().map(|(x, _)| x).max().unwrap();
    let _maxy = *pat.keys().map(|(_, y)| y).max().unwrap();
    for yi in 0..=mirror_y {
        let dy = mirror_y - yi;
        let yj = mirror_y + 1 + dy;
        for xi in 0..=maxx {
            let cell = pat.get(&(xi, yi)).unwrap();
            if pat.get(&(xi, yj)).unwrap_or(cell) != cell {
                return false;
            }
        }
    }
    true
}

fn summary(pat: &Pattern) -> i32 {
    let maxx = *pat.keys().map(|(x, _)| x).max().unwrap();
    let maxy = *pat.keys().map(|(_, y)| y).max().unwrap();
    let horiz = (0..maxx).filter(|&x| try_reflect_x(pat, x)).map(|x| x + 1).sum::<i32>();
    let verti = (0..maxy).filter(|&y| try_reflect_y(pat, y)).map(|y| 100 * (y + 1)).sum::<i32>();
    horiz + verti
}

fn all_summary(notes: &[Pattern]) -> i32 {
    notes.iter().map(|p| summary(p)).sum()
}

fn parse(file: &str) -> Vec<Pattern> {
    file.split("\n\n")
        .map(|pat| {
            pat.split('\n')
                .enumerate()
                .flat_map(|(y, row)| {
                    row.chars()
                        .enumerate()
                        .map(move |(x, ch)| {
                            ((x as i32, y as i32), ch == '#')
                        })
                })
            .collect::<Pattern>()
        }).collect::<Vec<_>>()
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let notes = parse(&file);
    println!("{}", all_summary(&notes));
}
