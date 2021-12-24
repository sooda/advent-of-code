use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap, BinaryHeap};

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
fn empty_map() -> Graph {
    // 13 by 4
    let map = "\
        #############\
        #ef.g.h.i.jk#\
        ###l#m#n#o###\
        ###p#q#r#s###".as_bytes();
    let nodes = (0..4)
        .flat_map(|y| (0..13).map(move |x| (x, y, map[y * 13 + x])))
        .filter(|&(_x, _y, id)| id >= b'a' && id <= b'z')
        .map(|(x, y, id)| Node::new(x as i8, y as i8, id))
        .collect::<Vec::<_>>();
    let n_by_id = |id| *nodes.iter().find(|n| n.id == id).unwrap();
    let route = [
        "ef", "fg", "gh", "hi", "ij", "jk",
        "fl", "gl", "lp",
        "gm", "hm", "mq",
        "hn", "in", "nr",
        "io", "jo", "os"
    ];
    let edges: Vec<_> = route.iter()
        .map(|pair| pair.as_bytes())
        .map(|bs| (n_by_id(bs[0] as char), n_by_id(bs[1] as char)))
        .collect();
    Graph::new(nodes, edges)
}

fn empty_map2() -> Graph {
    // 13 by 6
    let map = "\
        #############\
        #ef.g.h.i.jk#\
        ###l#m#n#o###\
        ###p#q#r#s###\
        ###t#u#v#w###\
        ###x#y#z#{###".as_bytes();
    // note: "{" comes after "z" in ascii
    let nodes = (0..6)
        .flat_map(|y| (0..13).map(move |x| (x, y, map[y * 13 + x])))
        .filter(|&(_x, _y, id)| (id >= b'a' && id <= b'{'))
        .map(|(x, y, id)| Node::new(x as i8, y as i8, id))
        .collect::<Vec::<_>>();
    let n_by_id = |id| { *nodes.iter().find(|n| n.id == id).unwrap() };
    let route = [
        "ef", "fg", "gh", "hi", "ij", "jk",
        "fl", "gl", "lp",
        "gm", "hm", "mq",
        "hn", "in", "nr",
        "io", "jo", "os",
        "pt", "qu", "rv", "sw",
        "tx", "uy", "vz", "w{"
    ];
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
                match self.node.y {
                    // in hallway
                    1 => (xdiff + 1) * step_energy(self.name),
                    // up once, down at least once
                    2 => (1 + xdiff + 1) * step_energy(self.name),
                    // up twice, down at least once
                    3 => (2 + xdiff + 1) * step_energy(self.name),
                    // up thrice, down at least once
                    4 => (3 + xdiff + 1) * step_energy(self.name),
                    // up 4, down at least once
                    5 => (4 + xdiff + 1) * step_energy(self.name),
                    _ => panic!("bad pos")
                }
            }
        } else {
            xdiff * step_energy(self.name)
        }
    }
}

/*
impl Ord for PodState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.cost_heuristic(), self.name, self.node).cmp(&(other.cost_heuristic(), other.name, other.node))
    }
}

impl PartialOrd for PodState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
*/

type PodList = Vec::<PodState>;

// kind of stupid to have ord for this, the ordering of identical costs does not matter
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    pods: PodList,
}

impl State {
    fn pod_by_node(&self, n: Node) -> Option<PodState> {
        self.pods.iter().find(|ps| ps.node == n).map(|&x| x)
    }
}

