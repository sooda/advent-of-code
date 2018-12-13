use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// "Step A must be finished before step B can begin."
fn parse_rule(line: &str) -> (usize, usize) {
    let first = line.bytes().nth("Step ".len()).unwrap();
    let then = line.bytes().nth("Step X must be finished before step ".len()).unwrap();
    ((first - b'A') as usize, (then - b'A') as usize)
}

fn ideal_order(rules: &[(usize, usize)]) -> String {
    let n = (b'z' - b'a' + 1) as usize;

    let mut finished = vec![false; n];
    let mut order = String::new();

    loop {
        let mut pending = vec![false; n];
        for &r in rules {
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

    order
}

fn parallel_order_time(rules: &[(usize, usize)], min_time: usize, worker_count: usize) -> usize {
    let n = (b'z' - b'a' + 1) as usize;

    let mut started = vec![false; n];
    let mut finished = vec![false; n];
    let mut order = String::new();
    let mut clock = 0;
    // (start_time, work_item_index)
    let mut workers: Vec<Option<(usize, usize)>> = vec![None; worker_count];

    loop {
        // complete ongoing work
        for w in &mut workers {
            if let Some((started, item)) = *w {
                if started + min_time + item == clock {
                    println!("doned {:?} at {}", w, clock);
                    finished[item] = true;
                    order.push((b'A' + item as u8) as char);
                    *w = None;
                }
            }

        }

        if finished.iter().all(|&f| f == true) {
            break;
        }

        let mut pending = vec![false; n];
        for &r in rules {
            if !finished[r.0] {
                pending[r.1] = true;
            }
        }

        // find next work and start it for the elves
        for w in &mut workers {
            if w.is_some() {
                continue;
            }

            if let Some(next) = pending.iter().enumerate().position(|(i, &p)| p == false && !started[i]) {
                println!("start {} at {}", next, clock);
                *w = Some((clock, next));
                started[next] = true;
            }
        }

        clock += 1;
    }

    println!("{}", order);

    clock
}

fn main() {
    let rules = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_rule(&x.unwrap())).collect::<Vec<_>>();

    println!("{}", ideal_order(&rules));
    // println!("{}", parallel_order_time(&rules, 1, 2)); // also fix 'z' -> 'f'
    println!("{}", parallel_order_time(&rules, 61, 5));
}
