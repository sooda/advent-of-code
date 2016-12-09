use std::fs::File;
use std::io::Read;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

fn decompressed_length(mut input: &str) -> usize {
    let mut len = 0;
    while let Some(open_paren) = input.find('(') {
        len += open_paren; // basic alphabet before the marker
        let close = input.find(')').unwrap();
        let parens = &input[open_paren + 1..close]; // let (parens) = input

        let mut sp = parens.split("x");
        let take = sp.next().unwrap().parse::<usize>().unwrap();
        let repeat = sp.next().unwrap().parse::<usize>().unwrap();

        len += take * repeat;

        // +1 to start after ), skip take amount of chars
        input = &input[close + 1 + take..];
    }
    len += input.len();

    len
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());
    for row in src.trim().split("\n") {
        println!("{}", decompressed_length(row));
    }
}
