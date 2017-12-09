use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn score_garbage(stream: &str) -> (u32, u32) {
    //println!("{}", stream);
    let mut level = 0;
    let mut garbage = false;
    let mut ignore = false;
    let mut scores = 0;
    let mut garbages = 0;
    for c in stream.chars() {
        //println!("c:{} l:{} s:{} g:{} i:{}", c, level, scores, garbage, ignore);
        if garbage {
            if ignore {
                ignore = false;
            } else {
                match c {
                    '!' => ignore = true,
                    '>' => garbage = false,
                    _ => garbages += 1,
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

    (scores, garbages)
}

fn main() {
    assert!(score_garbage("{}").0 == 1);
    assert!(score_garbage("{{{}}}").0 == 6);
    assert!(score_garbage("{{},{}}").0 == 5);
    assert!(score_garbage("{{{},{},{{}}}}").0 == 16);
    assert!(score_garbage("{<a>,<a>,<a>,<a>}").0 == 1);
    assert!(score_garbage("{{<ab>},{<ab>},{<ab>},{<ab>}}").0 == 9);
    assert!(score_garbage("{{<!!>},{<!!>},{<!!>},{<!!>}}").0 == 9);
    assert!(score_garbage("{{<a!>},{<a!>},{<a!>},{<ab>}}").0 == 3);

    assert!(score_garbage("<>").1 == 0);
    assert!(score_garbage("<random characters>").1 == 17);
    assert!(score_garbage("<<<<>").1 == 3);
    assert!(score_garbage("<{!>}>").1 == 2);
    assert!(score_garbage("<!!>").1 == 0);
    assert!(score_garbage("<!!!>>").1 == 0);
    assert!(score_garbage("<{o\"i!a,<{i<a>").1 == 10);

    let input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap();
    println!("{:?}", score_garbage(&input));
}
