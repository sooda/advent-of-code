use std::fs::File;
use std::io::Read;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

#[allow(dead_code)]
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

fn process_correct(row: &str, x0: i32, y0: i32) -> (i32, i32) {
    let mut x = x0;
    let mut y = y0;

    for op in row.chars() {
        // diamond-shaped keypad, so the limits are 2, 1..3, 0..4, 1..3, 2
        //     (2, 1, 0, 1, 2 and 2, 3, 4, 3, 2 for the two sides)
        //  y:  0  1  2  3  4
        //y-2: -2 -1  0  1  2
        match op {
            'L' => { x = std::cmp::max(x - 1, (y - 2).abs()) },
            'R' => { x = std::cmp::min(x + 1, 4 - (y - 2).abs()) },
            'U' => { y = std::cmp::max(y - 1, (x - 2).abs()) },
            'D' => { y = std::cmp::min(y + 1, 4 - (x - 2).abs()) },
            _ => unreachable!()
        }
        // println!("op {} x {} y {}", op, x, y);
    }
    let keys = "__1__X234X56789XABCX__D__";
    let idx = y * 5 + x;
    let key = keys.chars().nth(idx as usize).unwrap();
    println!("x {}, y {} key {}", x, y, key);

    (x, y)
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());
    let mut xy = (0i32, 2i32);
    for row in src.split("\n") {
        if row != "" {
            xy = process_correct(row, xy.0, xy.1);
        }
    }
}
