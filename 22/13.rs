use std::io::{self, Read};
use std::str;

// skip a recursive list, find the first one at any depth
// the [ already consumed
fn skip_list(mut a: &[u8]) -> (Option<u8>, usize, &[u8]) {
    // find the first element
    let (a0, mut n, anext) = match a[0] {
        b'0' ..= b':' => (Some(a[0]), 1, &a[1..]),
        b'[' => skip_list(&a[1..]),
        b']' => (None, 0, a),
        _ => panic!("skip err1 {}", a[0] as char),
    };
    // consume calcium
    a = anext;
    loop {
        match a[0] {
            b'0' ..= b':' | b',' => {
                a = &a[1..];
                n += 1;
            },
            b'[' => {
                let (_, _, anext) = skip_list(&a[1..]);
                a = anext;
                n += 1;
            },
            b']' => {
                return (a0, n, &a[1..]);
            },
            _ => panic!("skip err2 {}", a[0] as char),
        };
    }
}

// left smaller than right?
// the [ of l[0] and r[0] already consumed
fn compare_list<'a>(mut l: &'a [u8], mut r: &'a [u8]) -> (Option<bool>, &'a [u8], &'a [u8]) {
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
                let (l0, nl, lnew) = skip_list(&l[1..]);
                if let Some(l0) = l0 {
                    if l0 < r[0] {
                        return (Some(true), l, r);
                    } else if l0 > r[0] {
                        return (Some(false), l, r);
                    } else if nl > 1 {
                        return (Some(false), l, r);
                    }
                } else {
                    // empty list is smaller than anything
                    return (Some(true), l, r);
                }
                l = lnew;
                r = &r[1..];
            },
            (b'0' ..= b':', b'[') => {
                // l number, r list
                let (r0, _rl, rnew) = skip_list(&r[1..]);
                if let Some(r0) = r0 {
                    if l[0] < r0 {
                        return (Some(true), l, r);
                    } else if l[0] > r0 {
                        return (Some(false), l, r);
                    }
                } else {
                    // anything is greater than empty list
                    return (Some(false), l, r);
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
}