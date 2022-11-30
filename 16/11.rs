use std::collections::vec_deque::VecDeque;

// type encodes also the name, which is an arbitrary character i chose here manually for just
// debugging purposes
#[derive(Debug, PartialEq)]
enum Object {
    Generator(char),
    Microchip(char)
}
use Object::*;

type ObjectState = [Vec<Object>; 4];

// invalid if: for any chip { no matching gen exists and another gen exists in this floor }
fn valid(state: &ObjectState) -> bool {
    for floor in state {
        for obj in floor {
            if let &Microchip(name) = obj {
                if !floor.contains(&Generator(name)) {
                    for obj in floor {
                        if let &Generator(_) = obj {
                            return false;
                        }
                    }
                }
            }
        }
    }

    true
}

// for five microchips, five generators, and one elevator, two bits per each to store just the
// floor number (0..3) 11*2=22 in total.
//
// floor bits like this: chip N-1 .. chip 0 .. gen N-1 .. gen 0 .. elevator

type Encoded = u32;
const FLOOR_BITS: Encoded = 2;
const FLOOR_MASK: Encoded = 3;
// XXX XXX HOX SWAP THIS 2 and 5 for sample and input (lol 7)
const N_ELEMENTS: usize = 7;

// 2-bit field helpers; the whole puzzle is probably a very neat bit twiddling trick, but it's
// so much easier to read (and initially write) this way. Hopefully this will get optimized.

fn read_field(state: Encoded, idx: usize) -> Encoded {
    let idx = idx as Encoded;
    (state >> (idx * FLOOR_BITS)) & FLOOR_MASK
}

fn _read_elevator(state: Encoded) -> Encoded {
    read_field(state, 0)
}

fn read_microchip(state: Encoded, idx: usize) -> Encoded {
    read_field(state, 1 + N_ELEMENTS + idx)
}

fn read_generator(state: Encoded, idx: usize) -> Encoded {
    read_field(state, 1 + idx)
}

fn write_field(state: Encoded, idx: usize, new: Encoded) -> Encoded {
    let idx = idx as Encoded;
    let field = FLOOR_MASK << (idx * FLOOR_BITS);
    assert!(new <= FLOOR_MASK);
    let replace = new << (idx * FLOOR_BITS);

    (state & !field) | replace
}

fn write_elevator(state: Encoded, new: Encoded) -> Encoded {
    write_field(state, 0, new)
}

fn _write_microchip(state: Encoded, idx: usize, new: Encoded) -> Encoded {
    write_field(state, 1 + N_ELEMENTS + idx, new)
}

fn _write_generator(state: Encoded, idx: usize, new: Encoded) -> Encoded {
    write_field(state, 1 + idx, new)
}

fn _name_index_sample(ch: char) -> usize {
    match ch {
        'H' => 0,
        'L' => 1,
        _ => unreachable!()
    }
}

fn _obj_index_sample(o: &Object) -> usize {
    match *o {
        Generator(ch) => name_index(ch),
        Microchip(ch) => N_ELEMENTS + name_index(ch)
    }
}

fn _name_at_sample(pos: usize) -> char {
    match pos {
        0 => 'H',
        1 => 'L',
        _ => unreachable!()
    }
}

fn _obj_at_sample(pos: usize, nobjs: usize) -> Object {
    if pos < nobjs / 2 {
        Generator(name_at(pos))
    } else {
        Microchip(name_at(pos - nobjs / 2))
    }
}

fn name_index_input(ch: char) -> usize {
    match ch {
        'T' => 0,
        'P' => 1,
        'S' => 2,
        'O' => 3,
        'R' => 4,
        'E' => 5,
        'D' => 6,
        _ => unreachable!()
    }
}

fn obj_index_input(o: &Object) -> usize {
    match *o {
        Generator(ch) => name_index(ch),
        Microchip(ch) => N_ELEMENTS + name_index(ch)
    }
}

fn name_at_input(pos: usize) -> char {
    match pos {
        0 => 'T',
        1 => 'P',
        2 => 'S',
        3 => 'O',
        4 => 'R',
        5 => 'E',
        6 => 'D',
        _ => unreachable!()
    }
}

fn obj_at_input(pos: usize, nobjs: usize) -> Object {
    if pos < nobjs / 2 {
        Generator(name_at(pos))
    } else {
        Microchip(name_at(pos - nobjs / 2))
    }
}

// XXX swap for sample
fn name_index(ch: char) -> usize {
    name_index_input(ch)
}

fn obj_index(o: &Object) -> usize {
    obj_index_input(o)
}

fn name_at(pos: usize) -> char {
    name_at_input(pos)
}

