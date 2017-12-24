use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

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

fn drawgraphviz(strewn: &[Component]) {
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

fn dfs(level: usize, strewn: &[Component], ci: usize, in_port_a: bool, conns: &[(Vec<(usize, bool)>, Vec<(usize, bool)>)], visited: &mut [bool]) -> usize {
    let c = &strewn[ci];
    if visited[ci] {
        return 0;
    }
    //println!("{:->1$} lev {2} {3}/{4}", "", level, level, c.a, c.b);
    visited[ci] = true;
    let ret = if in_port_a {
        c.a + c.b + conns[ci].1.iter()
            .map(|&(neigh_ci, neigh_in)| dfs(level+1, strewn, neigh_ci, neigh_in, conns, visited)).max().unwrap_or(0)
    } else {
        c.a + c.b + conns[ci].0.iter()
            .map(|&(neigh_ci, neigh_in)| dfs(level+1, strewn, neigh_ci, neigh_in, conns, visited)).max().unwrap_or(0)
    };
    //println!("{:->1$} lev {2} {3}/{4} skore {5}", "", level, level, c.a, c.b, ret);
    visited[ci] = false; // mark this free for further calls; alternatively a clone of visited for each call could work
    ret
}

// ugh but it doesn't make sense to dfs these really? at least not with a simple visited map...
// however, a proper search might not be that heavy in this graph, looks like so
fn depth(strewn: &[Component]) -> usize {
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
    let mut visited = vec![false; strewn.len()];
    let amax = strewn.iter().enumerate().filter(|&(_, c)| c.a == 0).map(|(ci, _)| dfs(0, strewn, ci, true, &conns, &mut visited)).max().unwrap_or(0);
    let bmax = strewn.iter().enumerate().filter(|&(_, c)| c.b == 0).map(|(ci, _)| dfs(0, strewn, ci, false, &conns, &mut visited)).max().unwrap_or(0);
    amax.max(bmax)
}

fn main() {
    let compo_strewn = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    println!("{}", depth(&compo_strewn));
}
