use std::io::{self, BufRead};
// HashSet<String> isn't hashable so it wouldn't fit in another hashset
// (not a problem anymore?)
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

    fn all(self, other: Self) -> bool {
        self.and(other) == other
    }
}

impl std::fmt::Debug for OpenMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for i in 0..63 {
            if self.is_open(i) {
                write!(f, "X")?;
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
struct Score {
    timeleft: i64,
    nopened: usize,
    steamed: i64,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
struct Node {
    vidx: usize,
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
    let startnode = Node { vidx: startidx, vstates: OpenMap::new(), timeleft: 30 };
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited = HashMap::new();
    heap.push(State { score: startscore, node: startnode });
    // those two tuples backwards
    // from state to score
    visited.insert(startnode, startscore);

    let good_valves = valves.iter().enumerate()
        .filter(|&(_, v)| v.rate > 0)
        .fold(OpenMap::new(), |map, (i, _)| map.open(i));

    let mut push = |h: &mut BinaryHeap<_>, state: State| {
        let (score, node) = (state.score, state.node);

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
                let more_time = score.timeleft > oldscore.timeleft;
                // (nopened is part of state, cached for score)
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
                        vstates: node.vstates.open(node.vidx),
                        timeleft: newtime,
                    }
                });
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
                        vstates: node.vstates,
                        timeleft: newtime,
                    }
                });
            }
        }
    }

    steam_possible
}

fn most_pressure(valves: &[Valve]) -> i64 {
    dijkstra(valves, geti(valves, "AA"))
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
    }
}
