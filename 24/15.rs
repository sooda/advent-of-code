use std::io::{self, Read};
use std::collections::{VecDeque, HashSet};
use std::ops::{Index, IndexMut};

type Data = char;
#[derive(Clone)]
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
    fn at(&self, p: Pos) -> Option<&Data> {
        if p.0 >= 0 && p.0 < self.w() && p.1 >= 0 && p.1 < self.h() {
            Some(&self.0[p.1 as usize][p.0 as usize])
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

impl Index<Pos> for Map {
    type Output = Data;
    fn index(&self, p: Pos) -> &Self::Output {
        self.at(p).unwrap()
    }
}

impl IndexMut<Pos> for Map {
    fn index_mut(&mut self, p: Pos) -> &mut Data {
        self.at_mut(p).unwrap()
    }
}

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

fn neg(a: Pos) -> Pos {
    (-a.0, -a.1)
}

fn gps(p: Pos) -> i32 {
    100 * p.1 + p.0
}

// This is just a special case of the vertical graph business; with more clever edge connectivity,
// could maybe use the same bfs business for both.
fn shiftbig_horiz(map: &mut Map, robot: Pos, d: Pos) -> bool {
    let mut p = robot;
    while map[add(p, d)] == '[' || map[add(p, d)] == ']' {
        p = add(p, d);
    }
    if map[add(p, d)] == '#' {
        false
    } else {
        assert_eq!(map[add(p, d)], '.');
        // now the boxes look different so have to memmove; maybe could do better if this was
        // something more structured than ascii art and each box was just one element in a map, but
        // even then the whole row of boxes should move by "0.5 box unit".
        let d2 = neg(d);
        while p != add(robot, d2) {
            map[add(p, d)] = map[p];
            p = add(p, d2);
        }
        true
    }
}

// This could be fun to absolutely overengineer with some async-await parallelism where each
// early-ending leaf node of the search graph would wait for the whole move to complete, and then
// maybe act. And then see how much slower it is with the parallel overhead, and then imagine a
// suitable processor to run it better.
fn shiftbig_vert(map: &mut Map, robot: Pos, d: Pos) -> bool {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut moves = Vec::new();

    queue.push_back(robot);

    while let Some(p) = queue.pop_front() {
        if !visited.insert(p) {
            continue;
        }
        moves.push(p);
        let destpos = add(p, d);
        let destcell = map[destpos];
        if destcell == '#' {
            return false;
        } else if destcell == '.' {
            // nop
        } else if destcell == '[' {
            queue.push_back(destpos);
            queue.push_back(add(destpos, (1, 0)));
        } else if destcell == ']' {
            queue.push_back(destpos);
            queue.push_back(add(destpos, (-1, 0)));
        } else {
            panic!()
        }
    }
    // Or just could collect the visit map into a vec and sort that by y coord.
    while let Some(p) = moves.pop() {
        map[add(p, d)] = map[p];
        // this is useless for all but the edge, makes it easy though
        map[p] = '.';
    }
    true
}

fn shiftbig(map: &mut Map, robot: Pos, d: Pos) -> bool {
    if d.1 == 0 {
        shiftbig_horiz(map, robot, d)
    } else {
        shiftbig_vert(map, robot, d)
    }
}

fn shift(map: &mut Map, robot: Pos, d: Pos) -> bool {
    let mut p = robot;
    while map[add(p, d)] == 'O' {
        p = add(p, d);
    }
    if map[add(p, d)] == '#' {
        false
    } else if map[add(p, d)] == '.' {
        // if no boxes, p == robot and this works out
        map[add(p, d)] = 'O';
        map[add(robot, d)] = '.';
        true
    } else {
        shiftbig(map, robot, d)
    }
}

fn dump(map: &Map, r: Pos) {
    for (y, row) in map.0.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            print!("{}", if (x as i32, y as i32) != r { ch } else { assert_eq!(ch, '.'); '@' });
        }
        println!();
    }
    println!();
}

fn simulate(mut map: Map, moves: &[char]) -> Map {
    let mut robot = map.iter().find(|&(_, ch)| ch == '@').unwrap().0;
    map[robot] = '.';
    for m in moves {
        if false {
            dump(&map, robot);
        }
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
        .filter(|&(_, ch)| ch == 'O' || ch == '[')
        .map(|(p, _)| gps(p))
        .sum()
}

fn widen_warehouse(map: Map) -> Map {
    let mut wide = Vec::new();
    for row in map.0 {
        let mut r = Vec::new();
        for ch in row {
            r.extend(match ch {
                '#' => ['#', '#'],
                'O' => ['[', ']'],
                '.' => ['.', '.'],
                '@' => ['@', '.'],
                _ => panic!()
            });
        }
        wide.push(r);
    }
    Map(wide)
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
    println!("{:?}", predict(map.clone(), &moves));
    println!("{:?}", predict(widen_warehouse(map), &moves));
}
