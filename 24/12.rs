use std::io::{self, BufRead};
use std::collections::HashSet;

type Data = char;
struct Map(Vec<Vec<Data>>);
type Pos = (i32, i32);

impl Map {
    fn new(v: Vec<Vec<Data>>) -> Self {
        Self(v)
    }
    fn w(&self) -> i32 {
        self.0[0].len() as i32
    }
    fn h(&self) -> i32 {
        self.0.len() as i32
    }
    fn at(&self, p: Pos) -> Option<Data> {
        if p.0 >= 0 && p.0 < self.w() && p.1 >= 0 && p.1 < self.h() {
            Some(self.0[p.1 as usize][p.0 as usize])
        } else {
            None
        }
    }
    fn iter(&self) -> impl Iterator<Item = (Pos, Data)> + '_ {
        self.0.iter().enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, &h)| ((x as i32, y as i32), h))
            })
    }
}

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn search(map: &Map, p: Pos, c: char, visits: &mut HashSet<Pos>) -> (usize, usize) {
    if !visits.insert(p) {
        return (0, 0);
    }

    let mut area = 1;
    let mut perimeter = 4;
    for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let neighpos = add(p, delta);
        if map.at(neighpos) == Some(c) {
            perimeter -= 1;
            let (next_area, next_perimeter) = search(map, neighpos, c, visits);
            area += next_area;
            perimeter += next_perimeter;
        }
    }
    (area, perimeter)
}

fn total_fencing_price(map: &Map) -> usize {
    let mut visits = HashSet::new();
    map.iter()
        .map(|(p, c)| search(map, p, c, &mut visits))
        .map(|(area, perimeter)| area * perimeter)
        .sum()
}

fn main() {
    let map = Map::new(io::stdin().lock().lines()
        .map(|line| line.unwrap()
             .chars().collect::<Vec<_>>()
            ).collect::<Vec<_>>());
    println!("{}", total_fencing_price(&map));
}
