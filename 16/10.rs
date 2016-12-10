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
        let val = cap.at(1).unwrap().parse::<usize>().unwrap();
        let bot_num = cap.at(2).unwrap().parse::<usize>().unwrap();

        if bot_num >= bots.len() {
            bots.resize(bot_num + 1, Bot { lo: 0, hi: 0, lo_dest: Dest::Bot(0), hi_dest: Dest::Bot(0) });
        }

        assign(&mut bots[bot_num], val);
    } else if let Some(cap) = re_gives.captures(input) {
        let bot_num = cap.at(1).unwrap().parse::<usize>().unwrap();
        let lo_dest_type = cap.at(2).unwrap();
        let lo_dest = cap.at(3).unwrap().parse::<usize>().unwrap();
        let hi_dest_type = cap.at(4).unwrap();
        let hi_dest = cap.at(5).unwrap().parse::<usize>().unwrap();

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
    while remaining > 0 {
        // cannot iter() because that would borrow and thus couldn't mutate
        for i in 0..bots.len() {
            if handled[i] {
                continue;
            }

            let (lo, lod, hi, hid) = {
                let b = &bots[i];
                println!("{} {:?}", i, b);
                let mut lo = 0; let mut lod = 0;
                let mut hi = 0; let mut hid = 0;
                // fully filled values, this is handled now
                if b.lo != 0 && b.hi != 0 {
                    handled[i] = true;
                    remaining -= 1;
                    // can give only when both lo and hi are known
                    if let Dest::Bot(d) = b.lo_dest {
                        lo = b.lo;
                        lod = d;
                    } else if let Dest::Output(o) = b.lo_dest {
                        outs[o] = b.lo;
                    }
                    if let Dest::Bot(d) = b.hi_dest {
                        hi = b.hi;
                        hid = d;
                    } else if let Dest::Output(o) = b.hi_dest {
                        outs[o] = b.hi;
                    }
                }
                (lo, lod, hi, hid)
            };

            // dests are bots, not outputs? then pass values to them
            if lo != 0 {
                assert!(lod != i);
                println!("{} -> {}", lo, lod);
                assign(&mut bots[lod], lo);
            }
            if hi != 0 {
                assert!(hid != i);
                println!("{} -> {}", hi, hid);
                assign(&mut bots[hid], hi);
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

