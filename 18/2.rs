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

fn compare_boxes(a: &str, b: &str) -> Option<String> {
    let mut pos = None;
    for (i, (x, y)) in a.chars().zip(b.chars()).enumerate() {
        if x != y {
            if pos.is_some() {
                // more than one chars differ
                return None;
            }
            pos = Some(i);
        }
    }
    if let Some(pos) = pos {
        let chs = a.chars().enumerate()
            .filter(|&(i, _)| i != pos)
            .map(|(_, c)| c).collect::<Vec<_>>();

        Some(chs.into_iter().collect())
    } else {
        // exactly the same
        unreachable!();
    }
}
fn proto_fabric_boxes(ids: &[String]) -> String {
    for a in ids {
        for b in ids {
            if a != b {
                let common_letters = compare_boxes(a, b);
                if let Some(cl) = common_letters {
                    return cl;
                }
            }
        }
    }

    unreachable!()
}

fn main() {
    let ids = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    let twos = ids.iter().filter(|x| exactly_n(x, 2)).count();
    let threes = ids.iter().filter(|x| exactly_n(x, 3)).count();
    println!("{}", twos * threes);
    println!("{}", proto_fabric_boxes(&ids));
}
