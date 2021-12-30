use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap, BinaryHeap};
use std::cmp::Reverse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    x: i8,
    y: i8,
    id: char,
}

impl Node {
    fn new(x: i8, y: i8, id: u8) -> Node {
        Node { x, y, id: id as char }
    }
    fn walk_distance(&self, other: Node) -> i32 {
        (self.x - other.x).abs() as i32 + (self.y - other.y).abs() as i32
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashSet<Node>,
    edges: HashMap<Node, HashSet<Node>>,
    empty: HashSet<Node>,
}

impl Graph {
    fn new(nodes: Vec<Node>, edges: Vec<(Node, Node)>) -> Graph {
        let mut g = Graph { nodes: nodes.iter().copied().collect(), edges: HashMap::new(), empty: HashSet::new() };
        for (na, nb) in edges {
            g.edges.entry(na.clone()).or_insert(HashSet::new()).insert(nb.clone());
            g.edges.entry(nb).or_insert(HashSet::new()).insert(na);
        }
        g
    }

    fn neighs(&self, node: &Node) -> &HashSet<Node> {
        self.edges.get(node).unwrap_or(&self.empty)
    }

    fn node_at(&self, pos: (i8, i8)) -> Node {
        *self.nodes.iter()
            .find(|n| n.x == pos.0 && n.y == pos.1)
            .unwrap()
    }

    fn node_by_id(&self, id: char) -> Node {
        *self.nodes.iter()
            .find(|n| n.id == id)
            .unwrap()
    }
}

/*
 * hardcoded map for the network
 *
 * the first rule of never stopping directly in front of a room is encoded in the graph structure
 */
fn empty_map(extended: bool) -> Graph {
    let (height, map, route): (usize, &[u8], &[&str]) = if extended {
        (6, "\
        #############\
        #ef.g.h.i.jk#\
        ###l#m#n#o###\
        ###p#q#r#s###\
        ###t#u#v#w###\
        ###x#y#z#{###".as_bytes(),
        &[
            "ef", "fg", "gh", "hi", "ij", "jk",
            "fl", "gl", "lp",
            "gm", "hm", "mq",
            "hn", "in", "nr",
            "io", "jo", "os",
            "pt", "qu", "rv", "sw",
            "tx", "uy", "vz", "w{"
        ])
    } else {
        (4, "\
        #############\
        #ef.g.h.i.jk#\
        ###l#m#n#o###\
        ###p#q#r#s###".as_bytes(),
        &[
            "ef", "fg", "gh", "hi", "ij", "jk",
            "fl", "gl", "lp",
            "gm", "hm", "mq",
            "hn", "in", "nr",
            "io", "jo", "os"
        ])
    };
    let nodes = (0..height)
        .flat_map(|y| (0..13).map(move |x| (x, y, map[y * 13 + x])))
        .filter(|&(_x, _y, id)| id >= b'a' && id <= b'{')
        .map(|(x, y, id)| Node::new(x as i8, y as i8, id))
        .collect::<Vec::<_>>();
    let n_by_id = |id| *nodes.iter().find(|n| n.id == id).unwrap();
    let edges: Vec<_> = route.iter()
        .map(|pair| pair.as_bytes())
        .map(|bs| (n_by_id(bs[0] as char), n_by_id(bs[1] as char)))
        .collect();
    Graph::new(nodes, edges)
}

// also the return type of walk_distance()
type Cost = i32;
const MAX_COST: Cost = std::i32::MAX;

fn step_energy(pod_name: char) -> Cost {
    match pod_name {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("what pod")
    }
}

fn homeplace(name: char) -> i8 {
    match name {
         'A' => 3,
         'B' => 5,
         'C' => 7,
         'D' => 9,
        _ => panic!("bad name")
    }
}

// this might be dumb, storing the network state in one vec could also work and would probably
// speed up due to ambiguous states where the same kinds of pods are just swapped
// also the destination state is ambiguous, there are multiple good ones
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
struct PodState {
    name: char, // name is not unique, used only for debug visualization and movement cost
    node: Node,
}

impl PodState {
    fn new(name: char, node: Node) -> PodState {
        PodState { name, node }
    }
    fn cost_heuristic(&self) -> Cost {
        let xdiff = (self.node.x - homeplace(self.name)).abs() as i32;
        if true {
            if xdiff == 0 {
                0
            } else {
                let down = 1; // move down at least once from the hallway
                let up = if self.node.y > 1 {
                    // climb to the hallway
                    (self.node.y - 1) as i32
                } else {
                    // in hallway, no need to climb up again
                    0
                };
                (up + xdiff + down) * step_energy(self.name)
            }
        } else {
            xdiff * step_energy(self.name)
        }
    }
}

type PodList = Vec::<PodState>;

// kind of stupid to have ord for this, the ordering of identical costs (which happens when the
// heap finds the state next in the tuple to be compared) does not matter. maybe it doesn't happen
// often though.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    pods: PodList,
}

