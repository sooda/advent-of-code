use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// rustc -L foo/deps 17.rs
extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

use std::collections::vec_deque::VecDeque;

#[derive(Debug, Clone, PartialEq)]
enum Turn {
    Start,
    Up,
    Down,
    Left,
    Right
}
use Turn::*;

impl Turn {
    fn ch(&self) -> char {
        match *self {
            Start => ' ',
            Up => 'U',
            Down => 'D',
            Left => 'L',
            Right => 'R'
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    x: i8,
    y: i8,
    turn: Turn,
    path: String
}

impl State {
    fn new(x: i8, y: i8, turn: Turn, path: &str) -> Self {
        State { x: x, y: y, turn: turn, path: path.to_owned() }
    }
}

fn valid_moves(passcode: &str, pos: &State) -> [Option<(i8, i8, Turn)>; 4] {
    let mut md5 = Md5::new();
    let input = passcode.to_owned() + &pos.path;
    md5.input(input.as_bytes());

    let mut out = [0u8; 16];
    md5.result(&mut out);
    println!("  validity {:?} {:?}", out, input);

    [
        if out[0] & 0xf0 >= 0xb0 && pos.y > 0 { Some((pos.x, pos.y - 1, Up))    } else { None },
        if out[0] & 0x0f >= 0x0b && pos.y < 3 { Some((pos.x, pos.y + 1, Down))  } else { None },
        if out[1] & 0xf0 >= 0xb0 && pos.x > 0 { Some((pos.x - 1, pos.y, Left))  } else { None },
        if out[1] & 0x0f >= 0x0b && pos.x < 3 { Some((pos.x + 1, pos.y, Right)) } else { None },
    ]
}

fn path(passcode: &str) -> String {
    let root = State::new(0, 0, Start, "");
    let mut history = vec![root.clone()];
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(current) = queue.pop_front() {
        println!("current {:?}", current);
        let moves = valid_moves(passcode, &current);
        println!("  valids {:?}", moves);
        for next in moves.iter().filter_map(|x| x.clone()) {
            let mut p = current.path.clone(); p.push(next.2.ch());
            let state = State::new(next.0, next.1, next.2, &p);
            if history.contains(&state) {
                continue;
            }
            if next.0 == 3 && next.1 == 3 {
                return state.path;
            }
            println!("  push {:?}", state);
            queue.push_back(state.clone());
            history.push(state.clone());
        }
    }
    unreachable!()
}

fn main() {
    // path("hijkl");
    assert!(path("ihgpwlah") == "DDRRRD");
    assert!(path("kglvqrro") == "DDUDRLRRUDRD");
    assert!(path("ulqzkmiv") == "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines().next().unwrap().unwrap();
    println!("{}", path(&input));
}
