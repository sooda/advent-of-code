use std::fs::File;
use std::io::Read;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

fn process(row: &str) {
    // current position
    let mut x = 0i32;
    let mut y = 0i32;
    // delta vector, each turn flips these somehow
    let mut dx = 0i32;
    let mut dy = 1i32;
    let mut visits = Vec::new();

    for op in row.split(", ") {
        let rotation = op.chars().next().unwrap();
        let travel = op[1..].parse::<i32>().unwrap();
        // "multiply" by a rotation matrix of 90 or -90 deg
        std::mem::swap(&mut dx, &mut dy);
        match rotation {
            'L' => { dx = -dx; }, // dx, dy = -dy,  dx;
            'R' => { dy = -dy; }, // dx, dy =  dy, -dx;
            _ => unreachable!()
        }
        for _ in 0..travel {
            x += dx;
            y += dy;
            if visits.contains(&(x, y)) {
                println!("double {} {} dist {}", x, y, x.abs() + y.abs());
            }
            visits.push((x, y));
        }
    }
    println!("x {}, y {} dist {}", x, y, x.abs() + y.abs());
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());
    for row in src.split("\n") {
        if row != "" { process(row); }
    }
}
