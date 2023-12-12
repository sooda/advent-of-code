use std::io::{self, BufRead};

type Spec = (Vec<char>, Vec<usize>);

fn try_broken_sequence(springs: &[char], counts: &[usize]) -> usize {
    let seq_len = counts[0];
    let pattern_fits = springs.iter().take_while(|&&ch| ch == '?' || ch == '#').count() >= seq_len;
    let fits_exactly = seq_len == springs.len() && counts.len() == 1;
    let no_mismatch = seq_len < springs.len() && springs[seq_len] != '#';
    if pattern_fits && fits_exactly {
        // last one
        1
    } else if pattern_fits && no_mismatch {
        // consume this sequence and force the next one operational
        arrangements(&springs[seq_len + 1..], &counts[1..])
    } else {
        // didn't work, prune
        0
    }
}

fn try_operational(springs: &[char], counts: &[usize]) -> usize {
    match springs.first() {
        None => 0,
        Some('#') => 0,
        Some('.') => {
            // trivial case
            arrangements(&springs[1..], counts)
        },
        Some('?') => {
            // try to fit one operational spring here
            arrangements(&springs[1..], counts)
        }
        _ => panic!()
    }
}

fn try_consume(springs: &[char], counts: &[usize]) -> usize {
    if counts.len() == 0 {
        try_operational(springs, counts)
    } else {
        try_broken_sequence(springs, counts) + try_operational(springs, counts)
    }
}

fn arrangements(springs: &[char], counts: &[usize]) -> usize {
    match springs.first() {
        None if counts.len() == 0 => 1, // happy end of a recursion
        None => 0,
        Some('.') => arrangements(&springs[1..], counts),
        Some('#') | Some('?') => try_consume(springs, counts),
        _ => panic!()
    }
}

fn sum_arrangements(rows: &[Spec]) -> usize {
    rows.iter().map(|r| arrangements(&r.0, &r.1)).sum::<usize>()
}

// .??..??...?##. 1,1,3
fn parse_row(row: &str) -> Spec {
    let mut sp = row.split(' ');
    let chs = sp.next().unwrap().chars().collect();
    let counts = sp.next().unwrap().split(',').map(|s| s.parse().unwrap()).collect();
    (chs, counts)
}

fn main() {
    let rows = io::stdin().lock().lines()
        .map(|row| parse_row(&row.unwrap()))
        .collect::<Vec<_>>();
    println!("{}", sum_arrangements(&rows));
}
