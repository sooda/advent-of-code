use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn exactly_n(id: &str, n: usize) -> bool {
    for x in b'a'..=b'z' {
        let found = id.bytes().filter(|&c| c == x).count();
        if found == n {
            return true;
        }
    }

    false
}

fn main() {
    let ids = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    let twos = ids.iter().filter(|x| exactly_n(x, 2)).count();
    let threes = ids.iter().filter(|x| exactly_n(x, 3)).count();
    println!("{}", twos * threes);
}
