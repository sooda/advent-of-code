use std::io::{self, BufRead};

fn slide_north(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let w = map[0].len();
    let h = map.len();
    for x in 0..w {
        let mut free_start = 0;
        loop {
            let free_space = map.iter().skip(free_start).map(|r| r[x]).position(|ch| ch == '.');
            if let Some(ystart_off) = free_space {
                let mut moved = false;
                let ystart = free_start + ystart_off;
                // FIXME this could be more readable in iterator form
                for yi in (ystart+1)..h {
                    let ch = map[yi][x];
                    if ch == '#' {
                        break;
                    } else if ch == 'O' {
                        map[yi][x] = '.';
                        map[ystart][x] = 'O';
                        moved = true;
                        break;
                    }
                }
                if !moved {
                    free_start += 1;
                }
            } else {
                break;
            }
        }
    }
    map
}

fn total_north_load(map: &[Vec<char>]) -> usize {
    let h = map.len();
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter().map(move |&ch| {
                if ch == 'O' { h - y } else { 0 }
            }).sum::<usize>()
        })
        .sum()
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
        let _ncycles = (end - start) / self.0;
        let remaining = (end - start) % self.0;

        self.1.iter().copied().take(remaining+1).last().unwrap()
    }
}

fn clockwise(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map2 = map.clone();
    let w = map[0].len();
    for (y, row) in map.iter().enumerate() {
        for (x, ch) in row.iter().enumerate() {
            // turns out it's a square
            map2[x][w - 1 - y] = *ch;
        }
    }
    map2
}

fn spin(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for _ in 0..4 {
        map = slide_north(map);
        map = clockwise(map);
    }
    map
}

fn after_spin_cycles(mut map: Vec<Vec<char>>, ncycles: usize) -> i64 {
    let mut signal = CycledSignal::new();
    for i in 1..ncycles {
        map = spin(map);
        if let Some(cycle) = signal.feed_and_test(total_north_load(&map) as i64) {
            return cycle.extrapolate(i + 1, ncycles);
        }
    }
    panic!()
}

fn main() {
    let map = io::stdin().lock().lines()
        .map(|row| row.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{}", total_north_load(&slide_north(map.clone())));
    println!("{}", after_spin_cycles(map, 1_000_000_000));
}
