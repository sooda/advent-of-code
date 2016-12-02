use std::fs::File;
use std::io::Read;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

fn process(row: &str, x0: i32, y0: i32) -> (i32, i32) {
    let mut x = x0;
    let mut y = y0;

    for op in row.chars() {
        match op {
            'L' => { x = std::cmp::max(x - 1, 0) },
            'R' => { x = std::cmp::min(x + 1, 2) },
            'U' => { y = std::cmp::max(y - 1, 0) },
            'D' => { y = std::cmp::min(y + 1, 2) },
            _ => unreachable!()
        }
    }
    println!("x {}, y {} key {}", x, y, y * 3 + x + 1);

    (x, y)
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());
    let mut xy = (1i32, 1i32);
    for row in src.split("\n") {
        if row != "" {
            xy = process(row, xy.0, xy.1);
        }
    }
}
