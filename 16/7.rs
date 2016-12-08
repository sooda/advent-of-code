use std::fs::File;
use std::io::Read;

use std::str;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

// Autonomous Bridge Bypass Annotation
fn has_abba(s: &str) -> bool {
    let bs = s.as_bytes();
    for i in 0..bs.len() - 3 {
        if bs[i] == bs[i+3] && bs[i+1] == bs[i+2] && bs[i] != bs[i+1] {
            return true;
        }
    }

    false
}

// transport-layer snooping
fn support_tls(row: &str) -> u32 {
    let mut nonhyper_abba = false;
    // split into outer and inner parts of square brackets
    // last item is always out
    for pair in row.split("]") {
        let mut parts = pair.split("[");
        let nonhyper = parts.next().unwrap(); // always there
        let hypernet = parts.next(); // may not be there if last "pair"

        nonhyper_abba = nonhyper_abba || has_abba(nonhyper);

        if let Some(hyper) = hypernet {
            // hypernet sequences never have abbas
            if has_abba(hyper) {
                return 0;
            }
        }
    }

    nonhyper_abba as u32
}

// Area-Broadcast Accessor
fn next_aba(s: &str, start: usize) -> Option<usize> {
    let bs = s.as_bytes();
    for i in start .. bs.len()-2 {
        if bs[i] == bs[i+2] && bs[i] != bs[i+1] {
            return Some(i);
        }
    }

    None
}

// bab of aba in a hypernet?
fn has_bab(row: &str, aba: &str) -> bool {
    let aba = aba.as_bytes();
    let bab = &[aba[1], aba[0], aba[1]];
    let bab = str::from_utf8(bab).unwrap();

    for pair in row.split("]") {
        let mut parts = pair.split("[");
        let _ = parts.next().unwrap(); // ignore supernet
        if let Some(hyper) = parts.next() {
            if hyper.find(bab).is_some() {
                return true;
            }
        }
    }

    false
}

// super-secret listening
fn support_ssl(row: &str) -> u32 {
    // such bloat, search again for each supernet's aba
    for pair in row.split("]") {
        let mut parts = pair.split("[");
        let supernet = parts.next().unwrap(); // always there
        let mut idx = 0usize;
        while let Some(next) = next_aba(supernet, idx) {
            idx = next;
            if has_bab(row, &supernet[idx..idx+3]) {
                return 1;
            }
            idx += 1;
        }
        // hypernet ignored here
    }

    0
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());
    let sum = src.trim().split("\n").map(support_tls).sum::<u32>();
    println!("tls {}", sum);
    let sum = src.trim().split("\n").map(support_ssl).sum::<u32>();
    println!("ssl {}", sum);
}





