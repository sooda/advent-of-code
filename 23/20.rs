use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque};

enum ModuleKind {
    Flipflop,
    Conjunction,
    Broadcaster,
}
use ModuleKind::*;

struct Module {
    name: String,
    kind: ModuleKind,
    outputs: Vec<String>,
}

#[derive(Debug)]
enum Memory<'a> {
    Flipflop(bool),
    Conjunction(HashMap<&'a str, bool>),
    Broadcaster(()),
}

fn button<'a>(config: &HashMap<&'a str, &'a Module>, states: &mut HashMap<&'a str, Memory<'a>>, lookup: Option<&str>) -> Option<(usize, usize)> {
    let mut fifo = VecDeque::new();
    fifo.push_back(("button", "broadcaster", false));
    let mut signal_count = [0; 2];
    while let Some((in_name, name, level)) = fifo.pop_front() {
        signal_count[level as usize] += 1;
        let (outputs, next_level) = match config.get(name).map(|m| (&m.kind, &m.outputs)) {
            Some((Flipflop, o)) => {
                if !level {
                    let m = states.get(name).unwrap();
                    if let &Memory::Flipflop(prev) = m {
                        states.insert(name, Memory::Flipflop(!prev));
                        (o, !prev)
                    } else { panic!() } // FIXME bind instance and type better together
                } else {
                    continue;
                }
            },
            Some((Conjunction, o)) => {
                let m = states.get_mut(name).unwrap();
                if let Memory::Conjunction(inps) = m {
                    inps.insert(in_name, level);
                    let pulse = if inps.values().all(|&v| v) {
                        false
                    } else {
                        true
                    };
                    (o, pulse)
                } else { panic!(); } // FIXME same
            },
            Some((Broadcaster, o)) => {
                (o, level)
            },
            None => {
                assert!(name == "output" || name == "rx");
                continue;
            }
        };
        if let Some(x) = lookup {
            if x == name && next_level {
                return None;
            }
        }
        for dest in outputs {
            fifo.push_back((name, dest, next_level));
        }
    }
    Some((signal_count[0], signal_count[1]))
}

fn conj_input_state<'a, 'b>(config: &'a [Module], name: &str) -> HashMap<&'a str, bool> {
    config.iter().
        filter_map(|m| {
            if m.outputs.iter().any(|d| d == name) { Some((&m.name as &str, false)) } else { None }
        })
    .collect()
}

fn setup_sim(config: &[Module]) -> (HashMap<&str, &Module>, HashMap<&str, Memory>) {
    let state = config.iter()
        .map(|m| {
            (&m.name as &str, match m.kind {
                Flipflop => Memory::Flipflop(false),
                Conjunction => Memory::Conjunction(conj_input_state(config, &m.name)),
                Broadcaster => Memory::Broadcaster(()),
            })
        })
    .collect();
    let config = config.iter().map(|m| (&m.name as &str, m)).collect::<HashMap<_, _>>();
    (config, state)
}

// pulse network structure:
// broadcaster -> { 4 counters } -> one 4-in nand -> rx
// counters:
// - flop bits in series
// - random-ish feedback stuff from a big nand
// - one inverter output
// turns out the counters reset at prime number intervals,
// so least common multiple of their cycles is trivial
fn rx_low_pulse_time(config: &[Module]) -> usize {
    let rx_parent_nand = config.iter().find(|m| m.outputs.iter().all(|o| o == "rx")).unwrap();
    let network_parents = config.iter().filter(|m| m.outputs.iter().all(|o| *o == rx_parent_nand.name));
    let mut result = 1;
    // wasteful to loop each separately, but cumbersome to test many names in the loop.
    // one cool option would be to split the network to counters only?
    for p in network_parents {
        let (cfg, mut state) = setup_sim(config);
        let mut found = None;
        for i in 1..9999 {
            if button(&cfg, &mut state, Some(&p.name)).is_none() {
                found = Some(i);
                break;
            }
        }
        result *= found.unwrap();
    }
    result
}


fn pulses(config: &[Module]) -> usize {
    let (config, mut state) = setup_sim(config);
    let mut pulses_lo = 0;
    let mut pulses_hi = 0;
    for _ in 0..1000 {
        if let Some((lo, hi)) = button(&config, &mut state, None) {
            pulses_lo += lo;
            pulses_hi += hi;
        } else {
            panic!()
        }
    }
    pulses_lo * pulses_hi
}

fn parse_module(line: &str) -> Module {
    let mut sp = line.split(" -> ");
    let name = sp.next().unwrap();
    let outputs = sp.next().unwrap();
    let outputs = outputs.split(", ").map(|s| s.to_string()).collect();
    let (name, kind) = if name == "broadcaster" {
        (name.to_string(), Broadcaster)
    } else if name.starts_with('%') {
        (name.strip_prefix('%').unwrap().to_string(), Flipflop)
    } else if name.starts_with('&') {
        (name.strip_prefix('&').unwrap().to_string(), Conjunction)
    } else {
        panic!()
    };
    Module { name, kind, outputs }
}

fn main() {
    let config = io::stdin().lock().lines()
        .map(|row| parse_module(&row.unwrap()))
        .collect::<Vec<_>>();
    println!("{}", pulses(&config));
    println!("{}", rx_low_pulse_time(&config));
}
