use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

const ROOM: char = '.';
const WALL: char = '#';
const DOORH: char = '-';
const DOORV: char = '|';

type Nodes = HashSet<(i32, i32)>;
// left coord has always smaller x and y
type Edges = HashSet<((i32, i32), (i32, i32))>;
struct Graph {
    n: Nodes,
    e: Edges,
}

fn travel(instructions: &[char], mut x: i32, mut y: i32, g: &mut Graph) -> usize {
    let mut i = 0;
    let visit = |g: &mut Graph, xa: i32, ya: i32, xb: i32, yb: i32| {
        g.n.insert((xb, yb));
        g.e.insert(((xa.min(xb), ya.min(yb)), (xb.max(xa), yb.max(ya))));
    };
    g.n.insert((x, y));
    loop {
        let dir = instructions[i];
        match dir {
            '^' => (),
            '$' => return i,
            'N' => { y -= 1; visit(g, x, y + 1, x, y); },
            'S' => { y += 1; visit(g, x, y - 1, x, y); },
            'E' => { x += 1; visit(g, x - 1, y, x, y); },
            'W' => { x -= 1; visit(g, x + 1, y, x, y); },
            '(' => {
                i += 1;
                loop {
                    let n = travel(&instructions[i..], x, y, g);
                    i += n;
                    match instructions[i] {
                        '|' => i += 1,
                        ')' => break,
                        _ => unreachable!()
                    }
                }
            },
            ')' | '|' => return i,
            _ => unreachable!()
        }
        i += 1;
    }
}

// this is completely unnecessary for the result, but fun
fn render(g: &Graph) {
    let minx = g.n.iter().map(|&(x, _)| x).min().unwrap();
    let maxx = g.n.iter().map(|&(x, _)| x).max().unwrap();
    let miny = g.n.iter().map(|&(_, y)| y).min().unwrap();
    let maxy = g.n.iter().map(|&(_, y)| y).max().unwrap();
    /*    -1 0
     *    #####
     * -1 #.|.#
     *    #-###
     *  0 #.|X#
     *    #####
     */
    // note: outer cells (up, down, left, right) are always walls
    let w = (maxx - minx + 1) as usize;
    // meh, fill has to be a character so # is used explicitly instead of WALL :(
    // figuring out the format syntax was "fun" too.
    println!("{0:#<1$}", WALL, 2 * w + 1);
    for y in miny..=maxy {
        print!("{}", WALL);
        // room line
        for x in minx..=maxx {
            if g.n.contains(&(x, y)) {
                print!("{}", ROOM);
            } else {
                // doesn't seem to happen in sample inputs - the world is a nice grid
                panic!()
            }
            if x < maxx {
                if g.e.contains(&((x, y), (x + 1, y))) {
                    print!("{}", DOORV);
                } else {
                    print!("{}", WALL);
                }
            }
        }
        println!("{}", WALL);
        // horizontal wall line
        if y < maxy {
            print!("{}", WALL);
            for x in minx..=maxx {
                if g.e.contains(&((x, y), (x, y + 1))) {
                    print!("{}", DOORH);
                } else {
                    print!("{}", WALL);
                }
                if x < maxx {
                    print!("{}", WALL);
                }
            }
            println!("{}", WALL);
        }
    }
    println!("{0:#<1$}", WALL, 2 * w + 1);
}

type DistMap = HashMap<(i32, i32), usize>;

fn bfs(g: &Graph) -> DistMap {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    queue.push_back((0, 0, 0));
    distances.insert((0, 0), 0);

    while let Some(current) = queue.pop_front() {
        let (xi, yi, dist) = current;

        for &(xj, yj) in &[(xi - 1, yi), (xi + 1, yi), (xi, yi - 1), (xi, yi + 1)] {
            let edge = ((xi.min(xj), yi.min(yj)), (xi.max(xj), yi.max(yj)));
            let unknown = !distances.contains_key(&(xj, yj));
            let passable = g.e.contains(&edge);
            if unknown && passable {
                queue.push_back((xj, yj, dist + 1));
                distances.insert((xj, yj), dist + 1);
            }
        }
    }

    distances
}

fn furthest_room(g: &Graph) -> usize {
    let dists = bfs(g);
    *dists.values().max().unwrap()
}

fn faraway_rooms(g: &Graph) -> usize {
    let dists = bfs(g);
    dists.values().filter(|&&d| d >= 1000).count()
}

fn main() {
    let guide = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|l| l.unwrap().chars().collect::<Vec<char>>()).collect::<Vec<_>>();
    for regex in &guide {
        let mut g = Graph { n: Nodes::new(), e: Edges::new() };
        travel(regex, 0, 0, &mut g);
        if false {
            for &n in &g.n {
                println!("node {:?}", n);
            }
            for &e in &g.e {
                println!("edge {:?}", e);
            }
        }
        render(&g);
        println!("{}", furthest_room(&g));
        println!("{}", faraway_rooms(&g));
    }
}
