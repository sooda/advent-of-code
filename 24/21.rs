#![feature(let_chains)]

use std::io::{self, BufRead};
use std::collections::HashMap;

type Pos = (i32, i32);

fn add(a: Pos, b: Pos) -> Pos {
    (a.0 + b.0, a.1 + b.1)
}

struct Graph {
    nodes: Vec<char>,
    positions: Vec<Pos>,
    // much redundancy but it's simpler this way and not much more mem
    directions: HashMap<char, HashMap<char, (i32, i32)>>,
}

// return full graph matrix, n^2 distances
fn graph(descr: &str) -> Graph {
    let nodes = descr.chars().collect::<Vec<_>>();
    let w = 3i32;
    let h = descr.len() as i32 / 3i32;
    assert_eq!(descr.len() % 3, 0);
    let mut directions = HashMap::new();
    let mut positions = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let node = nodes[(y * w + x) as usize];
            positions.push((x, y));
            let mut ndirs = HashMap::new();
            for ey in 0..h {
                for ex in 0..w {
                    let en = nodes[(ey * w + ex) as usize];
                    let delta = (ex - x, ey - y);
                    ndirs.insert(en, delta);
                }
            }
            directions.insert(node, ndirs);
        }
    }
    Graph { nodes, directions, positions }
}

fn pos_of(ch: char, graph: &Graph) -> Pos {
     *graph.nodes.iter()
        .zip(graph.positions.iter())
        .find(|&(&n, _p)| n == ch)
        .unwrap().1
}

fn node_at(pos: Pos, graph: &Graph) -> char {
    *graph.nodes.iter()
        .zip(graph.positions.iter())
        .find(|&(_n, &p)| p == pos)
        .unwrap().0
}

// number of remaining keypads specifies graph queue exactly because it is so limited in values
type Cache = HashMap<(char, Vec<char>, usize, Vec<char>), (usize, Vec<char>)>;

// return how many human presses needed to control keypad[0] into dest
fn resolve_one(dest: char, code: &[char], keypads: &[&Graph], states: &mut [char], c: &mut Cache) -> usize {
    let cache_key = (dest, code.to_vec(), keypads.len(), states.to_vec());
    if let Some((dist, st)) = c.get(&cache_key) {
        states.copy_from_slice(&st);
        return *dist;
    }

    if keypads.len() == 1 {
        assert_eq!(states.len(), 0);
        // this is human pad, just press it
        1
    } else {
        let bot = states[0];
        let botpos = pos_of(bot, keypads[0]);
        let (dx, dy) = keypads[0].directions[&bot][&dest];
        let xch = node_at(add(botpos, (dx, 0)), keypads[0]);
        let ych = node_at(add(botpos, (0, dy)), keypads[0]);
        assert!(xch != '.' || ych != '.');

        let tap_dx = |dx: i32, st: &mut [char], c: &mut Cache| {
            let ch = if dx > 0 { '>' } else if dx < 0 { '<' } else { bot };
            (0..dx.abs()).map(|_| resolve_one(ch, code, &keypads[1..], st, c)).sum::<usize>()
        };

        let tap_dy = |dy: i32, st: &mut [char], c: &mut Cache| {
            let ch = if dy > 0 { 'v' } else if dy < 0 { '^' } else { bot };
            (0..dy.abs()).map(|_| resolve_one(ch, code, &keypads[1..], st, c)).sum::<usize>()
        };

        let dx_dy_a = {
            let mut st = states[1..].to_vec();
            let dxcost = tap_dx(dx, &mut st, c);
            let dycost = tap_dy(dy, &mut st, c);
            let actcost = resolve_one('A', code, &keypads[1..], &mut st, c);
            (dxcost + dycost + actcost, st)
        };
        let dy_dx_a = {
            let mut st = states[1..].to_vec();
            let dycost = tap_dy(dy, &mut st, c);
            let dxcost = tap_dx(dx, &mut st, c);
            let actcost = resolve_one('A', code, &keypads[1..], &mut st, c);
            (dycost + dxcost + actcost, st)
        };

        states[0] = dest;
        // dx first shorter and it doesn't go to the forbidden node?
        // or it's longer but y goes to the forbidden node?
        let bestcost = if (dx_dy_a.0 <= dy_dx_a.0 && xch != '.') || ych == '.' {
            states[1..].copy_from_slice(&dx_dy_a.1);
            dx_dy_a.0
        } else {
            states[1..].copy_from_slice(&dy_dx_a.1);
            dy_dx_a.0
        };
        c.insert(cache_key, (bestcost, states.to_vec()));
        bestcost
    }
}

fn resolve(code: &[char], keypads: &[&Graph], states: &mut [char], c: &mut Cache) -> usize {
    if code.is_empty() {
        0
    } else {
        let this_length = resolve_one(code[0], code, keypads, states, c);
        // this recursion could have been a loop
        this_length + resolve(&code[1..], keypads, states, c)
    }
}

// >>> len("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A")
// 68
fn complexity(code: &str) -> usize {
    let doorpad = graph("789456123.0A");
    let botpad = graph(".^A<v>");
    let sequence = resolve(&code.chars().collect::<Vec<char>>(),
    &[&doorpad, &botpad, &botpad, &botpad],
                           &mut ['A', 'A', 'A'], &mut Cache::new());

    sequence * code[0..code.len()-1].parse::<usize>().unwrap()
}

fn complexity_b(code: &str) -> usize {
    let doorpad = graph("789456123.0A");
    let botpad = graph(".^A<v>");

    // lol
    let sequence = resolve(
        &code.chars().collect::<Vec<char>>(),
        &[&doorpad,
            &botpad, &botpad, &botpad, &botpad, &botpad,
            &botpad, &botpad, &botpad, &botpad, &botpad,
            &botpad, &botpad, &botpad, &botpad, &botpad,
            &botpad, &botpad, &botpad, &botpad, &botpad,
            &botpad, &botpad, &botpad, &botpad, &botpad,
            &botpad,
        ],
        &mut ['A',
            'A', 'A', 'A', 'A', 'A',
            'A', 'A', 'A', 'A', 'A',
            'A', 'A', 'A', 'A', 'A',
            'A', 'A', 'A', 'A', 'A',
            'A', 'A', 'A', 'A', 'A',
        ],
        &mut Cache::new());

    sequence * code[0..code.len()-1].parse::<usize>().unwrap()
}

fn complexity_sum(codes: &[String]) -> usize {
    codes.iter().map(|c| complexity(c)).sum()
}

fn complexity_sum_b(codes: &[String]) -> usize {
    codes.iter().map(|c| complexity_b(c)).sum()
}

fn main() {
    let codes = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    //println!("{:?}", complexity("029A")); // 68
    //println!("{:?}", complexity("980A")); // 60
    //println!("{:?}", complexity("179A")); // 68
    //println!("{:?}", complexity("456A")); // 64
    //println!("{:?}", complexity("379A")); // 64
    println!("{:?}", complexity_sum(&codes));
    println!("{:?}", complexity_sum_b(&codes));
}
