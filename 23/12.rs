use std::io::{self, BufRead};
use std::collections::HashMap;

type Spec = (Vec<char>, Vec<usize>);

type Mem<'a> = HashMap<(&'a [char], &'a [usize]), usize>;

fn try_broken_sequence<'a>(springs: &'a [char], counts: &'a [usize], mem: &mut Mem<'a>) -> usize {
    let seq_len = counts[0];
    let pattern_fits = springs.iter().take_while(|&&ch| ch == '?' || ch == '#').count() >= seq_len;
    let fits_exactly = seq_len == springs.len() && counts.len() == 1;
    let no_mismatch = seq_len < springs.len() && springs[seq_len] != '#';
    if pattern_fits && fits_exactly {
        // last one
        1
    } else if pattern_fits && no_mismatch {
        // consume this sequence and force the next one operational
        arrangements(&springs[seq_len + 1..], &counts[1..], mem)
    } else {
        // didn't work, prune
        0
    }
}

fn try_operational<'a>(springs: &'a [char], counts: &'a [usize], mem: &mut Mem<'a>) -> usize {
    match springs.first() {
        None => 0,
        Some('#') => 0,
        Some('.') => {
            // trivial case
            arrangements(&springs[1..], counts, mem)
        },
        Some('?') => {
            // try to fit one operational spring here
            arrangements(&springs[1..], counts, mem)
        }
        _ => panic!()
    }
}

fn try_consume<'a>(springs: &'a [char], counts: &'a [usize], mem: &mut Mem<'a>) -> usize {
    if counts.len() == 0 {
        try_operational(springs, counts, mem)
    } else {
        try_broken_sequence(springs, counts, mem) + try_operational(springs, counts, mem)
    }
}

fn arrangements<'a>(springs: &'a [char], counts: &'a [usize], mem: &mut Mem<'a>) -> usize {
    if let Some(x) = mem.get(&(springs, counts)) {
        return *x;
    }
    let x = match springs.first() {
        None if counts.len() == 0 => 1, // happy end of a recursion
        None => 0,
        Some('.') => arrangements(&springs[1..], counts, mem),
        Some('#') | Some('?') => try_consume(springs, counts, mem),
        _ => panic!()
    };
    mem.insert((springs, counts), x);
    x
}

fn sum_arrangements(rows: &[Spec]) -> usize {
    rows.iter().map(|r| arrangements(&r.0, &r.1, &mut Mem::new())).sum::<usize>()
}

fn unfold(springs: &[char], counts: &[usize]) -> Spec {
    let springs = springs.iter()
        .copied()
        .chain(std::iter::once('?'))
        .cycle()
        .take(5 * springs.len() + 4)
        .collect::<Vec<_>>();
    let counts = counts.iter()
        .copied()
        .cycle()
        .take(5 * counts.len())
        .collect();
    (springs, counts)
}

fn unfold_rows(rows: &[Spec]) -> Vec<Spec> {
    rows.iter().map(|r| unfold(&r.0, &r.1)).collect()
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
    println!("{}", sum_arrangements(&unfold_rows(&rows)));
}
