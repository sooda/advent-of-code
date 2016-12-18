use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn next_tile(three: &[u8]) -> char {
    // ^^. or .^^ or ^.. or ..^
    if three[0] != three[2] {
        '^'
    } else {
        '.'
    }
}

fn next_row(current: &str) -> String {
    let current = current.as_bytes();
    let mut next = ".".to_owned();

    for i in 0..current.len()-2 {
        next.push(next_tile(&current[i..i+3]));
    }
    next.push('.');

    next
}

fn safe_tiles(first_row: &str, rows: usize) -> usize {
    let mut safes = 0;
    // add sentinel tiles for the borders; simpler than special cases for indices in next_tile
    let mut current = ".".to_owned() + &first_row + ".";
    for _ in 0..rows {
        if rows < 1000 { println!("{}", current); }
        safes += current.chars().filter(|&x| x == '.').count() - 2; // sentinels off
        let foo = next_row(&current);
        current = foo;
    }

    println!("{}", safes);
    safes
}

fn main() {
    assert!(safe_tiles(".^^.^.^^^^", 10) == 38);
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines().next().unwrap().unwrap();
    println!("{}", safe_tiles(&input, 40));
    println!("{}", safe_tiles(&input, 400000));
}

