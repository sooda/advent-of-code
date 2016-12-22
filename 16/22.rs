use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// rustc -L foo/deps 22 .rs
extern crate regex;
use regex::Regex;

struct Node {
    x: u32,
    y: u32,
    size: u32,
    used: u32
}

fn parse(input: String) -> Option<Node> {
    let re_swappos = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+) +(\d+)T +(\d+)T +(\d+)T").unwrap();

    if let Some(cap) = re_swappos.captures(&input) {
        let x = cap.at(1).unwrap().parse::<u32>().unwrap();
        let y = cap.at(2).unwrap().parse::<u32>().unwrap();
        let size = cap.at(3).unwrap().parse::<u32>().unwrap();
        let used = cap.at(4).unwrap().parse::<u32>().unwrap();
        let avail = cap.at(5).unwrap().parse::<u32>().unwrap();
        assert!(size == used + avail);
        Some(Node { x: x, y: y, size: size, used: used })
    } else {
        None
    }
}

fn main() {
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines().map(Result::unwrap);
    let nodes = input.filter_map(parse).collect::<Vec<_>>();
    let mut viable = 0;
    for (i, a) in nodes.iter().enumerate() {
        for (j, b) in nodes.iter().enumerate() {
            if i != j && a.used > 0 && a.used <= b.size - b.used {
                viable += 1;
            }
        }
    }
    println!("{}", viable);
}
