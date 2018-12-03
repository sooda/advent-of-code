use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

extern crate regex;
use regex::Regex;

fn parse_line(re: &Regex, line: &str) -> (usize, usize, usize, usize) {
    let cap = re.captures(line).unwrap();
    let x = cap.get(1).unwrap().as_str().parse().unwrap();
    let y = cap.get(2).unwrap().as_str().parse().unwrap();
    let w = cap.get(3).unwrap().as_str().parse().unwrap();
    let h = cap.get(4).unwrap().as_str().parse().unwrap();

    (x, y, w, h)
}

fn main() {
    let re = Regex::new(r"#\d+ @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let claims = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&re, &x.unwrap())).collect::<Vec<_>>();
    let mut fabric = vec![0; 1000 * 1000];
    for c in claims {
        let (x0, y0, w, h) = c;
        for y in y0..y0+h {
            for x in x0..x0+w {
                fabric[y * 1000 + x] += 1;
            }
        }
    }
    println!("{}", fabric.iter().filter(|&&x| x > 1).count());
}
