use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn score(stream: &str) -> u32 {
    //println!("{}", stream);
    let mut level = 0;
    let mut garbage = false;
    let mut ignore = false;
    let mut scores = 0;
    for c in stream.chars() {
        //println!("c:{} l:{} s:{} g:{} i:{}", c, level, scores, garbage, ignore);
        if garbage {
            if ignore {
                ignore = false;
            } else {
                match c {
                    '!' => ignore = true,
                    '>' => garbage = false,
                    _ => {},
                }
            }
        } else {
            match c {
                '{' => { level += 1; scores += level; },
                '}' => level -= 1,
                '<' => garbage = true,
                ',' => {},
                _ => panic!("bad: {}", c)
            }
        }
    }

    scores
}

fn main() {
    assert!(score("{}") == 1);
    assert!(score("{{{}}}") == 6);
    assert!(score("{{},{}}") == 5);
    assert!(score("{{{},{},{{}}}}") == 16);
    assert!(score("{<a>,<a>,<a>,<a>}") == 1);
    assert!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}") == 9);
    assert!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}") == 9);
    assert!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}") == 3);

    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap();
    println!("{}", score(&input));
}
