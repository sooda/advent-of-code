use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
struct Bot {
    x: i64,
    y: i64,
    z: i64,
    r: i64
}

fn parse_line(input: &str) -> Bot {
    // pos=<-39857152,26545464,51505035>, r=86328482
    let pos = input.split("pos=<").nth(1).unwrap().split(">,").nth(0).unwrap();
    let mut coords = pos.split(",");
    let x = coords.next().unwrap().parse().unwrap();
    let y = coords.next().unwrap().parse().unwrap();
    let z = coords.next().unwrap().parse().unwrap();
    let r = input.split("r=").nth(1).unwrap().parse().unwrap();
    Bot { x: x, y: y, z: z, r: r }
}

fn in_range(a: &Bot, pos: (i64, i64, i64)) -> bool {
    (a.x - pos.0).abs() + (a.y - pos.1).abs() + (a.z - pos.2).abs() <= a.r
}

fn biggest_contains(bots: &[Bot]) -> usize {
    let biggest = bots.iter().max_by(|&a, &b| a.r.cmp(&b.r)).unwrap();
    bots.iter().filter(|b| in_range(&biggest, (b.x, b.y, b.z))).count()
}

fn main() {
    let bots = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|l| parse_line(&l.unwrap())).collect::<Vec<_>>();
    println!("{:?}", biggest_contains(&bots));
}
