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

fn scramble(state: &str, input: &str, forward: bool) -> String {
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
            if forward {
                if leftright == "left" { rotleft(&mut state); } else { rotright(&mut state); }
            } else {
                if leftright == "right" { rotleft(&mut state); } else { rotright(&mut state); }
            }
        }
    } else if let Some(cap) = re_rotpos.captures(input) {
        let letter = cap.at(1).unwrap().as_bytes()[0];
        let pos = state.iter().position(|&ch| ch == letter).unwrap();
        if forward {
            let steps = 1 + pos + if pos >= 4 { 1 } else { 0 };
            for _ in 0..steps { rotright(&mut state); }
        } else {
            // bleh, not quite awake enough to figure out the exact inverse transform, so just
            // bruteforce one letter at a time. oh those many off-by-ones
            let pos = state.iter().position(|&ch| ch == letter).unwrap();
            let mut posnew = pos;
            loop {
                rotright(&mut state);
                posnew = (posnew + 1) % state.len();

                let steps_fwd = 1 + posnew + if posnew >= 4 { 1 } else { 0 };
                if (posnew + steps_fwd) % state.len() == pos {
                    break;
                }
            }
        }
    } else if let Some(cap) = re_reverse.captures(input) {
        let pos_a = cap.at(1).unwrap().parse::<usize>().unwrap();
        let pos_b = cap.at(2).unwrap().parse::<usize>().unwrap();
        &state[pos_a..pos_b+1].reverse();
    } else if let Some(cap) = re_move.captures(input) {
        let mut pos_from = cap.at(1).unwrap().parse::<usize>().unwrap();
        let mut pos_to = cap.at(2).unwrap().parse::<usize>().unwrap();
        if !forward {
            swap(&mut pos_from, &mut pos_to);
        }
        let letter = state.remove(pos_from);
        state.insert(pos_to, letter);
    } else {
        unreachable!()
    }

    String::from_utf8(state).unwrap()
}

fn main() {
    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines().map(Result::unwrap).collect::<Vec<_>>();
    //let mut sample = "abcde".to_owned();
    let mut puzzle = "abcdefgh".to_owned();
    let mut inverse = "fbgdceah".to_owned();
    for inp in input.iter() {
        //sample = scramble(&sample, &inp, true); // the puzzle input does not work with this
        puzzle = scramble(&puzzle, &inp, true);
    }
    //println!("{}", sample);
    println!("{}", puzzle);
    for inp in input.iter().rev() {
        inverse = scramble(&inverse, &inp, false);
    }
    println!("{}", inverse);
}