impl State {
    fn pod_by_node(&self, n: Node) -> Option<PodState> {
        self.pods.iter().find(|ps| ps.node == n).map(|&x| x)
    }

    fn occupied(&self, node: Node) -> bool {
        self.pods.iter().any(|p| p.node == node)
    }
}

fn dump_state(world: &Graph, pods: &PodList) {
    let part1 = pods.len() == 8;

    let map = if part1 {
        "\
        #############\n\
        #ef.g.h.i.jk#\n\
        ###l#m#n#o###\n\
        ###p#q#r#s###\n"
    } else {
        "\
        #############\n\
        #ef.g.h.i.jk#\n\
        ###l#m#n#o###\n\
        ###p#q#r#s###\n\
        ###t#u#v#w###\n\
        ###x#y#z#{###\n"
    };
    for ch in map.chars() {
        let pr = if ch >= 'a' && ch <= '{' {
            let n = world.node_by_id(ch);
            if let Some(p) = pods.iter().find(|p| p.node == n) {
                p.name
            } else {
                '.'
            }
        } else {
            ch
        };
        print!("{}", pr);
    }
}

fn hallway_node(node: Node) -> bool {
    node.y == 1
}

fn room_kind(node: Node) -> char {
    assert!(!hallway_node(node));
    match node.x {
        3 => 'A',
        5 => 'B',
        7 => 'C',
        9 => 'D',
        _ => panic!("bad room for kind")
    }
}

fn own_room(node: Node, name: char) -> bool {
    room_kind(node) == name
}

// comfort zone with possibly deep rooms: no foreigner there
fn safe_looking_room(state: &State, spot: Node, player: PodState) -> bool {
    let mut room_folk = state.pods.iter().filter(|ps| !hallway_node(ps.node) && ps.node.x == spot.x);
    room_folk.all(|ps| ps.name == player.name)
}

// all below spots must be filled by own people.
// not empty.
// not foreigners.
// assumes own_room().
fn final_looking_spot(world: &Graph, state: &State, player: PodState) -> bool {
    let mut same_room_below = world.nodes.iter().filter(|n| n.x == player.node.x && n.y > player.node.y);
    same_room_below.all(|n| {
        state.pod_by_node(*n).map(|ps| ps.name == player.name).unwrap_or(false)
    })
}

fn unobstructed_destinations_visit(world: &Graph, state: &State, current_node: Node, current_cost: Cost, neighmap: &mut HashMap<Node, Cost>) {
    if neighmap.contains_key(&current_node) && *neighmap.get(&current_node).unwrap() <= current_cost {
        return;
    }
    neighmap.insert(current_node, current_cost);

    for &n in world.neighs(&current_node) {
        if !state.occupied(n) {
            let edge_cost = current_node.walk_distance(n);
            unobstructed_destinations_visit(world, state, n, current_cost + edge_cost, neighmap);
        }
    }
}

// the graph should be considered a complete graph when moving: any point can reach any other
// point, but only if there's nothing in between
fn unobstructed_destinations(world: &Graph, state: &State, origin: Node) -> HashMap<Node, Cost> {
    let mut neighmap = HashMap::new();
    unobstructed_destinations_visit(world, state, origin, 0, &mut neighmap);
    neighmap.remove(&origin);
    neighmap
}

fn goal_heuristic(state: &State) -> Cost {
    state.pods.iter().map(|p| p.cost_heuristic()).sum::<Cost>()
}

