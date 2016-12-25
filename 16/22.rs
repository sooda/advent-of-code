use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// rustc -L foo/deps 22 .rs
extern crate regex;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Node {
    x: usize,
    y: usize,
    size: u32,
    used: u32
}

fn parse(input: String) -> Option<Node> {
    let re_swappos = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+) +(\d+)T +(\d+)T +(\d+)T").unwrap();

    if let Some(cap) = re_swappos.captures(&input) {
        let x = cap.at(1).unwrap().parse().unwrap();
        let y = cap.at(2).unwrap().parse().unwrap();
        let size = cap.at(3).unwrap().parse().unwrap();
        let used = cap.at(4).unwrap().parse().unwrap();
        let avail = cap.at(5).unwrap().parse::<u32>().unwrap();
        assert!(size == used + avail);
        Some(Node { x: x, y: y, size: size, used: used })
    } else {
        None
    }
}

// oh fuk dis, can't have multiple mutable refs so let's index then
fn movedata(map: &mut Vec<Node>, dest: usize, src: usize) {
    map[dest].used += map[src].used;
    map[src].used = 0;
    // this assert is the whole point of part b, the route was solved manually
    assert!(map[dest].used <= map[dest].size);
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
    println!("viable {}", viable);

    let w = nodes.iter().map(|n| n.x).max().unwrap() + 1; // starts from 0
    let h = nodes.iter().map(|n| n.y).max().unwrap() + 1;
    let dummy = Node { x: 0, y: 0, size: 0, used: 0 };
    let mut map = vec![dummy; (w * h) as usize];
    let pos = |x, y| (y * w + x) as usize;
    for n in nodes {
        let ix = pos(n.x, n.y);
        map[ix] = n;
    }
    for y in 0..h {
        for x in 0..w {
            let n = &map[pos(x, y)];
            print!("{:3}/{:3} ", n.used, n.size);
        }
        println!("");
    }

    // start moving from the only empty place, get to the goal data, and move it to the magical
    // corner one "rotating" (down left left up right) step at a time
    let start = map.iter().position(|n| n.used == 0).unwrap();

    // this works for just my puzzle input, see 22.input.map
    let left = |(x, y)| [(x - 1, y)];
    let up = |(x, y)| [(x, y - 1)];
    let right = |(x, y)| [(x + 1, y)];
    let rotleft = |(x, y)| [(x, y + 1), (x - 1, y + 1), (x - 2, y + 1), (x - 2, y), (x - 1, y)];

    #[derive(Copy, Clone, Debug)]
    enum Op { Left, Up, Right, RotLeft }
    let dance: [&[Op]; 4] = [&[Op::Left; 16], &[Op::Up; 25], &[Op::Right; 29], &[Op::RotLeft; 32]];
    let dance = dance.iter().flat_map(|x| x.iter());

    let mut route = vec![(start % w, start / w)];

    for &op in dance {
        let curr = *route.last().unwrap();
        match op {
            Op::Left => route.extend_from_slice(&left(curr)),
            Op::Up => route.extend_from_slice(&up(curr)),
            Op::Right => route.extend_from_slice(&right(curr)),
            Op::RotLeft => route.extend_from_slice(&rotleft(curr))
        }
    }

    for part in route.windows(2) {
        // empty node moves from first to second, data goes the other way
        let data_dst = part[0];
        let data_src = part[1];
        println!("{:?} -> {:?}", data_src, data_dst);
        movedata(&mut map, pos(data_dst.0, data_dst.1), pos(data_src.0, data_src.1));
    }

    println!("{} steps", route.len() - 1);
}
