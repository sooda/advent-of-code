use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn nth_number(start: &[usize], n: usize) -> usize {
    // numba -> (evenbefore, before)
    let mut spoken: HashMap<usize, (usize, usize)> = HashMap::new();
    for (turn, &x) in start.iter().enumerate() {
        // the evenbefore num doesn't matter here yet
        spoken.insert(x, (0, 1 + turn));
    }
    let mut last_spoken = *start.last().unwrap();
    let mut last_spoken_firstie = true;
    for turn in (1 + start.len())..=n {
        let speak_now = if last_spoken_firstie {
            0
        } else {
            let when = spoken.get(&last_spoken).unwrap();
            when.1 - when.0
        };
        let e = spoken.entry(speak_now);
        match e {
            Entry::Occupied(_) => last_spoken_firstie = false,
            Entry::Vacant(_) => last_spoken_firstie = true,
        }
        last_spoken = speak_now;
        //println!("turn {} spoken {} new {:?}", turn, last_spoken, last_spoken_firstie);
        let e = e.or_insert((0, 0));
        *e = (e.1, turn);
    }
    last_spoken
}

fn main() {
    let start: Vec<usize> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();
    println!("{}", nth_number(&start, 2020));
}
