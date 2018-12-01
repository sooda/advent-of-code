use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let diffs = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines();
    let x = diffs.fold(0, |acc, x| acc + x.unwrap().parse::<i32>().unwrap());
    println!("{}", x);
}
