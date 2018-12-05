use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashSet;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

#[derive(Debug)]
enum What {
    Begin,
    Fall,
    Wake
}
use What::*;

#[derive(Debug)]
struct Record {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    guard: u32,
    what: What,
}


fn parse_line(re: &Regex, line: &str, prevguard: u32) -> Record {
    let cap = re.captures(line).unwrap();
    let yy = cap.get(1).unwrap().as_str().parse().unwrap();
    let mm = cap.get(2).unwrap().as_str().parse().unwrap();
    let dd = cap.get(3).unwrap().as_str().parse().unwrap();
    let hour = cap.get(4).unwrap().as_str().parse().unwrap();
    let min = cap.get(5).unwrap().as_str().parse().unwrap();
    let what = cap.get(6).unwrap().as_str();
    let (guard, what) = match what {
        "falls asleep" => (prevguard, Fall),
        "wakes up" => (prevguard, Wake),
        _ => (cap.get(7).unwrap().as_str().parse().unwrap(), Begin),
    };

    let x = Record { year: yy, month: mm, day: dd, hour: hour, min: min, guard: guard, what: what };
    x
}

fn worst_employee(log: &[Record]) -> u32 {
    let mut hm = HashMap::new();
    let guards = log.iter().map(|r| r.guard).collect::<HashSet<_>>();
    println!("{:?}", guards);

    let mut g = 0; // placeholder init
    let mut fell_minute = 0; // same
    for e in log {
        match e.what {
            Begin => {
                g = e.guard;
            }
            Fall => {
                fell_minute = e.min;
            }
            Wake => {
                let asleep = e.min - fell_minute;
                *hm.entry(g).or_insert(0) += asleep;
            }
        }
    }

    let (_, sleepy_man) = hm.iter().map(|(&k, &v)| (v, k)).max().unwrap();
    sleepy_man
}

fn sneaky_time(log: &[Record], sleepy_man: u32) -> u32 {
    let mut naps = vec![0; 60];
    let mut fell_minute = 0;
    for e in log.iter().filter(|e| e.guard == sleepy_man) {
        match e.what {
            Begin => {
            }
            Fall => {
                fell_minute = e.min;
            }
            Wake => {
                for m in fell_minute..e.min {
                    naps[m as usize] += 1;
                }
            }
        }
    }

    naps.iter().enumerate().map(|(i, &n)| (n, i)).max().unwrap().1 as u32
}

fn main() {
    let re = Regex::new(r"\[(\d\d\d\d)-(\d\d)-(\d\d) (\d\d):(\d\d)\] (Guard #(\d+) begins shift|falls asleep|wakes up)").unwrap();

    let mut scribble = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    scribble.sort();

    let log = scribble.iter().scan(0, |prev, entry| {
        let e = parse_line(&re, entry, *prev);
        *prev = e.guard;
        Some(e)
    }).collect::<Vec<_>>();

    let sleepy_man = worst_employee(&log);
    let minute = sneaky_time(&log, sleepy_man);

    println!("{:?}", sleepy_man * minute);
}
