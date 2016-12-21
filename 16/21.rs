use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::mem::swap;

// rustc -L foo/deps 21 .rs
extern crate regex;
use regex::Regex;

fn rotleft(s: &mut Vec<u8>) {
    let letter = s.remove(0);
    s.push(letter);
}

fn rotright(s: &mut Vec<u8>) {
    let letter = s.pop().unwrap();
    s.insert(0, letter);
}

fn scramble(state: &str, input: &str) -> String {
    let mut state = state.as_bytes().to_vec();

    let re_swappos = Regex::new(r"swap position (\d) with position (\d)").unwrap();
    let re_swapletter = Regex::new(r"swap letter (.) with letter (.)").unwrap();
    let re_rotsteps = Regex::new(r"rotate (left|right) (\d) steps?").unwrap();
    let re_rotpos = Regex::new(r"rotate based on position of letter (.)").unwrap();
    let re_reverse = Regex::new(r"reverse positions (\d) through (\d)").unwrap();
    let re_move = Regex::new(r"move position (\d) to position (\d)").unwrap();

    if let Some(cap) = re_swappos.captures(input) {
        let pos_a = cap.at(1).unwrap().parse::<usize>().unwrap();
        let pos_b = cap.at(2).unwrap().parse::<usize>().unwrap();
        if pos_a < state.len() && pos_b < state.len() {
            state.swap(pos_a, pos_b);
        }
    } else if let Some(cap) = re_swapletter.captures(input) {
        let letter_a = cap.at(1).unwrap().as_bytes()[0];
        let letter_b = cap.at(2).unwrap().as_bytes()[0];
        let pos_a = state.iter().position(|&ch| ch == letter_a).unwrap();
        let pos_b = state.iter().position(|&ch| ch == letter_b).unwrap();
        if pos_a < state.len() && pos_b < state.len() {
            state.swap(pos_a, pos_b);
        }
    } else if let Some(cap) = re_rotsteps.captures(input) {
        let leftright = cap.at(1).unwrap();
        let steps = cap.at(2).unwrap().parse::<usize>().unwrap();
        for _ in 0..steps {
            if leftright == "left" { rotleft(&mut state); } else { rotright(&mut state); }
        }
    } else if let Some(cap) = re_rotpos.captures(input) {
        let letter = cap.at(1).unwrap().as_bytes()[0];
        let pos = state.iter().position(|&ch| ch == letter).unwrap();
        let steps = 1 + pos + if pos >= 4 { 1 } else { 0 };
        for _ in 0..steps { rotright(&mut state); }
    } else if let Some(cap) = re_reverse.captures(input) {
        let pos_a = cap.at(1).unwrap().parse::<usize>().unwrap();
        let pos_b = cap.at(2).unwrap().parse::<usize>().unwrap();
        &state[pos_a..pos_b+1].reverse();
    } else if let Some(cap) = re_move.captures(input) {
        let pos_from = cap.at(1).unwrap().parse::<usize>().unwrap();
        let pos_to = cap.at(2).unwrap().parse::<usize>().unwrap();
        let letter = state.remove(pos_from);
        state.insert(pos_to, letter);
    } else {
        unreachable!()
    }

    String::from_utf8(state).unwrap()
}

fn main() {
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines().map(Result::unwrap);
    let mut sample = "abcde".to_owned();
    let mut puzzle = "abcdefgh".to_owned();
    for inp in input {
        //sample = scramble(&sample, &inp); // the puzzle input does not work with this
        puzzle = scramble(&puzzle, &inp);
    }
    println!("{}", sample);
    println!("{}", puzzle);
}


