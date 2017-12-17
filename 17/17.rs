use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn short_circuit_value(step: usize) -> usize {
    let mut buf = vec![0];
    let mut pos_after = 0;
    let mut prev = 0;

    for value in 1..2018 {
        buf.insert(pos_after + 1, value);
        prev = pos_after + 2;
        pos_after += 1 + step;
        pos_after %= buf.len();
    }

    buf[prev]
}

fn more_angry_value(step: usize) -> usize {
    let mut pos_after = 0;
    let mut len = 1;
    let mut after_0 = 0;

    for value in 1..50000000 {
        if pos_after == 0 {
            after_0 = value;
        }
        len += 1;
        pos_after += 1 + step;
        pos_after %= len;
    }

    after_0
}

fn main() {
    assert!(short_circuit_value(3) == 638);

    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap().parse().unwrap();
    println!("{}", short_circuit_value(input));
    println!("{}", more_angry_value(input));
}
