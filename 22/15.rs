use std::io::{self, BufRead};
use std::collections::HashSet;

extern crate regex;
use regex::Regex;

type Pos = (i64, i64);
// its position and the closest beacon position, as in the input
type Sensor = (Pos, Pos);

fn positions(sensors: &[Sensor], row: i64) -> usize {
    let mut spans = HashSet::new();
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
            for x in min..=max {
                spans.insert(x);
            }
        }
    }

    // multiples are ok
    for b in sensors.iter().filter(|&(_, b)| b.1 == row).map(|&(_, b)| b.0) {
        spans.remove(&b);
    }

    spans.len()
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
}
