use std::io::{self, Read};
use std::str;
use std::cmp::Ordering;

// left smaller than right?
// the [ of l[0] and r[0] already consumed
fn compare_list<'a, 'b>(mut l: &'a [u8], mut r: &'b [u8]) -> (Option<bool>, &'a [u8], &'b [u8]) {
    loop {
        match (l[0], r[0]) {
            (b']', b']') => {
                // both ended, validity unknown
                return (None, &l[1..], &r[1..]);
            },
            (b']', _) => {
                // left ran out
                return (Some(true), l, r);
            },
            (_, b']') => {
                // right ran out
                return (Some(false), l, r);
            },
            (b'0' ..= b':', b'0' ..= b':') => {
                // both are numbers
                if l[0] < r[0] {
                    return (Some(true), l, r);
                } else if l[0] > r[0] {
                    return (Some(false), l, r);
                }
                l = &l[1..];
                r = &r[1..];
            },
            (b',', b',') => {
                // both lists continue
                l = &l[1..];
                r = &r[1..];
            },
            (b'[', b'[') => {
                // both contain a list; read and skip over
                let (valid, lnew, rnew) = compare_list(&l[1..], &r[1..]);
                if let Some(v) = valid {
                    return (Some(v), l, r);
                }
                l = lnew;
                r = rnew;
            },
            (b'[', b'0' ..= b':') => {
                // l list, r number
                let rr = [r[0], b']'];
                let (valid, lnew, _rnew) = compare_list(&l[1..], &rr);
                if let Some(v) = valid {
                    return (Some(v), l, r);
                }
                l = lnew;
                r = &r[1..];
            },
            (b'0' ..= b':', b'[') => {
                // l number, r list
                let ll = [l[0], b']'];
                let (valid, _lnew, rnew) = compare_list(&ll, &r[1..]);
                if let Some(v) = valid {
                    return (Some(v), l, r);
                }
                l = &l[1..];
                r = rnew;
            },
            _ => panic!("what? {} {}", l[0] as char, r[0] as char),
        };
    }
}

fn right_order(l: &[u8], r: &[u8]) -> bool {
    assert!(l[0] == b'[');
    assert!(r[0] == b'[');
    compare_list(&l[1..], &r[1..]).0.unwrap()
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
        match compare_list(&l.as_bytes()[1..], &r.as_bytes()[1..]).0 {
            Some(true) => Ordering::Less,
            None => Ordering::Equal,
            Some(false) => Ordering::Greater,
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
