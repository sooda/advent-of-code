use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn parse_rule(line: &str) -> Option<[u8; 5]> {
    let line = line.as_bytes();
    if line[line.len() - 1] == b'.' {
        None
    } else {
        let mut a = [0; 5];
        a.copy_from_slice(&line[0..5]);
        Some(a)
    }
}

fn spread(pots: &Vec<u8>, rules: &[[u8; 5]]) -> Vec<u8> {
    let mut next = Vec::with_capacity(pots.len());

    next.push(b'.');
    next.push(b'.');
    for i in 2..=pots.len()-3 {
        let current = &pots[i-2..=i+2];
        // cannot .contains()
        if rules.iter().find(|&r| r == current).is_some() {
            next.push(b'#');
        } else {
            next.push(b'.');
        }
    }
    next.push(b'.');
    next.push(b'.');

    // didn't overflow the padding
    assert!(next[2] == b'.');
    assert!(next[pots.len() - 3] == b'.');

    next
}

fn sum_planted_pots(mut pots: Vec<u8>, rules: &[[u8; 5]], rounds: usize) -> i32 {
    let pad = pots.len();
    // Add padding - no way it can grow this much. Now 0th is pots[pad]
    for _ in 0..pad {
        pots.push(b'.');
        pots.insert(0, b'.');
    }

    for _ in 0..rounds {
        let next = spread(&pots, &rules);
        println!("{}", String::from_utf8_lossy(&next));
        pots = next;
    }

    pots.iter().enumerate()
        .map(|(i, &pot)| if pot == b'#' { (i - pad) as i32 } else { 0 })
        .sum::<i32>()
}

fn main() {
    let mut lines = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines();

    let first = lines.next().unwrap().unwrap();
    let init_state_str = first.split("initial state: ").nth(1).unwrap();

    lines.next(); // empty separator
    let rules = lines.filter_map(|line| parse_rule(&line.unwrap())).collect::<Vec<_>>();

    let pots = init_state_str.bytes().collect::<Vec<_>>();
    println!("{}", sum_planted_pots(pots, &rules, 20));
}
