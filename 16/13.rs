use std::fs::File;
use std::io::Read;

use std::collections::vec_deque::VecDeque;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

type Coord = i16;
type Depth = u32;
type State = u32;

fn state(x: Coord, y: Coord) -> State {
    ((x as State) << 16) | (y as State)
}

fn wall(x: Coord, y: Coord, favorite: Coord) -> bool {
    return (x*x + 3*x + 2*x*y + y + y*y + favorite).count_ones() & 1 == 1;
}

fn search(x0: Coord, y0: Coord, endx: Coord, endy: Coord, fav: Coord) -> Depth {
    let mut visited = Vec::new();
    visited.resize(1 << 20, 0u64);
    visited[(state(x0, y0) / 64) as usize] = 1 << (state(x0, y0) % 64);

    let mut queue = VecDeque::new();
    queue.push_back((0, x0, y0));

    let moves = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    while let Some((steps, x, y)) = queue.pop_front() {
        for &(dx, dy) in moves.iter() {
            if dy == -1 && y == 0 { continue; }
            if dx == -1 && x == 0 { continue; }

            let xx = x + dx;
            let yy = y + dy;
            let s = state(xx, yy);
            //println!("x {} y {} steps {} wall {}", xx, yy, steps, wall(xx, yy, fav));

            if !wall(xx, yy, fav) && visited[(s / 64) as usize] & (1 << (s % 64)) == 0 {
                //println!("ok");
                if xx == endx && yy == endy {
                    return steps + 1;
                }

                visited[(s / 64) as usize] |= 1 << (s % 64);
                queue.push_back((steps + 1, xx, yy));
            }
        }
    }
    unreachable!()
}

fn search_steps(x0: Coord, y0: Coord, fav: Coord, max: usize) -> u32 {
    let mut visited = Vec::new();
    visited.resize(1 << 20, 0u64);
    visited[(state(x0, y0) / 64) as usize] = 1 << (state(x0, y0) % 64);
    let mut positions = 1;

    let mut queue = VecDeque::new();
    queue.push_back((0, x0, y0));

    let moves = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    while let Some((steps, x, y)) = queue.pop_front() {
        for &(dx, dy) in moves.iter() {
            if dy == -1 && y == 0 { continue; }
            if dx == -1 && x == 0 { continue; }

            let xx = x + dx;
            let yy = y + dy;
            let s = state(xx, yy);

            if !wall(xx, yy, fav) && visited[(s / 64) as usize] & (1 << (s % 64)) == 0 && steps + 1 <= max {
                positions += 1;
                visited[(s / 64) as usize] |= 1 << (s % 64);
                queue.push_back((steps + 1, xx, yy));
            }
        }
    }

    positions
}

fn main() {
    // sample
    println!("{}", search(1, 1, 7, 4, 10));
    // input
    let favorite = readfile(&std::env::args().nth(1).unwrap()).trim().parse::<Coord>().unwrap();
    println!("{}", search(1, 1, 31, 39, favorite));
    // second half
    println!("{}", search_steps(1, 1, favorite, 50));
}
