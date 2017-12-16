#![feature(slice_rotate)]

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

extern crate regex;
use regex::Regex;

fn spin(programs: &mut [char], n: usize) -> &[char] {
    let m = programs.len() - n;
    programs.rotate(m);
    programs
}

fn exchange(programs: &mut [char], a: usize, b: usize) -> &[char] {
    let (a, b) = (a.min(b), a.max(b));
    {
        let (left, right) = programs.split_at_mut(b);
        std::mem::swap(&mut left[a], &mut right[0]);
    }
    programs
}

fn partner(programs: &mut [char], a: char, b: char) -> &[char] {
    let ai = programs.iter().position(|&x| x == a).unwrap();
    let bi = programs.iter().position(|&x| x == b).unwrap();
    exchange(programs, ai, bi)
}

fn dance<'a>(programs: &'a mut [char], moves: &[&str]) -> &'a [char] {
    let re_s = Regex::new(r"s(\d+)").unwrap();
    let re_x = Regex::new(r"x(\d+)/(\d+)").unwrap();
    let re_p = Regex::new(r"p(.)/(.)").unwrap();
    for m in moves {
        if let Some(cap) = re_s.captures(m) {
            let n = cap.get(1).unwrap().as_str().parse().unwrap();
            spin(programs, n);
        } else if let Some(cap) = re_x.captures(m) {
            let a = cap.get(1).unwrap().as_str().parse().unwrap();
            let b = cap.get(2).unwrap().as_str().parse().unwrap();
            exchange(programs, a, b);
        } else if let Some(cap) = re_p.captures(m) {
            let a = cap.get(1).unwrap().as_str().chars().next().unwrap();
            let b = cap.get(2).unwrap().as_str().chars().next().unwrap();
            partner(programs, a, b);
        } else {
            unreachable!()
        }
    }

    programs
}

fn main() {
    let mut sample = ['a', 'b', 'c', 'd', 'e'];
    assert!(spin(&mut sample, 1) == ['e', 'a', 'b', 'c', 'd']);
    assert!(exchange(&mut sample, 3, 4) == ['e', 'a', 'b', 'd', 'c']);
    assert!(partner(&mut sample, 'e', 'b') == ['b', 'a', 'e', 'd', 'c']);
    sample = ['a', 'b', 'c', 'd', 'e'];
    assert!(dance(&mut sample, &["s1", "x3/4", "pe/b"]) == ['b', 'a', 'e', 'd', 'c']);

    let line = &BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap();
    let moves = line.split(',').collect::<Vec<_>>();
    let orig_programs = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'];
    let mut programs = orig_programs.clone();
    let mut history = Vec::new();
    // start from 1, not 0: comparison is after dancing "round" number of times
    for round in 1.. {
        history.push(programs.clone());
        println!("{:02} {}", round - 1, programs.iter().collect::<String>());

        dance(&mut programs, &moves);

        if programs == orig_programs {
            println!("cycle len: {} remainder: {} result: {}",
                     round,
                     1000000000 % round,
                     history[1000000000 % round].iter().collect::<String>());
            break;
        }
    }
}
