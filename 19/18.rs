use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque, BinaryHeap};

type Map = Vec<Vec<char>>;

#[derive(Debug, Clone)]
struct World {
    map: Map,
    // Player origin is replaced with '.', coordinate stored separately for easier printing
    player_x: usize,
    player_y: usize,
}

fn dump(world: &World) {
    for (y, row) in world.map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if (x, y) == (world.player_x, world.player_y) {
                print!("@");
            } else {
                print!("{}", col);
            }
        }
        println!();
    }
}

fn is_door(tile: char) -> bool {
    tile >= 'A' && tile <= 'Z'
}

fn is_key(tile: char) -> bool {
    tile >= 'a' && tile <= 'z'
}

fn key_for_door(doortile: char) -> char {
    assert!(is_door(doortile));
    (doortile as u8 - b'A' + b'a') as char
}

// search state includes which keys the player holds; the alphabet fits in 32 bits, so store them
// in a bitmap instead of some collection that would need allocation or a largeish store (also a
// fun rust exercise)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Keys(u32);

impl Keys {
    fn new() -> Self {
        Keys(0)
    }
    fn from_label(label: char) -> Keys {
        assert!(is_key(label));
        let nth = (label as u8 - b'a') as u32;
        Keys(1 << nth)
    }
    fn insert_all(&mut self, rhs: Keys) {
        self.0 |= rhs.0
    }
    fn insert(&mut self, label: char) {
        self.insert_all(Keys::from_label(label));
    }
    fn contains_all(&self, rhs: Keys) -> bool {
        self.0 | rhs.0 == self.0
    }
    fn contains(&self, label: char) -> bool {
        self.contains_all(Keys::from_label(label))
    }
}

impl std::fmt::Debug for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Keys(")?;
        for i in b'a'..=b'z' {
            let label = i as char;
            if self.contains(label) {
                write!(f, "{}", label)?;
            }
        }
        write!(f, ")")
    }
}

#[derive(Debug, Clone, Copy)]
struct Doors { keys: Keys }

impl Doors {
    fn new() -> Self {
        Doors { keys: Keys::new() }
    }
}

type BfsDistance = (usize, Keys, Doors);
type Vec2 = (usize, usize);

// distance encodes also keys and doors in the way
fn raw_bfs(map: &Map, origin: Vec2) -> HashMap<Vec2, BfsDistance> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    queue.push_back((origin, (0, Keys::new(), Doors::new())));
    distances.insert(origin, (0, Keys::new(), Doors::new()));

    while let Some(current) = queue.pop_front() {
        let ((xi, yi), (dist, keys, doors)) = current;
        // other ends of the graph "edge" from (xi, yi)
        // note that the outermost tiles are always wall so these won't overflow
        let steps = [
            (xi - 1, yi),
            (xi + 1, yi),
            (xi, yi - 1),
            (xi, yi + 1)
        ];
        for &nextpos in steps.into_iter() {
            let unknown = !distances.contains_key(&nextpos);
            let tile = map[nextpos.1][nextpos.0];
            let open = tile != '#';
            if unknown && open {
                let mut nextdoors = doors.clone();
                if is_door(tile) {
                    nextdoors.keys.insert(key_for_door(tile));
                }
                let mut nextkeys = keys;
                if is_key(tile) {
                    nextkeys.insert(tile);
                }
                distances.insert(nextpos, (dist + 1, nextkeys, nextdoors.clone()));
                // travel also over keys (walkable, but interesting) and doors (not necessarily
                // walkable yet); they're marked in the distance information as stuff needed on the
                // way along the edge between the origin and the dest node
                queue.push_back((nextpos, (dist + 1, nextkeys, nextdoors)));
            }
        }
    }

    distances
}

// (destnode, (distance, keys, doors))
type DistanceMap = HashMap<char, BfsDistance>;
// adjacency list as (srcnode, distances)
type DistanceList = HashMap<char, DistanceMap>;

// map of everything, not just direct neighbors
fn bfs_places(map: &Map, origin: Vec2) -> DistanceMap {
    let distances = raw_bfs(map, origin);
    // the found distances include also every floor tile and the source; only the points of
    // interest are useful. Convert also coordinates into tile labels because they're now unique
    let interesting = distances.into_iter()
        .filter(|((x, y), (dist, _keys, _doors))| is_key(map[*y][*x]))
        .map(|((x, y), distinfo)| (map[y][x], distinfo)).collect();
    // (this prints player origin as '.')
    println!("from {}: {:?}", map[origin.1][origin.0], interesting);
    interesting
}

// this is for history data: the raw bfs knows which doors need to be traversed to get somewhere,
// but we're interested in what keys we have collected - not which doors we've passed
type GdistMap = HashMap<char, (usize, Keys)>;

