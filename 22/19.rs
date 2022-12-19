use std::io::{self, BufRead};
use std::collections::{HashSet};

extern crate regex;
use regex::Regex;

type Quad = [i16; 4];

#[allow(dead_code)]
const ORE: usize = 0;
#[allow(dead_code)]
const CLAY: usize = 1;
#[allow(dead_code)]
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

#[derive(Debug)]
struct Blueprint {
    id_num: i16,
    // costs[robot][material]
    costs: [Quad; 4]
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    bots: Quad,
    store: Quad,
    time_remaining: i16,
}

fn add(a: &mut Quad, b: &Quad) {
    a.iter_mut().zip(b.iter()).for_each(|(a, b)| *a += b);
}

fn sub(a: &mut Quad, b: &Quad) {
    a.iter_mut().zip(b.iter()).for_each(|(a, b)| *a -= b);
}

fn has_money(store: &Quad, cost: &Quad) -> bool {
    store.iter().zip(cost.iter()).all(|(s, c)| s >= c)
}

fn summation(n: i16) -> i16 {
    // 1 + 2 + ... + n
    n * (n + 1) / 2
}

// stupid dfs for now
fn crack_geodes(bp: &Blueprint, state: State, best: &mut i16, visited: &mut HashSet<State>, bot_limit: &Quad) {
    if !visited.insert(state.clone()) {
        return;
    }

    // what if we got a bot each time and still can't beat the best anymore?
    let heuristic = state.store[GEODE]
        + state.bots[GEODE] * state.time_remaining
        + summation(state.time_remaining);

    if state.time_remaining == 0 || heuristic < *best {
        if false && *best != 0 && state.store[GEODE] > *best {
            println!("best {:?} {:?}", bp, state);
        }
        *best = (*best).max(state.store[GEODE]);
        return;
    }

    for bot in 0..4 {
        // don't build too many bots
        let limit_reached = state.bots[bot] == bot_limit[bot];
        if has_money(&state.store, &bp.costs[bot]) && !limit_reached {
            let mut state = state.clone();
            sub(&mut state.store, &bp.costs[bot]);
            add(&mut state.store, &state.bots);
            state.bots[bot] += 1;
            state.time_remaining -= 1;
            crack_geodes(bp, state, best, visited, bot_limit);
        }
    }

    // build nothing
    {
        let mut state = state.clone();
        add(&mut state.store, &state.bots);
        state.time_remaining -= 1;
        crack_geodes(bp, state, best, visited, bot_limit);
    }
}

fn quality_level(bp: &Blueprint, max_time: i16) -> i16 {
    let state = State {
        bots: [1, 0, 0, 0],
        store: [0, 0, 0, 0],
        time_remaining: max_time,
    };
    let mut best = 0;
    // Guess that the max manufacturing rate needs to be at most what's needed for a bot.
    // max, not sum, because at each step only one bot can be produced.
    let bot_limit_heuristic = &[
        bp.costs.iter().map(|c| c[0]).max().unwrap(),
        bp.costs.iter().map(|c| c[1]).max().unwrap(),
        bp.costs.iter().map(|c| c[2]).max().unwrap(),
        std::i16::MAX,
    ];
    crack_geodes(bp, state, &mut best, &mut HashSet::new(), bot_limit_heuristic);
    bp.id_num * best
}

fn quality_level_sum(blueprints: &[Blueprint]) -> i16 {
    blueprints.iter().map(|bp| quality_level(bp, 24)).sum()
}

fn parse_blueprint(line: &str) -> Blueprint {
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    let cap = re.captures(line).unwrap();
    // ughh
    let id_num = cap.get(1).unwrap().as_str().parse().unwrap();
    let ore_ore = cap.get(2).unwrap().as_str().parse().unwrap();
    let clay_ore = cap.get(3).unwrap().as_str().parse().unwrap();
    let obsi_ore = cap.get(4).unwrap().as_str().parse().unwrap();
    let obsi_clay = cap.get(5).unwrap().as_str().parse().unwrap();
    let geode_ore = cap.get(6).unwrap().as_str().parse().unwrap();
    let geode_obsidian = cap.get(7).unwrap().as_str().parse().unwrap();

    Blueprint {
        id_num,
        costs: [
            [ore_ore, 0, 0, 0],
            [clay_ore, 0, 0, 0],
            [obsi_ore, obsi_clay, 0, 0],
            [geode_ore, 0, geode_obsidian, 0],
        ]
    }
}

fn main() {
    let blueprints: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_blueprint(&line.unwrap()))
        .collect();
    println!("{}", quality_level_sum(&blueprints));
}
