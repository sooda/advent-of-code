use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn steps_away(path: &[&str]) -> i32 {
    let (mut x, mut y) = (0i32, 0i32);
    for &turn in path {
        let (dx, dy) = match turn {
            "n" => (0, 1),
            "ne" => (1, 0),
            "se" => (1, -1),
            "s" => (0, -1),
            "sw" => (-1, 0),
            "nw" => (-1, 1),
            _ => unreachable!()
        };
        x += dx;
        y += dy;
    }
    // this seems to work on pen & paper
    println!("{} {} {}", x.abs(), y.abs(), (-x - y).abs());
    std::cmp::max(std::cmp::max(x.abs(), y.abs()), (-x - y).abs())
}

fn main() {
    assert!(steps_away(&"ne,ne,ne".split(",").collect::<Vec<_>>()) == 3);
    assert!(steps_away(&"ne,ne,sw,sw".split(",").collect::<Vec<_>>()) == 0);
    assert!(steps_away(&"ne,ne,s,s".split(",").collect::<Vec<_>>()) == 2);
    assert!(steps_away(&"se,sw,se,sw,sw".split(",").collect::<Vec<_>>()) == 3);
    let line = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap();
    let path = line.split(",").collect::<Vec<_>>();
    println!("{}", steps_away(&path));
}