fn obj_at(pos: usize, nobjs: usize) -> Object {
    obj_at_input(pos, nobjs)
}

fn encode(objs: &ObjectState, elevator: usize) -> Encoded {
    let mut code = elevator as Encoded; // lowest bits
    for (i, floor) in objs.iter().enumerate() {
        for obj in floor {
            // shift floor index into position, skipping elevator
            code = write_field(code, 1 + obj_index(obj), i as Encoded);;
            //code |= (i as Encoded) << (obj_index(obj) + 1) as Encoded * FLOOR_BITS;
        }
    }

    code
}

fn decode(state: Encoded, nobjs: usize) -> ObjectState {
    let mut s = [vec![], vec![], vec![], vec![]];
    for i in 0..nobjs {
        s[read_field(state, 1 + i) as usize].push(obj_at(i, nobjs));
    }

    s
}

#[derive(Clone, Debug)]
struct Node {
    state: Encoded,
    distance: u32,
    //parent: usize,
    //idx: usize
}
// for Vec::contains - compare just states, distance doesn't matter
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.state == other.state
    }
}

// invalid if: for any chip { no matching gen exists and another gen exists in this floor }
fn valid_encoded(state: Encoded) -> bool {
    //println!("valid? {:064b}", state);
    for i in 0..N_ELEMENTS {
        let floor = read_microchip(state, i);
        if read_generator(state, i) != floor {
            for j in 0..N_ELEMENTS {
                if j != i && read_generator(state, j) == floor {
                    return false;
                }
            }
        }
    }

    true
}

fn print(state: Encoded, nobjects: usize) {
    if false {
        let debug = decode(state, nobjects);
        println!("  {:?}\n  {:?}\n  {:?}\n  {:?}", debug[0], debug[1], debug[2], debug[3]);
    }
}

// validate, see if not exist yet, add to nodes, enqueue
// // validate, see if not exist yet, add to nodes, enqueue
fn try_enqueue(nodes: &mut Vec<Node>, visited: &mut Vec<u64>, queue: &mut VecDeque<Node>, state: Encoded, parent: &Node) {
    let next = Node { state: state, distance: parent.distance + 1/*, parent: parent.idx, idx: nodes.len()*/ };
    if /* !nodes.contains(&next)*/ (visited[(state as usize) / 64] & (1 << (state % 64)) == 0) && valid_encoded(state) {
        //println!("push {:064b}", next.state);
        print(state, 4);
        if false {
            nodes.push(next.clone());
        }
        queue.push_back(next);
        visited[(state as usize) / 64] |= 1 << (state % 64);
    }
}

// breadth-first search of states via edges to valid states
// return shortest path to end
fn search(start: &ObjectState, end: &ObjectState) -> usize {
    let nobjects = start.iter().map(|row| row.len()).sum::<usize>();
    let nelements = nobjects / 2;
    assert!(nelements == N_ELEMENTS); // lazy hack
    let is_gen = |id| id < nelements; // gen indices come first in the bits
    let root = Node { state: encode(start, 0), distance: 0/*, parent: 0, idx: 0*/ };
    let end = Node { state: encode(end, 0), distance: 0/*, parent: 0, idx: 0*/ };
    let mut nodes = Vec::new();
    nodes.push(root.clone());
    let mut visited = Vec::new();
    visited.resize((1 << 30) / 64, 0u64);
    visited[(root.state as usize) / 64] |= 1 << (root.state % 64);
    let mut queue = VecDeque::new();
    queue.push_back(root);

    // floor bits like this: chip N-1 .. chip 0 .. gen N-1 .. gen 0 .. elevator (yes this)
    let mut ii = 0;
    while let Some(current) = queue.pop_front() {
        ii += 1;
        if ii % 100000 == 0 { println!("{} {}", ii, queue.len()); }
        let cur = current.state;
        if cur & !3 == end.state & !3 { println!("fffound {:?}\n", current); break; }
        //println!("current {:064b}", cur);
        print(cur, nobjects);
        // State changes: elevator up or down, takes one or two objects with it.
        // If two, then they're a) both chips, b) both gens, or c) matching chip and gen.
        // State after they're moved must be valid.
        let elevator = cur & 3;
        // the write_field indices here require that N_ELEMENTS matches the input
        for i in 0..nobjects {
            let objfloor = read_field(cur, 1 + i);
            if objfloor != elevator { continue; }
            //println!("obj {} at {}", i, elevator);

            if elevator > 0 {
                try_enqueue(&mut nodes, &mut visited, &mut queue,
                            write_field(write_elevator(cur, elevator - 1),
                            1 + i, elevator - 1), &current);
            }
            if elevator < 3 {
                try_enqueue(&mut nodes, &mut visited, &mut queue,
                            write_field(write_elevator(cur, elevator + 1),
                            1 + i, elevator + 1), &current);
            }
            // move another with this, maybe
            for j in i+1..nobjects {
                let obj2floor = read_field(cur, 1 + j);
                if obj2floor != elevator { continue; }

                let same_type = is_gen(i) == is_gen(j);
                // i is gen, j is matching chip
                let matching_idx = i < nelements && i + nelements == j;
                if same_type || matching_idx {
                    //println!("  with obj {}", j);
                    if elevator > 0 {
                        try_enqueue(&mut nodes, &mut visited, &mut queue,
                                write_field(
                                    write_field(
                                        write_elevator(cur, elevator - 1),
                                        1 + i, elevator - 1),
                                    1 + j, elevator - 1),
                                &current);
                    }
                    if elevator < 3 {
                        try_enqueue(&mut nodes, &mut visited, &mut queue,
                                write_field(
                                    write_field(
                                        write_elevator(cur, elevator + 1),
                                        1 + i, elevator + 1),
                                    1 + j, elevator + 1),
                                &current);
                    }
                }
            }
        }
    }

    /*
    //for n in &nodes { println!("{:?}", n); }
    let goal = nodes.iter().find(|&x| x.state & !3 == end.state & !3);
    println!("{:?} {:?}", goal, end);
    if let Some(mut node) = goal {
        let mut step = 0;
        loop {
            println!("step {}", step);
            print(node.state, 4);
            if node.idx == 0 {
                break;
            }
            node = &nodes[node.parent];
            step += 1;
        }
    }

    goal.unwrap().distance
    */
    0
}

