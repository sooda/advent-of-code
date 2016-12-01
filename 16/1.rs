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
    let mut x = 0;
    let mut y = 0;
    // delta vector, each turn flips these somehow
    let mut dx = 0;
    let mut dy = 1;

    for op in row.split(", ") {
        let rotation = op.chars().next().unwrap();
        let travel = &op[1..].parse::<i32>().unwrap();
        // "multiply" by a rotation matrix of 90 or -90 deg
        std::mem::swap(&mut dx, &mut dy);
        match rotation {
            'L' => { dx = -dx; }, // dx, dy = -dy,  dx;
            'R' => { dy = -dy; }, // dx, dy =  dy, -dx;
            _ => unreachable!()
        }
        x += dx * travel;
        y += dy * travel;
    }
    println!("x {}, y {} dist {}", x, y, x.abs() + y.abs());
}

fn main() {
    let src = readfile("1.input");
    for row in src.split("\n") {
        if row != "" { process(row); }
    }
}
