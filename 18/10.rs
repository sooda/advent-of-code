use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;

extern crate regex;
use regex::Regex;

fn parse_celestial_body(re: &Regex, line: &str) -> (i32, i32, i32, i32) {
    let cap = re.captures(line).unwrap();
    let x = cap.get(1).unwrap().as_str().parse().unwrap();
    let y = cap.get(2).unwrap().as_str().parse().unwrap();
    let dx = cap.get(3).unwrap().as_str().parse().unwrap();
    let dy = cap.get(4).unwrap().as_str().parse().unwrap();
    (x, y, dx, dy)
}

fn run(world: &mut [(i32, i32, i32, i32)]) {
    for b in world {
        b.0 += b.2;
        b.1 += b.3;
    }
}

fn back(world: &mut [(i32, i32, i32, i32)]) {
    for b in world {
        b.0 -= b.2;
        b.1 -= b.3;
    }
}

fn dump(world: &[(i32, i32, i32, i32)], i: u32) {
    let mut file = File::create(format!("10_frame_{:06}.pbm", i)).unwrap();
    let radius = 50;
    file.write_all(b"P1\n").unwrap();
    file.write_all(format!("{} {}\n", 2 * radius, 2 * radius).as_bytes()).unwrap();
    let cx = world.iter().map(|&b| b.0).sum::<i32>() / (world.len() as i32);
    let cy = world.iter().map(|&b| b.1).sum::<i32>() / (world.len() as i32);
    for y in -radius..radius {
        let mut line = Vec::new();
        for x in -radius..radius {
            if world.iter().find(|&b| b.0 - cx == x && b.1 - cy == y).is_some() {
                line.push(b'1');
            } else {
                line.push(b'0');
            }
            line.push(b' ');
        }
        line.push(b'\n');
        file.write_all(&line).unwrap();
    }
}

fn main() {
    let re = Regex::new(r"position=< *([\d\-]+), *([\d\-]+)> velocity=< *([\d\-]+), *([\d\-]+)>").unwrap();
    let mut bodies = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_celestial_body(&re, &x.unwrap())).collect::<Vec<_>>();
    let mut bb_prev = std::i32::MAX;
    for i in 1.. {
        run(&mut bodies);

        let bbox_x0 = bodies.iter().map(|&b| b.0).min().unwrap();
        let bbox_x1 = bodies.iter().map(|&b| b.0).max().unwrap();
        let bbox_y0 = bodies.iter().map(|&b| b.1).min().unwrap();
        let bbox_y1 = bodies.iter().map(|&b| b.1).max().unwrap();

        let bb_size = bbox_x1 - bbox_x0 + bbox_y1 - bbox_y0;
        let bb_delta = bb_size - bb_prev;
        bb_prev = bb_size;

        if bb_delta > 0 {
            // started to disperse; it's the previous frame
            back(&mut bodies);
            dump(&bodies, i - 1);
            break;
        }
    }
}
