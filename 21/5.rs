use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct Line {
    x0: u32,
    y0: u32,
    x1: u32,
    y1: u32,
}

fn line_horiz_or_vert(line: &&Line) -> bool {
    line.x0 == line.x1 || line.y0 == line.y1
}

fn dangerous_sum(lines: &[Line]) -> usize {
    let mut map = HashMap::<(u32, u32), usize>::new();
    for line in lines.iter().filter(line_horiz_or_vert) {
        // one of these iterates only once
        for x in line.x0..=line.x1 {
            for y in line.y0..=line.y1 {
                *map.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    map.values().filter(|&&n| n >= 2).count()
}

fn parse_line(input: &str) -> Line {
    let mut sp = input.split(" -> ");
    let mut asp = sp.next().unwrap().split(',');
    let mut bsp = sp.next().unwrap().split(',');
    let x0: u32 = asp.next().unwrap().parse().unwrap();
    let y0: u32 = asp.next().unwrap().parse().unwrap();
    let x1: u32 = bsp.next().unwrap().parse().unwrap();
    let y1: u32 = bsp.next().unwrap().parse().unwrap();
    Line { x0: x0.min(x1), y0: y0.min(y1), x1: x0.max(x1), y1: y0.max(y1) }
}

fn main() {
    let lines: Vec<Line> = io::stdin().lock().lines()
        .map(|input| parse_line(&input.unwrap()))
        .collect();
    println!("{}", dangerous_sum(&lines));
}
