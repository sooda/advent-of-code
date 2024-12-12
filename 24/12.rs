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

fn right(p: Pos) -> Pos {
    (-p.1, p.0)
}

fn search(map: &Map, p: Pos, c: char, visits: &mut HashSet<Pos>, region: &mut HashSet<Pos>) -> (usize, usize) {
    if !visits.insert(p) {
        return (0, 0);
    }

    region.insert(p);

    let mut area = 1;
    let mut perimeter = 4;
    for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let neighpos = add(p, delta);
        if map.at(neighpos) == Some(c) {
            perimeter -= 1;
            let (next_area, next_perimeter) = search(map, neighpos, c, visits, region);
            area += next_area;
            perimeter += next_perimeter;
        }
    }
    (area, perimeter)
}

fn total_fencing_price(map: &Map) -> usize {
    let mut visits = HashSet::new();
    map.iter()
        .map(|(p, c)| search(map, p, c, &mut visits, &mut HashSet::new()))
        .map(|(area, perimeter)| area * perimeter)
        .sum()
}

// "flood fill" a horizontal line over each cell that doesn't have anything above it
fn count_top(pos: Pos, region: &HashSet<Pos>, visited: &mut HashSet<Pos>) -> usize {
    if !region.contains(&pos) {
        return 0;
    } else if !visited.insert(pos) {
        return 0;
    } else if region.contains(&add(pos, (0, -1))) {
        0
    } else {
        count_top(add(pos, (-1, 0)), &region, visited);
        count_top(add(pos, ( 1, 0)), &region, visited);
        1
    }
}

fn printregion(r: &HashSet<Pos>) {
    if r.is_empty() {
        return;
    }
    let minx = r.iter().map(|p| p.0).min().unwrap();
    let maxx = r.iter().map(|p| p.0).max().unwrap();
    let miny = r.iter().map(|p| p.1).min().unwrap();
    let maxy = r.iter().map(|p| p.1).max().unwrap();
    for y in miny..=maxy {
        for x in minx..=maxx {
            if r.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn rotate(region: HashSet<Pos>) -> HashSet<Pos> {
    region.into_iter().map(right).collect()
}

fn apply_discount(mut region: HashSet<Pos>) -> usize {
    let mut total = 0;
    for i in 0..4 {
        if i != 0 {
            region = rotate(region);
        }
        if false { printregion(&region); }
        let mut visited = HashSet::new();
        total += region.iter()
            .map(|&pos| count_top(pos, &region, &mut visited))
            .sum::<usize>();
    }
    total
}

fn total_fencing_price_discounted(map: &Map) -> usize {
    let mut visits = HashSet::new();
    map.iter()
        .map(|(pos, c)| {
            let mut region = HashSet::new();
            let (area, _) = search(map, pos, c, &mut visits, &mut region);
            area * apply_discount(region)
        })
        .sum()
}

fn main() {
    let map = Map::new(io::stdin().lock().lines()
        .map(|line| line.unwrap()
             .chars().collect::<Vec<_>>()
            ).collect::<Vec<_>>());
    println!("{}", total_fencing_price(&map));
    println!("{}", total_fencing_price_discounted(&map));
}