// try abstract paths like abcd, adbc, dcba, find shortest path that visits all keys
fn graph_dfs(dl: &DistanceList, origin: char) -> usize {
    // dist negated so that bigger would be better; we want the shortest
    let mut heap: BinaryHeap<(i64, char, Keys)> = BinaryHeap::new(); // -dist, ch, equip
    let mut distances: GdistMap = dl.iter()
        .map(|(&ch, _dist)| {
            let k = if is_key(ch) { Keys::from_label(ch) } else { Keys::new() };
            (ch, (std::usize::MAX, k))
        }).collect();

    // for a candidate node, test if a) might have better score and b) is reachable
    // NOTE: using keys_i because a "shortcut edge" that goes over keys and doors might have those
    // keys and doors in any order; consider a -> c vs. a -> A -> B -> b -> c
    let test_and_insert = |ch_j: char, doors_j: Doors,
            keys_i: Keys, keys_j: Keys, dist_j: usize,
            distances: &mut GdistMap, heap: &mut BinaryHeap<_>, insertmode: bool| {
        let (old_dist, old_keys) = distances[&ch_j];
        // we're going to have keys_j, does it have something new compared to old_keys?
        // it's ok if old has more than new but old must not be missing any of new
        let new_keys_exist = !old_keys.contains_all(keys_j);
        let should_visit = dist_j < old_dist || new_keys_exist;
        // is this edge valid from this state? that is, do we carry the key from the start?
        let proper_eq = keys_i.contains_all(doors_j.keys);

        // this doesn't seem to be very useful
        if !insertmode {
            // optimization short-circuit mode - visit equal?
            let should_visit = dist_j <= old_dist || new_keys_exist;
            should_visit
        } else if should_visit && proper_eq {
            //println!("  look YES for {} some {} steps away from origin", ch_j, dist_j);
            //println!("  old keys {:?} dist {}", old_keys, old_dist);
            //println!("  new keys {:?} dist {}", keys_j, dist_j);
            heap.push((-(dist_j as i64), ch_j, keys_j));
            distances.insert(ch_j, (dist_j, keys_j));
            true
        } else if !should_visit {
            //println!("  look NO1 for {} some {} steps away from origin | no better", ch_j, dist_j);
            //println!("  old keys {:?} dist {}", old_keys, old_dist);
            //println!("  new keys {:?} dist {}", keys_j, dist_j);
            false
        } else if !proper_eq {
            //println!("  look NO2 for {} some {} steps away from origin | no equipment", ch_j,
            //dist_j);
            //println!("  {:?}", doors_j);
            //println!("  {:?}", keys_j);
            false
        } else {
            panic!("logic error");
        }
    };

    distances.insert(origin, (0, Keys::new()));
    heap.push((0, origin, Keys::new()));

    let all_keys = dl.keys() // note: keys are not quite like keys
        .filter(|&&k| is_key(k))
        .fold(Keys::new(), |mut keychain, &x| { keychain.insert(x); keychain });

    while let Some(current) = heap.pop() {
        if false {
            // note: this shortcut doesn't seem useful
            if distances.iter().all(|(&place, &(dist, keys))| keys == all_keys) {
                break;
            }
        }

        let (dist_i, ch_i, keys_i) = current;
        let dist_i = (-dist_i) as usize;
        //println!("visit dist {:?} ch {:?} keys {:?} | distmap size is {}", dist_i, ch_i, keys_i, distances.len());
        //println!("  distances are {:?}", distances);
        if false {
            // this doesn't seem to speed up much either
            if !test_and_insert(ch_i, Doors::new(), keys_i, keys_i, dist_i, &mut distances, &mut heap, false) {
                //println!("  too expensive");
                //println!("  new {:?}", (dist_i, keys_i));
                //println!("  old {:?}", distances.get(&ch_i).unwrap());
                continue;
            }
        }
        //println!("  neighs are {:?}", dl.get(&ch_i).unwrap());

        // dl has *all* things, not just reachable - reachability is checked separately
        for (&ch_j, &(dist_ij, keys_ij, doors_j)) in &dl[&ch_i] {
            let dist_j = dist_i + dist_ij;
            let mut keys_j = keys_i;
            // about keys:
            // - the way to these neighbors might walk over other keys or through doors
            // - to avoid order checking, go through doors only if initial key set contains them
            // - collect all keys on the way though; just don't consider them for doors of this dest
            keys_j.insert_all(keys_ij);

            test_and_insert(ch_j, doors_j, keys_i, keys_j, dist_j, &mut distances, &mut heap, true);
        }
    }
    println!("{:?}", distances);

    distances.values()
        .filter(|&&(_dist, keys)| keys == all_keys)
        .map(|&(dist, _keys)| dist).min().unwrap()
}

fn shortest_keypath(world: World) -> usize {
    // (pos, label) of all non-wall, non-empty
    let mut sightseeing: Vec<(Vec2, char)> = world.map.iter().enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate()
                .filter(|&(_x, &ch)| is_key(ch))
                .map(move |(x, &ch)| ((x, y), ch))
        }).collect();
    sightseeing.push(((world.player_x, world.player_y), '@'));

    // le debug
    let placenames: Vec<char> = sightseeing.iter().map(|&(_pos, ch)| ch).collect();
    println!("found {:?}", placenames);

    // map of everything, not just direct neighbors
    let dl: DistanceList = sightseeing.iter().map(|&((x, y), ch)| {
        (ch, bfs_places(&world.map, (x, y)))
    }).collect();

    graph_dfs(&dl, '@')
}

fn parse_world(mut map: Vec<Vec<char>>) -> World {
    for (y, row) in map.iter_mut().enumerate() {
        for (x, col) in row.iter_mut().enumerate() {
            if *col == '@' {
                *col = '.';
                return World { map, player_x: x, player_y: y };
            }
        }
    }
    panic!("no game");
}

fn main() {
    let input: Vec<Vec<char>> = io::stdin().lock().lines().map(|line|
        line.unwrap().chars().collect()).collect();
    let world = parse_world(input);
    dump(&world);
    println!("{}", shortest_keypath(world));
}
