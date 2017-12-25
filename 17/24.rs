use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::ops::Add;

type Pins = usize;

#[derive(Debug)]
struct Component {
    a: Pins,
    b: Pins
}

fn parse_line(line: &str) -> Component {
    let mut parts = line.split("/");
    let a = parts.next().unwrap().parse().unwrap();
    let b = parts.next().unwrap().parse().unwrap();
    Component { a: a, b: b }
}

fn _drawgraphviz(strewn: &[Component]) {
    println!("graph {{");
    //let conns = Vec::new();
    for (i, c) in strewn.iter().enumerate() {
        println!("c{:?} [label=\"{}/{}{}\"];", i, c.a, c.b, if c.a * c.b == 0 { "AAAAAAAAAAAA" } else { ""});
        for (j, cc) in strewn.iter().enumerate().skip(i) {
            if c.a == cc.a || c.a == cc.b || c.b == cc.a || c.b == cc.b {
                println!("c{} -- c{} [label=\"{}/{}-{}/{}\"];",
                         i, j,
                         c.a, c.b, cc.a, cc.b
                         );
            }
        }
    }
    println!("}}");
}

type ConnsList = Vec<(usize, bool)>;

fn dfs_level<Score: Ord + Add<Output = Score>, ScoreFn>(strewn: &[Component], ci: usize, in_port_a: bool,
       conns: &[(ConnsList, ConnsList)], visited: &mut [bool], score: &ScoreFn) -> Option<Score>
    where ScoreFn: Fn(&Component) -> Score {
    let c = &strewn[ci];
    if visited[ci] {
        return None;
    }
    visited[ci] = true;

    let neigh_iter = if in_port_a {
        conns[ci].1.iter()
    } else {
        conns[ci].0.iter()
    };

    let child_scores = neigh_iter.filter_map(
        |&(neigh_ci, neigh_in)| dfs_level(strewn, neigh_ci, neigh_in, conns, visited, score)
    ).max();
    visited[ci] = false; // mark this free for further calls; alternatively a clone of visited for each call could work

    if let Some(child_scores) = child_scores {
        Some(score(c) + child_scores)
    } else {
        // no neighbors - leaf node in this component domino chain
        Some(score(c))
    }
}

// could also model this as a tricky trigraph for three nodes for each component; then dfs_level
// and stuff would be easier
fn compute_connectivity(strewn: &[Component]) -> Vec<(ConnsList, ConnsList)> {
    let mut conns = Vec::new(); // list of indices from either port
    for (i, c) in strewn.iter().enumerate() {
        let (mut a, mut b) = (Vec::new(), Vec::new());
        for (j, cc) in strewn.iter().enumerate() {
            if j == i {
                continue;
            }
            if c.a == cc.a {
                a.push((j, true));
            }
            if c.a == cc.b {
                a.push((j, false));
            }
            if c.b == cc.a {
                b.push((j, true));
            }
            if c.b == cc.b {
                b.push((j, false));
            }
        }
        conns.push((a, b));
    }
    conns
}

fn dfs_max<Score, ScoreFn>(strewn: &[Component], score: &ScoreFn) -> Score
where Score: Ord + Add<Output = Score>,
      ScoreFn: Fn(&Component) -> Score {
    let conns = compute_connectivity(strewn);
    let mut visited = vec![false; strewn.len()];
    let a_start_compos = strewn.iter().enumerate().filter(|&(_, c)| c.a == 0).map(|(ci, _)| (ci, true));
    let b_start_compos = strewn.iter().enumerate().filter(|&(_, c)| c.b == 0).map(|(ci, _)| (ci, false));
    a_start_compos.chain(b_start_compos)
        .filter_map(|(ci, start_a)| dfs_level(strewn, ci, start_a, &conns, &mut visited, score))
        .max().unwrap()
}

fn strongest_bridge(strewn: &[Component]) -> usize {
    let strength_score = |c: &Component| c.a + c.b;
    dfs_max(strewn, &strength_score)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct LengthStrength(usize, usize);

impl Add for LengthStrength {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        LengthStrength(self.0 + other.0, self.1 + other.1)
    }
}

fn strongest_long_bridge(strewn: &[Component]) -> LengthStrength {
    let longest_plus_strength_score = |c: &Component| LengthStrength(1, c.a + c.b);
    dfs_max(strewn, &longest_plus_strength_score)
}

fn main() {
    let compo_strewn = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    println!("{}", strongest_bridge(&compo_strewn));
    println!("{:?}", strongest_long_bridge(&compo_strewn));
}
