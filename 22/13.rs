use std::io::{self, Read};
use std::str;
use std::cmp::Ordering;

enum CompareResult<'a, 'b> {
    Less,
    Greater,
    Equal(&'a [u8], &'b [u8]),
}

// this always gets one result, never breaks the recursion
fn compare_proxy<'a, 'b>(l: &'a [u8], r: &'b [u8]) -> CompareResult<'a, 'b> {
    match (l[0], r[0]) {
        (b']', b']') => {
            // both ended, validity unknown
            CompareResult::Equal(&l[1..], &r[1..])
        },
        (b']', _) => {
            // left ran out
            CompareResult::Less
        },
        (_, b']') => {
            // right ran out
            CompareResult::Greater
        },
        (b'0' ..= b':', b'0' ..= b':') => {
            // both are numbers
            match l[0].cmp(&r[0]) {
                Ordering::Less => CompareResult::Less,
                Ordering::Greater => CompareResult::Greater,
                _ => CompareResult::Equal(&l[1..], &r[1..]),
            }
        },
        (b',', b',') => {
            // both lists continue
            CompareResult::Equal(&l[1..], &r[1..])
        },
        (b'[', b'[') => {
            // both contain a list; read and skip over
            compare_list(&l[1..], &r[1..])
        },
        (b'[', b'0' ..= b':') => {
            // l list, r number
            let rr = [r[0], b']']; // can't be inline in call because technically matched below
            let result = compare_list(&l[1..], &rr);
            match result {
                // proxy the r back
                CompareResult::Equal(lnew, _) => CompareResult::Equal(lnew, &r[1..]),
                // "returns a value referencing data owned by the current function"
                // no, rustc, rr is not in these enumerators
                CompareResult::Less => CompareResult::Less,
                CompareResult::Greater => CompareResult::Greater,
            }
        },
        (b'0' ..= b':', b'[') => {
            // l number, r list
            let ll = [l[0], b']'];
            let result = compare_list(&ll, &r[1..]);
            match result {
                CompareResult::Equal(_, rnew) => CompareResult::Equal(&l[1..], rnew),
                CompareResult::Less => CompareResult::Less,
                CompareResult::Greater => CompareResult::Greater,
            }
        },
        _ => panic!("what? {} {}", l[0] as char, r[0] as char),
    }
}

// left smaller than right?
// the [ of l[0] and r[0] already consumed
fn compare_list<'a, 'b>(mut l: &'a [u8], mut r: &'b [u8]) -> CompareResult<'a, 'b> {
    while l.len() > 0 && r.len() > 0 {
        match compare_proxy(l, r) {
            lt @ CompareResult::Less => return lt,
            CompareResult::Equal(lnew, rnew) => { l = lnew; r = rnew; },
            gt @ CompareResult::Greater => return gt,
        }
    }

    CompareResult::Equal(l, r)
}

fn right_order(l: &[u8], r: &[u8]) -> bool {
    assert!(l[0] == b'[');
    assert!(r[0] == b'[');
    match compare_list(&l[1..], &r[1..]) {
        CompareResult::Less => true,
        CompareResult::Equal(..) => panic!("silence! order!"),
        CompareResult::Greater => false,
    }
}

fn right_order_sum(pairs: &[(String, String)]) -> usize {
    pairs.iter().enumerate()
        .filter(|(_, p)| right_order(p.0.as_bytes(), p.1.as_bytes()))
        .map(|(i, _)| i + 1)
        .sum()
}

fn decoder_key(pairs: &[(String, String)]) -> usize {
    let dividers = &[ "[[2]]", "[[6]]" ];
    let mut v: Vec<&str> = pairs.iter()
        .flat_map(|p| std::iter::once(&p.0 as &str).chain(std::iter::once(&p.1 as &str)))
        .collect();
    v.extend(dividers.iter());
    v.sort_unstable_by(|l, r| {
        match compare_list(&l.as_bytes()[1..], &r.as_bytes()[1..]) {
            CompareResult::Less => Ordering::Less,
            CompareResult::Equal(..) => Ordering::Equal,
            CompareResult::Greater => Ordering::Greater,
        }
    });

    dividers.iter()
        .map(|div| v.iter().position(|a| a == div).unwrap() + 1)
        .fold(1, |acc, x| acc * x)
}

fn parse_pairs(input: &str) -> Vec<(String, String)> {
    input.split("\n\n").map(|i| {
        let mut sp = i.split("\n");
        (sp.next().unwrap().replace("10", ":"), sp.next().unwrap().replace("10", ":"))
    }).collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let pairs = parse_pairs(&input);
    println!("{}", right_order_sum(&pairs));
    println!("{}", decoder_key(&pairs));
}
