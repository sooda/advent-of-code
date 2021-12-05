use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct Line {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

fn line_horiz_or_vert(line: &&Line) -> bool {
    line.x0 == line.x1 || line.y0 == line.y1
}

fn dangerous_sum(lines: &[Line]) -> usize {
    let mut map = HashMap::<(i32, i32), usize>::new();
    for line in lines.iter().filter(line_horiz_or_vert) {
        // one of these iterates only once
        for x in line.x0.min(line.x1)..=line.x0.max(line.x1) {
            for y in line.y0.min(line.y1)..=line.y0.max(line.y1) {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    map.values().filter(|&&n| n >= 2).count()
}

fn full_dangerous_sum(lines: &[Line]) -> usize {
    let mut map = HashMap::<(i32, i32), usize>::new();
    for line in lines.iter() {
        let dx = line.x1 - line.x0;
        let dy = line.y1 - line.y0;
        let n = dx.abs().max(dy.abs());
        let mut x = line.x0;
        let mut y = line.y0;
        for _ in 0..=n {
            *map.entry((x, y)).or_insert(0) += 1;
            x += dx.signum();
            y += dy.signum();
        }
    }
    map.values().filter(|&&n| n >= 2).count()
}

fn parse_line(input: &str) -> Line {
    let mut sp = input.split(" -> ");
    let mut asp = sp.next().unwrap().split(',');
    let mut bsp = sp.next().unwrap().split(',');
    let x0: i32 = asp.next().unwrap().parse().unwrap();
    let y0: i32 = asp.next().unwrap().parse().unwrap();
    let x1: i32 = bsp.next().unwrap().parse().unwrap();
    let y1: i32 = bsp.next().unwrap().parse().unwrap();
    Line { x0, y0, x1, y1 }
}

fn main() {
    let lines: Vec<Line> = io::stdin().lock().lines()
        .map(|input| parse_line(&input.unwrap()))
        .collect();
    println!("{}", dangerous_sum(&lines));
    println!("{}", full_dangerous_sum(&lines));
}
