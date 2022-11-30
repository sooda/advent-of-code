use std::fs::File;
use std::io::Read;
use std::mem::swap;
//
// rustc -L foo/deps 10.rs
extern crate regex;
use regex::Regex;

fn readfile(name: &str) -> String {
    let mut f = File::open(name).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    s
}

#[derive(Clone, Debug)]
enum Dest {
    Bot(usize),
    Output(usize)
}

// no zeros for values in the input
#[derive(Clone, Debug)]
struct Bot {
    lo: usize,
    hi: usize,
    lo_dest: Dest,
    hi_dest: Dest
}

fn assign(bot: &mut Bot, val: usize) {
    // is_none might be more rusty tho
    if bot.lo == 0 {
        bot.lo = val;
    }
    else {
        assert!(bot.hi == 0); // must not be full yet
        assert!(bot.lo != val); // no duplicates
        bot.hi = val;
        if bot.lo > bot.hi {
            swap(&mut bot.lo, &mut bot.hi);
        }
    }
}

fn action(input: &str, bots: &mut Vec<Bot>, outs: &mut Vec<usize>) {
    let re_goesto = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    let re_gives = Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)").unwrap();
    if let Some(cap) = re_goesto.captures(input) {
        let val = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let bot_num = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();

        if bot_num >= bots.len() {
            bots.resize(bot_num + 1, Bot { lo: 0, hi: 0, lo_dest: Dest::Bot(0), hi_dest: Dest::Bot(0) });
        }

        assign(&mut bots[bot_num], val);
    } else if let Some(cap) = re_gives.captures(input) {
        let bot_num = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let lo_dest_type = cap.get(2).unwrap().as_str();
        let lo_dest = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let hi_dest_type = cap.get(4).unwrap().as_str();
        let hi_dest = cap.get(5).unwrap().as_str().parse::<usize>().unwrap();

        if bot_num >= bots.len() {
            bots.resize(bot_num + 1, Bot { lo: 0, hi: 0, lo_dest: Dest::Bot(0), hi_dest: Dest::Bot(0) });
        }

        if lo_dest_type == "bot" {
            bots[bot_num].lo_dest = Dest::Bot(lo_dest);
        } else {
            bots[bot_num].lo_dest = Dest::Output(lo_dest);
            if lo_dest > outs.len() { outs.resize(lo_dest + 1, 0); }
        }

        if hi_dest_type == "bot" {
            bots[bot_num].hi_dest = Dest::Bot(hi_dest);
        } else {
            bots[bot_num].hi_dest = Dest::Output(hi_dest);
            if hi_dest > outs.len() { outs.resize(hi_dest + 1, 0); }
        }
    } else {
        unreachable!()
    }
}

fn evaluate(bots: &mut Vec<Bot>, outs: &mut Vec<usize>) {
    let mut handled = vec![false; bots.len()];
    let mut remaining = bots.len();
    // breadth-first-style evaluation of the "giving graph"
    while remaining > 0 {
        // cannot iter() because that would borrow and thus couldn't mutate
        for i in 0..bots.len() {
            if handled[i] {
                continue;
            }

            // clone this out to please the borrow fuc..checker as some are written later
            let b = bots[i].clone();
            println!("{} {:?}", i, b);
            // not yet fully filled values, so skip in this iteration
            if b.lo == 0 || b.hi == 0 {
                continue;
            }

            // ok, propagate this node in the graph
            handled[i] = true;
            remaining -= 1;
            match b.lo_dest {
                Dest::Bot(d) => {
                    assert!(b.lo != i);
                    println!("{} -> {}", b.lo, d);
                    assign(&mut bots[d], b.lo);
                },
                Dest::Output(o) => {
                    outs[o] = b.lo;
                }
            }
            match b.hi_dest {
                Dest::Bot(d) => {
                    assert!(b.hi != i);
                    println!("{} -> {}", b.hi, d);
                    assign(&mut bots[d], b.hi);
                },
                Dest::Output(o) => {
                    outs[o] = b.hi;
                }
            }
        }
    }
}

fn main() {
    let src = readfile(&std::env::args().nth(1).unwrap());
    let mut bots = Vec::new();
    let mut outs = Vec::new();
    for row in src.trim().split("\n") {
        action(row, &mut bots, &mut outs);
    }
    evaluate(&mut bots, &mut outs);
    for (i, b) in bots.iter().enumerate() {
        println!("bot {} compares {} with {}", i, b.lo, b.hi);
    }
    for (i, o) in outs.iter().enumerate() {
        println!("output {} has value {}", i, o);
    }
}

