use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use Dir::*;

impl From<char> for Dir {
    fn from(ch: char) -> Dir {
        match ch {
            '^' => Up,
            'v' => Down,
            '<' => Left,
            '>' => Right,
            _ => panic!()
        }
    }
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ch = match self {
            Up => '^',
            Down => 'v',
            Left => '<',
            Right => '>',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Cart {
    x: usize,
    y: usize,
    dir: Dir,
    turn_count: usize,
}

fn counterclockwise(d: Dir) -> Dir {
    match d {
        Up => Left,
        Left => Down,
        Down => Right,
        Right => Up
    }
}

fn clockwise(d: Dir) -> Dir {
    match d {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up
    }
}

fn slide(cart: &Cart, map: &[Vec<char>]) -> Cart {
    let (x, y) = match cart.dir {
        Up => (cart.x, cart.y - 1),
        Down => (cart.x, cart.y + 1),
        Left => (cart.x - 1, cart.y),
        Right => (cart.x + 1, cart.y),
    };

    // If these were just numbers, this would probably be a clever trick
    let dir = match (cart.dir, map[y][x]) {
        (Up, '/') => Right,
        (Up, '\\') => Left,
        (Down, '/') => Left,
        (Down, '\\') => Right,
        (Left, '/') => Down,
        (Left, '\\') => Up,
        (Right, '/') => Up,
        (Right, '\\') => Down,
        (_, '+') => {
            match cart.turn_count % 3 {
                0 => counterclockwise(cart.dir),
                1 => cart.dir,
                2 => clockwise(cart.dir),
                _ => unreachable!()
            }
        },
        _ => cart.dir,
    };

    let turn_count = if map[y][x] == '+' {
        cart.turn_count + 1
    } else {
        cart.turn_count
    };

    Cart { x: x, y: y, dir: dir, turn_count }
}

fn dump(map: &[Vec<char>], carts: &[Cart]) {
    for (y, row) in map.iter().enumerate() {
        for (x, track) in row.iter().enumerate() {
            if let Some(cart) = carts.iter().find(|c| c.x == x && c.y == y) {
                print!("\x1b[1;32;41m{}\x1b[0m", cart.dir);
            } else {
                print!("{}", track);
            }
        }
        println!("");
    }
    println!("");
}

fn step(map: &[Vec<char>], carts: &mut [Cart]) -> Option<(usize, usize)> {
    for i in 0..carts.len() {
        let new = slide(&carts[i], &map);
        let collision = carts.iter().enumerate()
            .filter(|&(j, _)| j != i)
            .any(|(_, c)| c.x == new.x && c.y == new.y);
        if collision {
            return Some((new.x, new.y))
        }
        carts[i] = new;
    }

    None
}

fn reorder(carts: &mut [Cart]) {
    carts.sort_unstable_by(|a, b| (a.y, a.x).cmp(&(b.y, b.x)));
}

fn play(map: &[Vec<char>], carts: &mut [Cart]) -> (usize, usize) {
    loop {
        dump(&map, carts);
        if let Some(collision) = step(&map, carts) {
            return collision;
        }
        reorder(carts);
    }
}

fn main() {
    let mut map = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| x.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut carts = Vec::new();
    for (y, row) in map.iter_mut().enumerate() {
        for (x, dir) in row.iter_mut().enumerate().filter(|(_, &mut b)| "<>v^".contains(b)) {
            carts.push(Cart { x: x, y: y, dir: Dir::from(*dir), turn_count: 0 });
            // note: none of the carts seems to start at an intersection
            *dir = match *dir {
                '<' | '>' => '-',
                '^' | 'v' => '|',
                _ => unreachable!()
            };
        }
    }
    let map = map; // mutated no more
    let endpos = play(&map, &mut carts);
    println!("{},{}", endpos.0, endpos.1);
}
