use std::io::{self, Read};

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
    fn at_mut(&mut self, p: Pos) -> Option<&mut Data> {
        if p.0 >= 0 && p.0 < self.w() && p.1 >= 0 && p.1 < self.h() {
            Some(&mut self.0[p.1 as usize][p.0 as usize])
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

fn gps(p: Pos) -> i32 {
    100 * p.1 + p.0
}

fn shift(map: &mut Map, robot: Pos, d: Pos) -> bool {
    let mut p = robot;
    while map.at(add(p, d)).unwrap() == 'O' {
        p = add(p, d);
    }
    if map.at(add(p, d)).unwrap() == '#' {
        false
    } else {
        // if no boxes, p == robot and this works out
        *map.at_mut(add(p, d)).unwrap() = 'O';
        *map.at_mut(add(robot, d)).unwrap() = '.';
        true
    }
}

fn simulate(mut map: Map, moves: &[char]) -> Map {
    let mut robot = map.iter().find(|&(_, ch)| ch == '@').unwrap().0;
    for m in moves {
        let d = match m {
            '^' => (0, -1),
            'v' => (0,  1),
            '<' => (-1, 0),
            '>' => ( 1, 0),
            _ => panic!()
        };
        if shift(&mut map, robot, d) {
            robot = add(robot, d);
        }
    }
    map
}

fn predict(mut map: Map, moves: &[char]) -> i32 {
    map = simulate(map, moves);
    map.iter()
        .filter(|&(_, ch)| ch == 'O')
        .map(|(p, _)| gps(p))
        .sum()
}

fn parse(file: &str) -> (Map, Vec<char>) {
    let mut sp = file.split("\n\n");
    let map = Map::new(sp.next().unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect());

    let mut moves = Vec::new();
    for line in sp.next().unwrap().lines() {
        moves.extend(line.chars());
    }
    (map, moves)
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let (map, moves) = parse(&file);
    println!("{:?}", predict(map, &moves));
}
