use std::fs::File;
use std::io::Read;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

fn has_abba(s: &str) -> bool {
    let bs = s.as_bytes();
    for i in 0..bs.len() - 3 {
        if bs[i] == bs[i+3] && bs[i+1] == bs[i+2] && bs[i] != bs[i+1] {
            return true;
        }
    }

    false
}

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

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());
    let sum = src.trim().split("\n").map(support_tls).sum::<u32>();
    println!("{}", sum);
}