fn amphipod_dijkstra(world: &Graph, origin: PodList) -> Cost {
    // estimated total path to goal with heuristic, remaining cost heuristic to goal, realized dist, pod state
    let mut heap: BinaryHeap::<(Reverse<i32>, Reverse<i32>, Reverse<i32>, State)> = BinaryHeap::new();
    let mut distances = HashMap::<State, Cost>::new();
    let origstate = State { pods: origin };
    distances.insert(origstate.clone(), 0);
    heap.push((Reverse(goal_heuristic(&origstate)), Reverse(goal_heuristic(&origstate)), Reverse(0), origstate));

    let debug = false;

    let mut parents = HashMap::<State, State>::new();

    while let Some((Reverse(_goal_heuristic_i), Reverse(cost_heuristic_i), Reverse(dist_i), state_i)) = heap.pop() {
        if cost_heuristic_i == 0 {
            if debug {
                println!("FOUND PATH!");
                dump_state(world, &state_i.pods);
                println!("dist of this ^ is {:?}", dist_i);
                let mut here = &state_i;
                while let Some(par) = parents.get(&here) {
                    println!();
                    dump_state(world, &par.pods);
                    println!("dist of this ^ is {:?} deltacost is {}", distances.get(&par).unwrap(), distances.get(&here).unwrap() - distances.get(&par).unwrap());
                    here = &par;
                }
            }
            return dist_i;
        }

        for (podi, pod) in state_i.pods.iter().enumerate() {
            // stay put if this is a destination spot
            // final_looking_spot is needed because a pod might need to jump out to help a
            // foreigner out first, like D in the example
            if !hallway_node(pod.node) && own_room(pod.node, pod.name) && final_looking_spot(world, &state_i, *pod) {
                continue;
            }

            // don't move into a node that has something already
            // (building the entire map every time is not wise though, but the world is small)
            let pod_neighs = unobstructed_destinations(world, &state_i, pod.node);

            for (&next_node, &walk_distance) in pod_neighs.iter() {
                assert!(!state_i.occupied(next_node));
                let podj = PodState::new(pod.name, next_node);

                // stubborn rule #2: from hallway to only own room.
                // note that the source doesn't have to be hallway; a path from a room always
                // visits a hallway anyway
                if !hallway_node(next_node) {
                    if own_room(next_node, pod.name)
                            && safe_looking_room(&state_i, next_node, *pod)
                            && final_looking_spot(world, &state_i, podj) {
                        // okay to enter
                    } else {
                        // not ours, or someone bothering, or empty gap in between
                        continue;
                    }
                }

                // stubborn rule #3: don't move from hallway to hallway
                if hallway_node(pod.node) && hallway_node(next_node) {
                    continue;
                }

                let dist_ij = walk_distance * step_energy(pod.name);
                let dist_j = dist_i + dist_ij;
                let mut state_j = state_i.clone();
                state_j.pods[podi] = podj;

                if dist_j < *distances.get(&state_j).unwrap_or(&MAX_COST) {
                    let cost_heuristic_j = goal_heuristic(&state_j);
                    let goal_heuristic_j = dist_j + cost_heuristic_j;
                    heap.push((
                            Reverse(goal_heuristic_j),
                            Reverse(cost_heuristic_j),
                            Reverse(dist_j),
                            state_j.clone()));
                    if debug {
                        parents.insert(state_j.clone(), state_i.clone());
                    }
                    distances.insert(state_j, dist_j);
                }
            }
        }
    }

    panic!("no route to goal");
}

fn collect_map(map: &[String], world: &Graph) -> PodList {
    map.iter().enumerate()
        .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, ch)| ((x as i8, y as i8), ch)))
        .filter(|(_pos, ch)| *ch >= 'A' && *ch <= 'D')
        .map(|(pos, ch)| {
            PodState::new(ch, world.node_at(pos))
        })
        .collect()
}

fn organize_amphipods(raw_map: &[String]) -> i32 {
    let part2 = raw_map.len() > 5;
    let world = empty_map(part2);
    let pods = collect_map(raw_map, &world);
    amphipod_dijkstra(&world, pods)
}

fn main() {
    let mut raw_map: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    println!("{:?}", organize_amphipods(&raw_map));
    raw_map.insert(3, String::from("  #D#C#B#A#"));
    raw_map.insert(4, String::from("  #D#B#A#C#"));
    println!("{:?}", organize_amphipods(&raw_map));
}
