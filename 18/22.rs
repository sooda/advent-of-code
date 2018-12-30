use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;

fn geologic_index(x: usize, y: usize, tx: usize, ty: usize, map: &Vec<usize>) -> usize {
    if x == 0 && y == 0 {
        0
    } else if x == tx && y == ty {
        0
    } else if y == 0 {
        x * 16807
    } else if x == 0 {
        y * 48271
    } else {
        let a = map[(y - 1) * (tx + 1) + x];
        let b = map[y * (tx + 1) + x - 1];
        a * b
    }
}

fn expand_map(cave: &(usize, usize, usize)) -> Vec<usize> {
    let mut v = Vec::new();
    for y in 0..=cave.2 {
        for x in 0..=cave.1 {
            let index = geologic_index(x, y, cave.1, cave.2, &mut v);
            let erosion_level = (index + cave.0) % 20183;
            v.push(erosion_level);
        }
    }
    v
}

fn total_risk(cave: &(usize, usize, usize)) -> usize {
    let levels = expand_map(cave);
    levels.iter().map(|&erosion_level| erosion_level % 3).sum()
}

fn parse_cave(input: &mut Lines<BufReader<File>>) -> (usize, usize, usize) {
    /*
     * depth: 11739
     * target: 11,718
     */
    let depline = input.next().unwrap().unwrap();
    let depth = depline.split(" ").nth(1).unwrap().parse().unwrap();
    let tarline = input.next().unwrap().unwrap();
    let coords = tarline.split(" ").nth(1).unwrap();
    let mut xy = coords.split(",");
    let x = xy.next().unwrap().parse().unwrap();
    let y = xy.next().unwrap().parse().unwrap();
    (depth, x, y)
}

fn main() {
    let mut input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines();
    let cave = parse_cave(&mut input);
    println!("{:?}", cave);
    println!("{}", total_risk(&cave));
}
