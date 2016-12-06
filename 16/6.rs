use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

fn message(rows: Vec<&str>) -> String {
    let mut allcounts = vec![HashMap::new(); rows[0].len()];

    // count frequencies per position
    for row in rows {
        for (ch, cnts) in row.chars().zip(allcounts.iter_mut()) {
            *cnts.entry(ch).or_insert(0) += 1;
        }
    }

    let pwd = allcounts.iter().map(
        |one_hash| {
            // into vec to be able to move items around...
            let per_char = one_hash.iter().collect::<Vec<_>>();
            // flip chars and counts
            let mut per_count = per_char.iter().map(|&(x, y)| (y, x)).collect::<Vec<_>>();
            per_count.sort_by(|a, b| b.0.cmp(a.0));

            // most common
            *per_count[0].1
        }).collect::<String>();

    pwd
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());

    println!("{}", message(src.trim().split("\n").collect()));
}