// solve shortest path in this directed graph:
// node = where each object is; (gen/chip, floor) pairs for each + elevator.
// edge = what moves where with the elevator - nothing, one object, or two, so objects in an
// elevator does not need to be a state (=node) separately.
// edge exists if destination is valid.
// valid if: for each chip { matching gen exists or no other gens exist in this floor }.
// invalid if: for any chip { no matching gen exists and another gen exists in this floor }.
// end state = all objects in floor 4.
fn mainsample() {
    //let src = readfile(&std::env::args().nth(1).unwrap());
    let state: ObjectState = [
        vec![Microchip('H'), Microchip('L')],
        vec![Generator('H')],
        vec![Generator('L')],
        vec![]
    ];
    let end: ObjectState = [
        vec![],
        vec![],
        vec![],
        vec![Microchip('H'), Microchip('L'), Generator('H'), Generator('L')],
    ];
    let all0: ObjectState = [
        vec![Microchip('H'), Microchip('L'), Generator('H'), Generator('L')],
        vec![],
        vec![],
        vec![],
    ];
    let split: ObjectState = [
        vec![Generator('H'), Microchip('H')],
        vec![],
        vec![],
        vec![Generator('L'), Microchip('L')],
    ];
    let split2: ObjectState = [
        vec![Generator('H'), Generator('L')],
        vec![],
        vec![],
        vec![Microchip('H'), Microchip('L')],
    ];
    let inva: ObjectState = [
        vec![],
        vec![],
        vec![Generator('H')],
        vec![Microchip('H'), Microchip('L'), Generator('L')],
    ];
    println!("{} {} {}", valid(&state), valid(&end), valid(&inva));
    println!("{:064b} {:064b} {:064b} {:064b} {:064b} {:064b}", encode(&all0, 0), encode(&split, 0), encode(&split2, 0), encode(&state, 0), encode(&end, 0), encode(&inva, 0));
    println!("{}", search(&state, &end));
    let step: ObjectState = [
        vec![Microchip('L')],
        vec![Microchip('H'), Generator('H')],
        vec![Generator('L')],
        vec![],
    ];
    println!("{} {:064b} {}", encode(&step, 0), encode(&step, 0), valid_encoded(encode(&step, 0)));

    let step: ObjectState = [
        vec![Microchip('L')],
        vec![],
        vec![Microchip('H'), Generator('H'), Generator('L')],
        vec![],
    ];
    println!("{} {:064b} {}", encode(&step, 0), encode(&step, 0), valid_encoded(encode(&step, 0)));

    let step: ObjectState = [
        vec![Microchip('L')],
        vec![Microchip('H')],
        vec![Generator('H'), Generator('L')],
        vec![],
    ];
    println!("{} {:064b} {}", encode(&step, 0), encode(&step, 0), valid_encoded(encode(&step, 0)));

    let step: ObjectState = [
        vec![Microchip('H'), Microchip('L')],
        vec![],
        vec![Generator('H'), Generator('L')],
        vec![],
    ];
    println!("{} {:064b} {}", encode(&step, 0), encode(&step, 0), valid_encoded(encode(&step, 0)));

    let step: ObjectState = [
        vec![],
        vec![Microchip('H'), Microchip('L')],
        vec![Generator('H'), Generator('L')],
        vec![],
    ];
    println!("{} {:064b} {}", encode(&step, 0), encode(&step, 0), valid_encoded(encode(&step, 0)));

    let step: ObjectState = [
        vec![],
        vec![],
        vec![Microchip('H'), Microchip('L'), Generator('H'), Generator('L')],
        vec![],
    ];
    println!("{} {:064b} {}", encode(&step, 0), encode(&step, 0), valid_encoded(encode(&step, 0)));

    let step: ObjectState = [
        vec![],
        vec![],
        vec![Generator('H'), Generator('L')],
        vec![Microchip('H'), Microchip('L')],
    ];
    println!("{} {:064b} {}", encode(&step, 0), encode(&step, 0), valid_encoded(encode(&step, 0)));

    let step: ObjectState = [
        vec![],
        vec![],
        vec![Microchip('H'), Generator('H'), Generator('L')],
        vec![Microchip('L')],
    ];
    println!("{} {:064b} {}", encode(&step, 0), encode(&step, 0), valid_encoded(encode(&step, 0)));

    let step: ObjectState = [
        vec![],
        vec![],
        vec![Microchip('H')],
        vec![Microchip('L'), Generator('H'), Generator('L')],
    ];
    println!("{} {:064b} {}", encode(&step, 0), encode(&step, 0), valid_encoded(encode(&step, 0)));

    let step: ObjectState = [
        vec![],
        vec![],
        vec![Microchip('L'), Microchip('H')],
        vec![Generator('H'), Generator('L')],
    ];
    println!("{} {:064b} {}", encode(&step, 0), encode(&step, 0), valid_encoded(encode(&step, 0)));

    let step: ObjectState = [
        vec![],
        vec![],
        vec![],
        vec![Microchip('L'), Microchip('H'), Generator('H'), Generator('L')],
    ];
    println!("{} {:064b} {}", encode(&step, 0), encode(&step, 0), valid_encoded(encode(&step, 0)));
}

