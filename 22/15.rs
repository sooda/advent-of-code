use std::io::{self, BufRead};
use std::collections::HashSet;

extern crate regex;
use regex::Regex;

type Pos = (i64, i64);
// its position and the closest beacon position, as in the input
type Sensor = (Pos, Pos);

// inclusive
type Span = (i64, i64);

// works both ways
fn overlap(a: Span, b: Span) -> bool {
    return b.0 <= a.1 && b.1 >= a.0
}

fn try_merge(a: Span, b: Span) -> Option<Span> {
    if overlap(a, b) {
        Some((a.0.min(b.0), a.1.max(b.1)))
    } else {
        None
    }
}

fn try_merge_any(spans: &[Span], next: Span) -> Option<(usize, Span)> {
    spans.iter()
        .enumerate()
        .filter_map(|(i, &sp)| try_merge(sp, next).map(|sp| (i, sp)))
        .next()
}

fn positions(sensors: &[Sensor], row: i64) -> usize {
    let mut spans: Vec<Span> = Vec::new();
    for sensor in sensors {
        let view = (sensor.0.0 - sensor.1.0).abs() + (sensor.0.1 - sensor.1.1).abs();
        if false {
            println!("sens at {:?} for {:?} view {} sees {:?}",
                     sensor.0, sensor.1, view, (sensor.0.1 - row).abs() <= view);
        }
        let dy = (row - sensor.0.1).abs();
        if dy <= view {
            let dx = view - dy;
            let min = sensor.0.0 - dx;
            let max = sensor.0.0 + dx;
            if false {
                println!("dy {}  sees this much {} its min max = {} {}",
                         dy, dx, min, max);
            }
            let mut this_span = (min, max);
            while let Some((i, combined)) = try_merge_any(&spans, this_span) {
                this_span = combined;
                spans.remove(i);
            }
            spans.push(this_span);
        }
    }

    if spans.len() == 2 {
        spans.sort_unstable();
        let (l, r) = (spans[0], spans[1]);
        if l.0 <= 0 && r.1 >= 20 && l.1 + 2 == r.0 {
            let x = l.1 + 1;
            println!("ahoy x {} y {} score {}", x, row, x * 4000000 + row);
        }
    }

    // multiples are ok
    let beacons: HashSet<_> = sensors.iter().map(|&(_, b)| b).collect();
    let visible = beacons.iter().filter(|&b| {
        b.1 == row && spans.iter().any(|sp| b.0 >= sp.0 && b.0 <= sp.1)
    }).count();

    spans.iter().map(|sp| sp.1 - sp.0 + 1).sum::<i64>() as usize - visible
}

fn tuning_freq(sensors: &[Sensor], maxsize: i64) -> i64 {
    for y in 0..=maxsize {
        positions(sensors, y);
    }
    0
}

fn parse_sensor(line: &str) -> Sensor {
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    let cap = re.captures(line).unwrap();
    let sx = cap.get(1).unwrap().as_str().parse().unwrap();
    let sy = cap.get(2).unwrap().as_str().parse().unwrap();
    let bx = cap.get(3).unwrap().as_str().parse().unwrap();
    let by = cap.get(4).unwrap().as_str().parse().unwrap();

    ((sx, sy), (bx, by))
}

fn main() {
    let sensors: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_sensor(&line.unwrap()))
        .collect();
    println!("sample {}", positions(&sensors, 10));
    println!("{}", positions(&sensors, 2000000));
    println!("sample {}", tuning_freq(&sensors, 20));
    println!("{}", tuning_freq(&sensors, 4000000));
}
