use std::io::{self, BufRead};

type Pos = (i64, i64);

fn area(a: Pos, b: Pos) -> i64 {
    ((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1)
}

fn greatest_area(tiles: &[Pos]) -> i64 {
    tiles.iter()
        .enumerate()
        .flat_map(|(i, &a)| tiles.iter().skip(i + 1).map(move |&b| area(a, b)))
        .max().unwrap()
}

fn axis_flip(p: Pos) -> Pos {
    (p.1, p.0)
}

// a > b completely, "inside box from this corner"
fn gt(a: Pos, b: Pos) -> bool {
    a.0 > b.0 && a.1 > b.1
}

// as a and b specify a rectangle, get top left and bottom right
fn psort(a: Pos, b: Pos) -> (Pos, Pos) {
    ((a.0.min(b.0), a.1.min(b.1)), (a.0.max(b.0), a.1.max(b.1)))
}

// e slices box vertically? don't consider left/right edges
fn vert_crosses(e: (Pos, Pos), a: Pos, b: Pos) -> bool {
    e.0.0 > a.0 && e.1.0 < b.0 && e.0.1 <= a.1 && e.1.1 >= b.1
}

// e.0 inside box, not on the edges?
fn head_crosses(e: (Pos, Pos), a: Pos, b: Pos) -> bool {
    gt(e.0, a) && gt(b, e.0)
}

fn crosses(e: (Pos, Pos), a: Pos, b: Pos) -> bool {
    let (a, b) = psort(a, b);
    let e = psort(e.0, e.1);
    let x = head_crosses(e, a, b);
    let y = head_crosses((e.1, e.0), a, b);
    let z = vert_crosses(e, a, b);
    let w = vert_crosses((axis_flip(e.0), axis_flip(e.1)), axis_flip(a), axis_flip(b));
    x || y || z || w
}

fn crossing(edges: &[(Pos, Pos)], a: Pos, b: Pos) -> bool {
    edges.iter().any(|&e| crosses(e, a, b))
}

fn greatest_noncrossing_area(tiles: &[Pos]) -> i64 {
    // could just make these as we go but this is maybe simpler
    let edges = tiles.iter()
        .zip(tiles.iter().cycle().skip(1))
        .map(|(&a, &b)| (a, b))
        .collect::<Vec<(Pos, Pos)>>();
    tiles.iter()
        .enumerate()
        .flat_map(|(i, &a)| tiles.iter().skip(i + 1).map(move |&b| (a, b)))
        .filter(|&(a, b)| !crossing(&edges, a, b))
        .map(|(a, b)| area(a, b))
        .max().unwrap()
}

fn parse(line: &str) -> Pos {
    let ab = line.split_once(',').unwrap();
    (ab.0.parse().unwrap(), ab.1.parse().unwrap())
}

fn main() {
    let tiles = io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap()))
        .collect::<Vec<_>>();
    println!("{}", greatest_area(&tiles));
    println!("{}", greatest_noncrossing_area(&tiles));
}
