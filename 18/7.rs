use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// "Step A must be finished before step B can begin."
fn parse_rule(line: &str) -> (usize, usize) {
    let first = line.bytes().nth("Step ".len()).unwrap();
    let then = line.bytes().nth("Step X must be finished before step ".len()).unwrap();
    ((first - b'A') as usize, (then - b'A') as usize)
}

fn main() {
    let rules = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_rule(&x.unwrap())).collect::<Vec<_>>();
    let n = (b'z' - b'a' + 1) as usize;

    let mut finished = vec![false; n];
    let mut order = String::new();

    loop {
        let mut pending = vec![false; n];
        for &r in &rules {
            if !finished[r.0] {
                pending[r.1] = true;
            }
        }

        let next = pending.iter().enumerate().position(|(i, &p)| p == false && !finished[i]).unwrap();
        finished[next] = true;
        order.push((b'A' + next as u8) as char);

        if pending.iter().all(|&s_pend| s_pend == false) {
            break;
        }
    }

    println!("{}", order);
}