fn occupied(state: &State, node: Node) -> bool {
    state.pods.iter().any(|p| p.node == node)
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

// comfort zone with deep rooms, like room_pair_owner in part 1: no foreigner there
fn safe_looking_room(state: &State, spot: Node, player: PodState) -> bool {
    let mut room_folk = state.pods.iter().filter(|ps| !hallway_node(ps.node) && ps.node.x == spot.x);
    room_folk.all(|ps| ps.name == player.name)
}

// all below spots must be filled by own people.
// not empty.
// not foreigners.
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
        if !occupied(state, n) {
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

// perhaps sort the pods first but even then there is ambiguity for the win state, either A_a and
// A_b or A_b and A_a sit in one column
fn amphipod_dijkstra(world: &Graph, origin: PodList, destination: PodList) -> Cost {
    let mut heap: BinaryHeap::<(i32, i32, i32, State)> = BinaryHeap::new(); // -total path to goal with heuristic, -cost heuristic, -dist, coords
    let mut distances = HashMap::<State, Cost>::new();
    let origstate = State { pods: origin };
    heap.push((-goal_heuristic(&origstate), -goal_heuristic(&origstate), 0, origstate));

    let debug = false;
    let debug2 = debug && false;

    let mut minheur = MAX_COST;
    let mut min_zero = MAX_COST;

    let mut parents = HashMap::<State, State>::new();

    while let Some(current) = heap.pop() {
        let (goal_heuristic_i, cost_heuristic_i, dist_i, state_i) = current;
        let goal_heuristic_i = -goal_heuristic_i;
        let cost_heuristic_i = -cost_heuristic_i;
        let dist_i = -dist_i;

        if cost_heuristic_i == 0 {
            min_zero = min_zero.min(dist_i);
        }

        if cost_heuristic_i < minheur || (cost_heuristic_i == 0 && minheur == 0)  {
            println!("UPDATED closest cost: {} heuristic to win: {} total path to win with heuristics: {}", dist_i, cost_heuristic_i, goal_heuristic_i);
            dump_state(world, &state_i.pods);
            println!();
            minheur = cost_heuristic_i;
            if cost_heuristic_i == 0 {
                println!("FOUND PATH!");
                let mut here = state_i.clone();
                println!("dist of this ^ is {:?} and smallest is {}", distances.get(&state_i), min_zero);
                while let Some(par) = parents.get(&here) {
                    println!();
                    dump_state(world, &par.pods);
                    println!("dist of this ^ is {:?} deltacost is {}", distances.get(&par), distances.get(&here).unwrap() - distances.get(&par).unwrap_or(&0));
                    here = par.clone();
                }
            }
        }

        if debug {
            println!("cost: {} heuristic: {} total path to win with heuristics: {}", dist_i, cost_heuristic_i, goal_heuristic_i);
            dump_state(world, &state_i.pods);
            println!();
        }
        for (podi, pod) in state_i.pods.iter().enumerate() {
            // stay put if this is a destination spot
            // final_looking_spot is needed because a pod might need to jump out to help a
            // foreigner out first, like D in the example
            if !hallway_node(pod.node) && own_room(pod.node, pod.name) && final_looking_spot(world, &state_i, *pod) {
                //println!("kaikki ok {:?}", pod);
                continue;
            }
            // don't move into a node that has something already
            let pod_neighs = unobstructed_destinations(world, &state_i, pod.node);
            if debug2 {
                println!("pod {:?} neighs are {:#?}", pod, pod_neighs);
            }
            for (&next_node, &walk_distance) in pod_neighs.iter() {
                assert!(!occupied(&state_i, next_node));
                let podj = PodState::new(pod.name, next_node);

                // stubborn rule #2: from hallway to only own room
                if hallway_node(pod.node) && !hallway_node(next_node) {
                    if own_room(next_node, pod.name) && safe_looking_room(&state_i, next_node, *pod) && final_looking_spot(world, &state_i, podj) {
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

                if !hallway_node(pod.node) && !hallway_node(next_node) {
                    // room to room is prone to break this brittle logic
                    continue;
                }

                let dist_ij = walk_distance * step_energy(pod.name);
                let dist_j = dist_i + dist_ij;
                let mut state_j = state_i.clone();
                state_j.pods[podi] = podj;
                if debug2 {
                    println!("move {:?} to {:?} cost is {}, that would look like:", pod, next_node, dist_ij);
                    dump_state(world, &state_j.pods);
                }

                if dist_j < *distances.get(&state_j).unwrap_or(&MAX_COST) {
                    let cost_heuristic_j = goal_heuristic(&state_j);
                    let goal_heuristic_j = dist_j + cost_heuristic_j;
                    heap.push((-goal_heuristic_j, -cost_heuristic_j, -dist_j, state_j.clone()));
                    if state_j.pods == destination {
                        return dist_j;
                    }
                    if true {
                        parents.insert(state_j.clone(), state_i.clone());
                    }
                    distances.insert(state_j, dist_j);
                }
            }
        }
        if distances.len() % 10000 == 0 {
            println!("dists {} heaplen {} min heuristic {} this cost {} zeromin {}", distances.len(), heap.len(), minheur, dist_i, min_zero);
        }
    }

    println!("this many states found {}", distances.len());
    *distances.get(&State { pods: destination }).expect("noo")
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

// FIXME: the ordering of the elements does not work when comparing states like this because
// they're in a vec. Should turn the state into which nodes are occupied.
fn optimal_arrangement(world: &Graph) -> PodList {
    let optimal_raw_map = vec![
        String::from("#############"),
        String::from("#...........#"),
        String::from("###A#B#C#D###"),
        String::from("  #A#B#C#D#"),
        String::from("  #########"),
    ];
    collect_map(&optimal_raw_map, world)
}

fn optimal_arrangement2(world: &Graph) -> PodList {
    let optimal_raw_map = vec![
        String::from("#############"),
        String::from("#...........#"),
        String::from("###A#B#C#D###"),
        String::from("  #A#B#C#D#"),
        String::from("  #A#B#C#D#"),
        String::from("  #A#B#C#D#"),
        String::from("  #########"),
    ];
    collect_map(&optimal_raw_map, world)
}

fn organize_amphipods(raw_map: &[String]) -> i32 {
    let world = empty_map();
    let pods = collect_map(raw_map, &world);
    let destination = optimal_arrangement(&world);
    amphipod_dijkstra(&world, pods, destination)
}

fn organize_amphipods2(raw_map: &[String]) -> i32 {
    let world = empty_map2();
    let pods = collect_map(raw_map, &world);
    let destination = optimal_arrangement2(&world);
    amphipod_dijkstra(&world, pods, destination)
}

fn main() {
    let raw_map: Vec<String> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let part1 = raw_map.len() == 5;
    if part1 {
        println!("{:?}", organize_amphipods(&raw_map));
    } else {
        println!("{:?}", organize_amphipods2(&raw_map));
    }
}
