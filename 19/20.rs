use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque, BinaryHeap};

type Map = Vec<Vec<char>>;

fn dump(map: &Map) {
    for row in map {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn is_alpha(ch: char) -> bool {
    ch >= 'A' && ch <= 'Z'
}

// the map vecs are indexed by usize, but sometimes signed integers are handy too :(
type Vec2 = (usize, usize);

// teleports are nodes on the map with two-character names and x,y to distinguish which is which
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Portal {
    label: (char, char),
    begin: Vec2,
    end: Vec2,
}

impl Portal {
    fn new(label: (char, char), begin: Vec2, end: Vec2) -> Self {
        Portal { label, begin, end }
    }
}

impl std::fmt::Debug for Portal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}@({},{})", self.label.0, self.label.1, self.begin.0, self.begin.1)
    }
}

fn point_of_interest(map: &Map, x: usize, y: usize, require_head: bool) -> Option<Portal> {
    let ch = map[y][x];
    if ch == '.' || ch == '#' || ch == ' ' {
        None
    } else {
        assert!(is_alpha(ch));
        if is_alpha(map[y + 1][x]) {
            // this is the top of a vertical port name
            Some(Portal::new((map[y][x], map[y + 1][x]), (x, y), (x, y + 1)))
        } else if is_alpha(map[y][x + 1]) {
            // this is the left of a horizontal port name
            Some(Portal::new((map[y][x], map[y][x + 1]), (x, y), (x + 1, y)))
        } else if !require_head && is_alpha(map[y - 1][x]) {
            // this is the bottom of a vertical port name
            Some(Portal::new((map[y - 1][x], map[y][x]), (x, y - 1), (x, y)))
        } else if !require_head && is_alpha(map[y][x - 1]) {
            // this is the right of a horizontal port name
            Some(Portal::new((map[y][x - 1], map[y][x]), (x - 1, y), (x, y)))
        } else {
            None
        }
    }
}

/*
 *   #
 * XY. <- start from the dot
 *   #
 */
fn port_entrance(map: &Map, port: Portal) -> Vec2 {
    // try the surrounding area of both heads; the map seems to have a unique entrance
    for dy in -1..=1 {
        for dx in -1..=1 {
            let xa = (port.begin.0 as i32 + dx) as usize;
            let ya = (port.begin.1 as i32 + dy) as usize;
            let xb = (port.end.0 as i32 + dx) as usize;
            let yb = (port.end.1 as i32 + dy) as usize;
            if map[ya][xa] == '.' {
                return (xa, ya);
            } else if map[yb][xb] == '.' {
                return (xb, yb);
            }
        }
    }
    panic!("isolated teleport");
}

fn raw_bfs(map: &Map, source_port: Portal) -> HashMap<Vec2, usize> {
    let orig_pos = port_entrance(map, source_port);
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    queue.push_back((orig_pos, 0));
    distances.insert(orig_pos, 0);

    while let Some(current) = queue.pop_front() {
        let ((xi, yi), dist) = current;
        // other ends of the graph "edge" from (xi, yi)
        let steps = &[
            (xi - 1, yi),
            (xi + 1, yi),
            (xi, yi - 1),
            (xi, yi + 1)
        ];
        for &nextpos in steps {
            let unknown = !distances.contains_key(&nextpos);
            let tile = map[nextpos.1][nextpos.0];
            // each port is two tiles big; filter out edges to itself
            let same_port = nextpos == source_port.begin || nextpos == source_port.end;
            let open = tile != '#' && tile != ' ' && !same_port;
            if unknown && open {
                if !is_alpha(tile) {
                    // open space, go for it
                    queue.push_back((nextpos, dist + 1));
                }
                // portals act like a wall to walking around but we can reach them at this point;
                // distance does not change, this is the entrance point in front of the portal
                distances.insert(nextpos, dist);
            }
        }
    }

    distances
}

type DestinationMap = HashMap<Portal, usize>;
type DistanceList = HashMap<Portal, DestinationMap>;

fn bfs_places(map: &Map, origin: Portal) -> DestinationMap {
    let distances = raw_bfs(map, origin);
    let interesting = distances.into_iter()
        .filter_map(|((x, y), dist)|
            // consider bot heads visitable because we don't know yet which is the right way,
            // FIXME: maybe just entrance? all have just one
            point_of_interest(map, x, y, false).map(|port| (port, dist))
        )
        .collect();
    println!("from {:?}: {:?}", origin, interesting);
    interesting
}

type GdistMap = HashMap<Portal, usize>;

fn portal_dijkstra(dl: &DistanceList, origin: Portal) -> usize {
    let mut heap: BinaryHeap<(i64, Portal)> = BinaryHeap::new(); // -dist, node
    let mut distances: GdistMap = dl.iter()
        .map(|(&port, _dist)| {
            (port, std::usize::MAX)
        }).collect();

    distances.insert(origin, 0);
    heap.push((0, origin));

    while let Some(current) = heap.pop() {
        let (dist_i, port_i) = current;
        let dist_i = (-dist_i) as usize;

        for (&port_j, &dist_ij) in &dl[&port_i] {
            let dist_j = dist_i + dist_ij;
            if dist_j < distances[&port_j] {
                heap.push((-(dist_j as i64), port_j));
                // XXX: maybe the entry api would hash this just once? look into it
                distances.insert(port_j, dist_j);
            }
        }
    }

    distances.into_iter()
        .find(|&(port, _dist)| port.label == ('Z', 'Z'))
        .map(|(_port, dist)| dist)
        .unwrap()
}

// AA and ZZ have no pairs
fn port_pair(a: Portal, ports: &[Portal]) -> Option<Portal> {
    ports.iter().find(|&&p| p.label == a.label && p != a).cloned()
}

fn step_maze(map: &Map) -> usize {
    // the dist list keys don't need the portal end positions
    let sightseeing: Vec<Portal> = (0..map.len()).flat_map(|y| {
        (0..map[0].len()).filter_map(move |x|
            point_of_interest(map, x, y, true)
        )
    }).collect();

    let mut dl: DistanceList = sightseeing.iter().map(|&port| {
            (port, bfs_places(map, port))
    }).collect();

    // note: AA and ZZ have no pairs
    let port_pairs = sightseeing.iter().filter_map(|&srcport| {
        port_pair(srcport, &sightseeing).map(|other_side| (srcport, other_side))
    });
    for (src, dst) in port_pairs {
        dl.entry(src).and_modify(|map| { map.insert(dst, 1); }); // teleport travel costs one
    }

    // player starts at AA that is unique
    portal_dijkstra(&dl, *sightseeing.iter().find(|&p| p.label == ('A', 'A')).unwrap())
}

fn parse_map(raw_map: Vec<Vec<char>>) -> Map {
    let mut map = Vec::new();
    // the raw coordinates are not important as such. Expand the map with a sentinel border as
    // there isn't any in these inputs; this makes indexing easier
    let new_wid = raw_map[0].len() + 2;
    map.push(vec!['#'; new_wid]);
    map.extend(raw_map.into_iter().map(|row| {
        // why no slices ok to chain?
        vec!['#'].into_iter()
            .chain(row.into_iter())
            .chain(vec!['#'].into_iter())
            .collect()
    }));
    map.push(vec!['#'; new_wid]);
    map
}

fn main() {
    let input: Vec<Vec<char>> = io::stdin().lock().lines().map(|line|
        line.unwrap().chars().collect()).collect();
    let map = parse_map(input);

    dump(&map);
    println!("{}", step_maze(&map));
}