fn main_partone() {
    let state: ObjectState = [
        vec![Generator('T'), Microchip('T'), Generator('P'), Generator('S')],
        vec![Microchip('P'), Microchip('S')],
        vec![Generator('O'), Microchip('O'), Generator('R'), Microchip('R')],
        vec![]
    ];
    let end: ObjectState = [
        vec![],
        vec![],
        vec![],
        vec![Generator('T'), Microchip('T'), Generator('P'), Generator('S'),
        Microchip('P'), Microchip('S'),
        Generator('O'), Microchip('O'), Generator('R'), Microchip('R')]
    ];
    let inva: ObjectState = [
        vec![],
        vec![],
        vec![Generator('T'), Generator('P')],
        vec![Microchip('T'), Generator('S'),
        Microchip('P'), Microchip('S'),
        Generator('O'), Microchip('O'), Generator('R'), Microchip('R')]
    ];
    println!("{} {} {}", valid(&state), valid(&end), valid(&inva));
    println!("{:064b} {:064b} {:064b}", encode(&state, 0), encode(&end, 0), encode(&inva, 0));
    println!("{}", search(&state, &end));
}

// 2b some extra items not listed on the record
fn main() {
    if false {
        main_partone();
        mainsample();
    }
    let state: ObjectState = [
        vec![Generator('T'), Microchip('T'), Generator('P'), Generator('S'), Generator('E'), Microchip('E'), Generator('D'), Microchip('D')],
        vec![Microchip('P'), Microchip('S')],
        vec![Generator('O'), Microchip('O'), Generator('R'), Microchip('R')],
        vec![]
    ];
    let end: ObjectState = [
        vec![],
        vec![],
        vec![],
        vec![Generator('T'), Microchip('T'), Generator('P'), Generator('S'), Generator('E'), Microchip('E'), Generator('D'), Microchip('D'),
        Microchip('P'), Microchip('S'),
        Generator('O'), Microchip('O'), Generator('R'), Microchip('R')]
    ];
    let inva: ObjectState = [
        vec![],
        vec![],
        vec![Generator('T'), Generator('P')],
        vec![Microchip('T'), Generator('S'),
        Microchip('P'), Microchip('S'),
        Generator('O'), Microchip('O'), Generator('R'), Microchip('R')]
    ];
    println!("{} {} {}", valid(&state), valid(&end), valid(&inva));
    println!("{:064b} {:064b} {:064b}", encode(&state, 0), encode(&end, 0), encode(&inva, 0));
    println!("{}", search(&state, &end));
}
