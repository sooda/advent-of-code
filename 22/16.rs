use std::io::{self, BufRead};
use std::collections::{HashMap, hash_map::Entry, BinaryHeap};

extern crate regex;
use regex::Regex;

struct Valve {
    name: String,
    rate: i64,
    neighs: Vec<String>,
}

fn get<'a>(valves: &'a [Valve], name: &str) -> &'a Valve {
    valves.iter().find(|v| v.name == name).unwrap()
}

fn geti(valves: &[Valve], name: &str) -> usize {
    valves.iter().position(|v| v.name == name).unwrap()
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
struct OpenMap(u64);

impl OpenMap {
    fn new() -> Self {
        Self(0)
    }

    fn is_open(self, i: usize) -> bool {
        (self.0 & (1 << i)) != 0
    }

    fn open(self, i: usize) -> Self {
        Self(self.0 | (1 << i))
    }

    fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    fn and(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

impl std::fmt::Debug for OpenMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let alphas = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
        for i in 0..63 {
            if self.is_open(i) {
                write!(f, "{}", alphas[i % 26] as char)?;
            } else {
                write!(f, "_")?;
            }
        }
        Ok(())
    }
}

// nopened steamed timeleft: 7s
// nopened timeleft steamed: 1.6s
// steamed nopened timeleft: 7s
// steamed timeleft nopened: 7s
// timeleft steamed nopened: 1.5s
// timeleft nopened steamed: 1.5s
// timeleft dominates, needs to be before steamed
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
// like distance when reached this node, but bigger means better
// time, vent opening and steaming happens during the travel on an edge
struct Score {
    timeleft: i64, // when entering this state; 0 when there's nothing to do
    nopened: usize, // also, opened before this state
    steamed: i64, // also, when entering this state
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
struct Node {
    vidx: usize,
    elephant: usize,
    vstates: OpenMap,
    timeleft: i64,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct State {
    score: Score,
    node: Node
}

fn vent(valves: &[Valve], opened: OpenMap) -> i64 {
    valves.iter().enumerate()
        .filter(|&(i, _)| opened.is_open(i))
        .map(|(_, v)| v.rate)
        .sum()
}

fn dijkstra(valves: &[Valve], startidx: usize) -> i64 {
    let debug = false;
    let startscore = Score { steamed: 0, nopened: 0, timeleft: 30 };
    let startnode = Node { vidx: startidx, elephant: 0, vstates: OpenMap::new(), timeleft: 30 };
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited = HashMap::new();
    heap.push(State { score: startscore, node: startnode });
    // those two tuples backwards
    // from state to score
    visited.insert(startnode, startscore);

    let good_valves = valves.iter().enumerate()
        .filter(|&(_, v)| v.rate > 0)
        .fold(OpenMap::new(), |map, (i, _)| map.open(i));

    let mut push = |h: &mut BinaryHeap<_>, state: State, best| {
        let (score, node) = (state.score, state.node);
        let max_possible_vent = score.timeleft * good_valves.len() as i64;

        // goal is max possible vent
        if score.steamed + max_possible_vent <= best {
            return;
        }

        let nent = visited.entry(node);
        match nent {
            Entry::Vacant(e) => {
                if debug {
                    println!("   to {} {:?} steam {} nopen {} time {} (new)",
                             valves[node.vidx].name, node.vstates,
                             score.steamed, score.nopened, score.timeleft);
                }
                e.insert(score);
                h.push(state);
            },
            Entry::Occupied(mut e) => {
                let oldscore = e.get();
                let more_steam = score.steamed > oldscore.steamed;
                if more_steam {
                    if debug {
                        println!("   to {} {:?} steam {} nopen {} time {} (better)",
                                 valves[node.vidx].name, node.vstates,
                                 score.steamed, score.nopened, score.timeleft);
                    }
                    e.insert(score);
                    h.push(state);
                }
            },
        }
    };

    let mut steam_possible = 0;

    while let Some(state) = heap.pop() {
        let (score, node) = (state.score, state.node);
        if debug {
            println!("visit {} {:?} steam {} nopen {} time {}",
                     valves[node.vidx].name, node.vstates,
                     score.steamed, score.nopened, score.timeleft);
        }

        steam_possible = steam_possible.max(score.steamed);

        let posvalve = &valves[node.vidx];
        let release = vent(valves, node.vstates);
        let newtime = score.timeleft - 1;

        if newtime >= 0 {
            if good_valves.is_open(node.vidx) {
                // steam this ham in particular
                push(&mut heap, State {
                    score: Score {
                        steamed: score.steamed + release,
                        nopened: node.vstates.open(node.vidx).and(good_valves).len(),
                        timeleft: newtime,
                    },
                    node: Node {
                        vidx: node.vidx,
                        elephant: 0,
                        vstates: node.vstates.open(node.vidx),
                        timeleft: newtime,
                    }
                }, steam_possible);
            }
            for nname in &posvalve.neighs {
                let nidx = geti(valves, &nname);
                push(&mut heap, State {
                    score: Score {
                        steamed: score.steamed + release,
                        nopened: score.nopened,
                        timeleft: newtime,
                    },
                    node: Node {
                        vidx: nidx,
                        elephant: 0,
                        vstates: node.vstates,
                        timeleft: newtime,
                    }
                }, steam_possible);
            }
        }
    }

    steam_possible
}

fn most_pressure(valves: &[Valve]) -> i64 {
    dijkstra(valves, geti(valves, "AA"))
}

fn dijkstra_with_elephant(valves: &[Valve], startidx: usize) -> i64 {
    let debug = false;
    let startscore = Score { steamed: 0, nopened: 0, timeleft: 26 };
    let startnode = Node { vidx: startidx, elephant: startidx, vstates: OpenMap::new(), timeleft: 26 };
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited = HashMap::new();
    heap.push(State { score: startscore, node: startnode });
    // those two tuples backwards
    // from state to score
    visited.insert(startnode, startscore);

    let good_valves = valves.iter().enumerate()
        .filter(|&(_, v)| v.rate > 0)
        .fold(OpenMap::new(), |map, (i, _)| map.open(i));

    let mut steam_possible = 0;

    let mut push = |h: &mut BinaryHeap<_>, state: State, best| {
        let (score, node) = (state.score, state.node);
        let max_possible_vent = score.timeleft * good_valves.len() as i64;

        // goal is max possible vent
        if score.steamed + max_possible_vent <= best {
            return;
        }

        let nent = visited.entry(node);
        match nent {
            Entry::Vacant(e) => {
                if debug {
                    println!("   to {} {} {:?} steam {} nopen {} time {} (new)",
                             valves[node.vidx].name, valves[node.elephant].name, node.vstates,
                             score.steamed, score.nopened, 27-score.timeleft);
                }
                e.insert(score);
                h.push(state);
            },
            Entry::Occupied(mut e) => {
                let oldscore = e.get();
                let more_steam = score.steamed > oldscore.steamed;
                if more_steam {
                    if debug {
                        println!("   to {} {} {:?} steam {} nopen {} time {} (better)",
                                 valves[node.vidx].name, valves[node.elephant].name, node.vstates,
                                 score.steamed, score.nopened, 27-score.timeleft);
                    }
                    e.insert(score);
                    h.push(state);
                }
            },
        }
    };

    while let Some(state) = heap.pop() {
        let (score, node) = (state.score, state.node);
        if debug {
            println!("visit {} {} {}left {:?} steam {} nopen {} time {}",
                     valves[node.vidx].name, valves[node.elephant].name, node.timeleft, node.vstates,
                     score.steamed, score.nopened, 27-score.timeleft);
        }

        if score.steamed > steam_possible {
            println!("{}", score.steamed);
        }
        steam_possible = steam_possible.max(score.steamed);

        let release = vent(valves, node.vstates);
        let steamed = score.steamed + release;
        let timeleft = score.timeleft - 1;

        if timeleft >= 0 {
            // both main characters either move elsewhere or open the valve they're at.
            // "opening" an opened valve may happen; that just means to sit tight.
            // TODO: doing nothing is meaningful only if there's nothing else to do
            for &(self_move, elep_move) in &[
                (false, false),
                (false, true),
                (true, false),
                (true, true),
            ] {
                // can be already open, but kept simple for now
                let mut vstates = node.vstates;
                if !self_move && good_valves.is_open(node.vidx) {
                    vstates = vstates.open(node.vidx);
                }
                if !elep_move && good_valves.is_open(node.elephant) {
                    vstates = vstates.open(node.elephant);
                }
                let nopened = vstates.and(good_valves).len();
                let score = Score {
                    steamed,
                    nopened,
                    timeleft,
                };

                if !self_move && !elep_move {
                    // steam this ham in particular
                    push(&mut heap, State {
                        score,
                        node: Node {
                            vidx: node.vidx,
                            elephant: node.elephant,
                            vstates,
                            timeleft,
                        }
                    }, steam_possible);
                }

                // ok to use the above score, as move and open cannot happen in parallel

                if self_move && !elep_move {
                    for nname in &valves[node.vidx].neighs {
                        let nidx = geti(valves, &nname);
                        push(&mut heap, State {
                            score,
                            node: Node {
                                vidx: nidx,
                                elephant: node.elephant,
                                vstates,
                                timeleft,
                            }
                        }, steam_possible);
                    }
                }

                if elep_move && !self_move {
                    for nname in &valves[node.elephant].neighs {
                        let nidx = geti(valves, &nname);
                        push(&mut heap, State {
                            score,
                            node: Node {
                                vidx: node.vidx,
                                elephant: nidx,
                                vstates,
                                timeleft,
                            }
                        }, steam_possible);
                    }
                }
                if self_move && elep_move {
                    for selfname in &valves[node.vidx].neighs {
                        for elepname in &valves[node.elephant].neighs {
                            push(&mut heap, State {
                                score,
                                node: Node {
                                    vidx: geti(valves, &selfname),
                                    elephant: geti(valves, &elepname),
                                    vstates,
                                    timeleft,
                                }
                            }, steam_possible);
                        }
                    }
                }
            }
        }
    }

    steam_possible
}

fn most_pressure_with_elephant(valves: &[Valve]) -> i64 {
    dijkstra_with_elephant(valves, geti(valves, "AA"))
}

fn dump_map(valves: &[Valve]) {
    println!("digraph G {{");
    for v in valves {
        for n in &v.neighs {
            println!("{} [style=filled; fillcolor={}];", v.name, if v.rate == 0 { "grey" } else { "white" });
            println!("{} -> {};", v.name, get(valves, &n).name);
        }
    }
    println!("}}");

}

fn parse_valve(line: &str) -> Valve {
    // Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    // this does not strictly validate the format but accepts the input alright
    let re = Regex::new(r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
    let cap = re.captures(line).unwrap();
    // ughh
    let name = cap.get(1).unwrap().as_str().to_string();
    let rate = cap.get(2).unwrap().as_str().parse().unwrap();
    let neighs = cap.get(3).unwrap().as_str().split(", ").map(|s| s.to_string()).collect();

    Valve { name, rate, neighs }
}

fn main() {
    let valves: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_valve(&line.unwrap()))
        .collect();
    if false {
        dump_map(&valves);
    } else {
        println!("{}", most_pressure(&valves));
        println!("{}", most_pressure_with_elephant(&valves));
    }
}
