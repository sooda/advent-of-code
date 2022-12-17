use std::io::{self, BufRead};
use std::collections::HashSet;

// pixels and max height
struct Map(HashSet<(i64, i64)>, i64);

const WIDE: i64 = 7;
const LAUNCH_X: i64 = 2;
const LAUNCH_Y: i64 = 4;

fn render(map: &HashSet<(i64, i64)>) {
    let miny = map.iter().map(|&(_, y)| y).min().unwrap();
    let maxy = map.iter().map(|&(_, y)| y).max().unwrap();
    assert_eq!(miny, 1);
    for y in (1..=maxy).rev() {
        print!("|");
        for x in 0..WIDE {
            print!("{}", if map.contains(&(x, y)) { '#' } else { '.' });
        }
        println!("|");
    }
    println!("+-------+");
}

fn drop<D: Iterator<Item=i64>>(map: &mut Map, shape: &[(i64, i64)], dirs: &mut D) -> i64 {
    let shapewid = shape.iter().map(|&(x, _)| x).max().unwrap() - 0 + 1; // min always 0
    let shapehei = shape.iter().map(|&(_, y)| y).max().unwrap() - 0 + 1; // min always 0
    let (mut x, mut y) = (LAUNCH_X, map.1 + LAUNCH_Y);
    let rocks_ok = |nx, ny| shape.iter().all(|&(sx, sy)| !map.0.contains(&(nx + sx, ny + sy)));
    loop {
        // move left or right one step
        let dx = dirs.next().unwrap();
        let dy = 0;
        let edges_ok = x + dx >= 0 && x + dx + shapewid <= WIDE;
        if edges_ok && rocks_ok(x + dx, y + dy) {
            x += dx;
            y += dy;
        }

        // If I take one more step, I'll be the farthest away from home I've ever been.
        let dx = 0;
        let dy = -1;
        let edges_ok = y + dy > 0;
        if edges_ok && rocks_ok(x + dx, y + dy) {
            x += dx;
            y += dy;
        } else {
            break;
        }
    }

    for &(sx, sy) in shape.iter() {
        map.0.insert((x + sx, y + sy));
    }
    let m1 = map.1;
    map.1 = map.1.max(y + shapehei - 1);
    map.1 - m1
}

struct CycledSignal<T>(Vec<T>);

struct Cycle<'a, T>(usize, &'a [T]);

impl<T: Eq>
CycledSignal<T> {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn test(&self, window: usize, n: usize) -> bool {
        if window * n == 0 || self.0.len() < window * n {
            false
        } else {
            // sanity check: len 8, 4 windows of size 2 each, windows begin at 0
            let begin = &self.0[self.0.len() - n * window..];
            (0..n-1).all(|i| {
                let left  = &begin[(i+0) * window..(i+1) * window];
                let right = &begin[(i+1) * window..(i+2) * window];
                left == right
            })
        }
    }

    fn feed_and_test<'a>(&'a mut self, v: T) -> Option<Cycle<'a, T>> {
        // now a cycle, if any, ends at this index; another begins at len()
        self.0.push(v);
        // test at least a few cycles to be sure, maybe a big cycle includes subcycles?
        // also ignore the first half to allow the signal to settle
        let four = (self.0.len() / 2) / 4;
        let three = (self.0.len() / 2) / 3;
        (four..three)
            .find(|&len| self.test(len, 3))
            .map(|len| Cycle(len, &self.0[self.0.len() - len ..]))
    }
}

impl<'a, T: Copy + From<i64> + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + std::iter::Sum>
Cycle<'a, T> {
    // the part between [start, end]
    fn extrapolate(&'a self, start: usize, end: usize) -> T {
        let ncycles = (end - start) / self.0;
        let remaining = (end - start) % self.0;

        T::from(ncycles as i64) * self.1.iter().copied().sum::<T>()
            + self.1.iter().copied().take(remaining).sum::<T>()
    }
}

fn end_height(directions: &[i64], rocks_limit: usize) -> i64 {
    // left and bottom is 0; x goes right, y goes up
    let shapes: [&[(i64, i64)]; 5] = [
        &[
            (0, 0), (1, 0), (2, 0), (3, 0)
        ],
        &[
                    (1, 2),
            (0, 1), (1, 1), (2, 1),
                    (1, 0)
        ],
        &[
                            (2, 2),
                            (2, 1),
            (0, 0), (1, 0), (2, 0),
        ],
        &[
            (0, 3),
            (0, 2),
            (0, 1),
            (0, 0),
        ],
        &[
            (0, 1), (1, 1),
            (0, 0), (1, 0),
        ],
    ];

    let mut map = Map(HashSet::new(), 0);
    let mut dirs = directions.iter().copied().cycle();
    let mut signal = CycledSignal::new();

    for (i, shape) in (0..rocks_limit).zip(shapes.iter().cycle()) {
        // signal for the cycle: number of height increments per iteration
        let height_increase = drop(&mut map, shape, &mut dirs);
        if let Some(cycle) = signal.feed_and_test(height_increase) {
            return map.1 + cycle.extrapolate(i + 1, rocks_limit);
        }

        if false {
            render(&map.0);
            println!();
        }
    }

    map.1
}

fn main() {
    let directions: Vec<i64> = io::stdin().lock().lines().next().unwrap().unwrap()
        .as_bytes().iter().map(|b| {
            match b {
                b'<' => -1,
                b'>' => 1,
                _ => panic!()
            }
        })
        .collect();
    println!("{}", end_height(&directions, 2022));
    println!("{}", end_height(&directions, 1000000000000));
}
