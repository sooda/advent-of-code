use std::io::{self, BufRead, Write};
use std::collections::{HashMap, VecDeque, HashSet};
use std::fs::File;

type Coord = (i32, i32);
type Map = HashMap<Coord, bool>;

fn sum(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1)
}

fn mul(a: i32, b: Coord) -> Coord {
    (a * b.0, a * b.1)
}

fn mod_(a: Coord, b: Coord) -> Coord {
    (a.0 % b.0, a.1 % b.1)
}

fn search(map: &Map, spos: Coord, steps: usize) -> usize {
    let mut fifo = VecDeque::new();
    fifo.push_back((spos, steps));
    let mut reached = 0;
    let move_deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut visited = HashSet::new();
    while let Some((pos, steps_remaining)) = fifo.pop_front() {
        if !visited.insert((pos, steps_remaining)) {
            continue;
        }
        if steps_remaining > 0 {
            for nextpos in move_deltas.iter().map(|&d| sum(pos, d)) {
                if let Some(false) = map.get(&nextpos) {
                    fifo.push_back((nextpos, steps_remaining - 1));
                }
            }
        } else {
            //println!("{:?}", pos);
            reached += 1;
        }
    }
    reached
}

fn print_ppm(name: &str, map: &Map, visited: &HashSet<(Coord, usize)>, i: usize) {
    //let visited: HashSet<Coord> = visited.iter().map(|&(p, _)| p).collect();
    let map_max = map.keys().fold((0, 0), |acc, p| (acc.0.max(p.0), acc.1.max(p.1)));
    let map_size = sum(map_max, (1, 1)); // first idx is 0th
    let visit_max = visited.iter().fold(map_max, |acc, (p, _)| (acc.0.max(p.0), acc.1.max(p.1)));
    let visit_min = visited.iter().fold((0, 0), |acc, (p, _)| (acc.0.min(p.0), acc.1.min(p.1)));
    let w = visit_max.0 - visit_min.0 + 1;
    let h = visit_max.1 - visit_min.1 + 1;
    let mut s = String::new();
    s.push_str(&format!("P3\n{} {}\n255\n", w, h));
    for y in visit_min.1..=visit_max.1 {
        for x in visit_min.0..=visit_max.0 {
            let visitpos = (x, y);
            let mappos = mod_(sum(visitpos, mul(1000, map_size)), map_size);
            if map.get(&mappos) == Some(&true) {
                s.push_str("255 0 0 ");
            } else if visited.contains(&(visitpos, i)) {
                s.push_str("0 0 0 ");
            } else {
                s.push_str("255 255 255 ");
            }
        }
        s.push_str("\n");
    }
    let mut file = File::create(name).unwrap();
    file.write_all(s.as_bytes()).unwrap();
}

fn search_b(map: &Map, spos: Coord, steps: usize) -> usize {
    let map_max = map.keys().fold((0, 0), |acc, p| (acc.0.max(p.0), acc.1.max(p.1)));
    let map_size = sum(map_max, (1, 1)); // first idx is 0th
    let mut fifo = VecDeque::new();
    fifo.push_back((spos, steps));
    let mut reached = 0;
    let move_deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut visited = HashSet::new();
    let mut frame = 0;
    while let Some((pos, steps_remaining)) = fifo.pop_front() {
        if !visited.insert((pos, steps_remaining)) {
            continue;
        }
        if steps_remaining != frame && steps_remaining != steps {
            frame = steps_remaining;
            if false {
                print_ppm(&format!("frame_{:0>8}.ppm", steps - frame), map, &visited, frame + 1);
            }
            let nextvisit = visited.iter().copied().filter(|(_, time)| *time == frame).collect();
            visited = nextvisit;
        }
        if steps_remaining > 0 {
            for nextpos in move_deltas.iter().map(|&d| sum(pos, d)) {
                // repeat-clamp to range with (a + b*N) % N
                let mappos = mod_(sum(nextpos, mul(1000, map_size)), map_size);
                if !map.get(&mappos).unwrap() {
                    fifo.push_back((nextpos, steps_remaining - 1));
                }
            }
        } else {
            reached += 1;
        }
    }
    reached
}

fn parse(lines: &[String]) -> (Map, Coord) {
    let map = lines.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as i32, y as i32), ch == '#'))
        })
    .collect();
    let spos = lines.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, ch)| ((x as i32, y as i32), ch == 'S'))
        })
    .find(|p| p.1).unwrap();

    (map, spos.0)
}

/*
 * y = a + b*x + c*x*x
 * y0 = a + 0 + 0      y0 = a
 * y1 = a + b*1 + c*1  y1 = a + b + c
 * y2 = a + b*2 + c*4  y2 = a + 2b + 4c
 *
 * a = y0
 * b = y1 - a - c
 *   = y1 - a - (y2 - a - 2b) / 4
 *   = y1 - y0 - y2/4 + y0/4 + 2b/4
 *   = y1 - 3/4 y0 - y2/4 + b/2
 *   = 2 y1 - 3/2 y0 - y2/2
 * c = (y2 - a - 2b) / 4
 *   = y2/4 - a/4 - b/2
 *   = y2/4 - y0/4 - (2 y1 - 3/2 y0 - y2/2)/2
 *   = y2/4 - y0/4 - y1 + 3/4 y0 + y2/4
 *   = y2/2 - y1 + y0/2
 */
fn search_infinite(map: &Map, spos: Coord, steps: usize) -> usize {
    let map_max = map.keys().fold((0, 0), |acc, p| (acc.0.max(p.0), acc.1.max(p.1)));
    assert!(map_max.0 == map_max.1);
    let size = (map_max.0 + 1) as usize; // 131
    let offset = steps % size;
    // FIXME search only once and track the mid results, this is wasteful
    let y0 = search_b(&map, spos, offset) as f64;
    let y1 = search_b(&map, spos, offset + size) as f64;
    let y2 = search_b(&map, spos, offset + 2 * size) as f64;
    let a = y0 as usize;
    let b = (2.0 * y1 - 3.0 / 2.0 * y0 - y2 / 2.0) as usize;
    let c = (y2 / 2.0 - y1 + y0 / 2.0) as usize;
    let x = steps / size; // 202300 * 131 + 65
    let y = a + b * x + c * x * x;
    y
}

fn main() {
    let lines = io::stdin().lock().lines()
        .map(|row| row.unwrap())
        .collect::<Vec<_>>();
    let (map, start) = parse(&lines);

    println!("{}", search(&map, start, 6));
    println!("{}", search(&map, start, 64));
    println!("{}", search_infinite(&map, start, 26501365));
}
