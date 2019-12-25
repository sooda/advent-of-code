use std::io::{self, BufRead};
use std::collections::{HashMap, VecDeque};
use std::collections::hash_map::Entry;


fn step<'a, I: Iterator<Item = i64>>(program: &'a mut [i64], ip: usize, base: i64, input: &mut I) -> Option<(usize, i64, Option<i64>)> {
    let opcode = program[ip] % 100;
    if opcode == 99 {
        return None;
    }
    let mode0 = program[ip] / 100 % 10;
    let mode1 = program[ip] / 1000 % 10;
    let mode2 = program[ip] / 10000 % 10;
    assert!(mode0 <= 2);
    assert!(mode1 <= 2);
    assert!(mode2 <= 2);
    let immflags = (mode0 == 1, mode1 == 1, mode2 == 1);
    let relflags = (mode0 == 2, mode1 == 2, mode2 == 2);

    let rel0 = if relflags.0 { base } else { 0 };
    let rel1 = if relflags.1 { base } else { 0 };
    let rel2 = if relflags.2 { base } else { 0 };
    let imm0 = || program[ip + 1];
    let imm1 = || program[ip + 2];
    let val0 = || if immflags.0 { imm0() } else { program[(imm0() + rel0) as usize ] };
    let val1 = || if immflags.1 { imm1() } else { program[(imm1() + rel1) as usize ] };

    let mut0 = |program: &'a mut [i64]| {
        assert!(!immflags.0); &mut program[(program[ip + 1] + rel0) as usize] };
    let mut2 = |program: &'a mut [i64]| {
        assert!(!immflags.2); &mut program[(program[ip + 3] + rel2) as usize] };

    match opcode {
        1 => {
            *mut2(program) = val0() + val1();
            Some((ip + 4, base, None))
        },
        2 => {
            *mut2(program) = val0() * val1();
            Some((ip + 4, base, None))
        },
        3 => {
            *mut0(program) = input.next().unwrap();
            Some((ip + 2, base, None))
        }
        4 => {
            Some((ip + 2, base, Some(val0())))
        },
        5 => {
            if val0() != 0 {
                Some((val1() as usize, base, None))
            } else {
                Some((ip + 3, base, None))
            }
        },
        6 => {
            if val0() == 0 {
                Some((val1() as usize, base, None))
            } else {
                Some((ip + 3, base, None))
            }
        },
        7 => {
            *mut2(program) = if val0() < val1() { 1 } else { 0 };
            Some((ip + 4, base, None))
        },
        8 => {
            *mut2(program) = if val0() == val1() { 1 } else { 0 };
            Some((ip + 4, base, None))
        },
        9 => {
            Some((ip + 2, base + val0(), None))
        },
        _ => panic!("something went wrong at {}: {}", ip, program[ip])
    }
}

#[derive(Clone)]
struct Computer {
    program: Vec<i64>,
    ip: usize,
    base: i64,
}

fn execute_dungeon(computer: &mut Computer, inputs: &str) -> Option<char> {
    let mut input = inputs.bytes().map(|b| b as i64);
    //let mut input = inputs.bytes().map(|b| b as i64).chain([b'\n' as i64].into_iter().cloned());
    while let Some((newip, newbase, newout)) =
            step(&mut computer.program, computer.ip, computer.base, &mut input) {
        computer.ip = newip;
        computer.base = newbase;
        if let Some(out) = newout {
            if out <= 127 {
                return Some(out as u8 as char);
            } else {
                panic!();
            }
        }
    }
    None
}

/*
fn airlock_password(program: &[i64]) -> String {
    let mut prog = program.to_vec();
    prog.resize(prog.len() + prog.len(), 0);
    let mut computer = Computer {
        program: prog,
        ip: 0,
        base: 0
    };

    let steps = &[
        "", // sentinel
        "west",
        "north",
        "north",
        "west",
        "north",
    ];

    let mut n = 0;
    let mut current_row = vec![];
    while let Some(ch) = execute_dungeon(&mut computer, &steps[n]) {
        print!("{}", ch);
        if ch == '\n' {
            current_row.clear();
        } else {
            current_row.push(ch);
        }
        if current_row.iter().collect::<String>() == "Command?" {
            n += 1;
            println!(" [ Step {}: {} ]", n, steps[n]);
        }
    }
    "".to_string()
}
*/

const TRACE_ASCII: bool = true;

// TODO: iterator or something
fn read_line(computer: &mut Computer) -> String {
    read_line2(computer, "")
}
fn read_line2(computer: &mut Computer, input: &str) -> String {
    let mut chars = Vec::new();

    while let Some(ch) = execute_dungeon(computer, input) {
        if TRACE_ASCII {
            print!("{}", ch);
        }
        if ch == '\n' {
            break;
        } else {
            chars.push(ch);
        }
    }

    chars.iter().collect()
}

#[derive(Debug, PartialEq)]
struct Room {
    title: String,
    description: String,
    doors: [bool; 4],
    items: Vec<String>,
}

const DOOR_NORTH: usize = 0;
const DOOR_SOUTH: usize = 1;
const DOOR_EAST: usize = 2;
const DOOR_WEST: usize = 3;

/*
 * == Hull Breach ==
 * You got in through a hole in the floor here. To keep your ship from also freezing, the hole has
 * been sealed.
 *
 * Doors here lead:
 * - east
 * - south
 * - west
 *
 * Command?
 * == Navigation ==
 * Status: Stranded. Please supply measurements from fifty stars to recalibrate.
 *
 * Doors here lead:
 * - north
 * - east
 * - west
 *
 * Items here:
 * - giant electromagnet
 *
 * Command?
 */
fn read_room(computer: &mut Computer, inventory: &mut Option<&mut Vec<String>>) -> Room {
    assert_eq!(read_line(computer), "");
    assert_eq!(read_line(computer), "");
    let title = read_line(computer);
    let description = read_line(computer);
    assert_eq!(read_line(computer), "");
    let _doors_header = read_line(computer);

    let mut doors = [false; 4];
    loop {
        let door = read_line(computer);
        match door.as_str() {
            "- north" => doors[0] = true,
            "- south" => doors[1] = true,
            "- east" => doors[2] = true,
            "- west" => doors[3] = true,
            "" => break,
            _ => panic!("what door? '{}'", door),
        };
    }

    let mut items = Vec::new();
    let line = read_line(computer);
    if line == "Items here:" {
        loop {
            let line = read_line(computer);
            if line == "" {
                break;
            }
            // keep the "- " part
            items.push(line);
        }
        assert_eq!(read_line(computer), "Command?");
        if let Some(deep_pockets) = inventory {
            for grab in &items {
                let grab = &grab[2..];
                if grab == "photons" {
                    // It is suddenly completely dark! You are eaten by a Grue!
                    continue;
                }
                if grab == "giant electromagnet" {
                    // The giant electromagnet is stuck to you.  You can't move!!
                    continue;
                }
                if grab == "escape pod" {
                    // You're launched into space! Bye!
                    continue;
                }
                if grab == "molten lava" {
                    // The molten lava is way too hot! You melt!
                    continue;
                }
                if grab == "infinite loop" {
                    // You take the infinite loop.
                    // You take the infinite loop.
                    // You take the infinite loop.
                    continue;
                }
                deep_pockets.push(grab.to_string());
                println!("{}", ("take ".to_string() + grab + "\n"));
                let reply = read_line2(computer, &("take ".to_string() + grab + "\n"));
                assert_eq!(reply, "");
                assert_eq!(read_line(computer), "You take the ".to_string() + grab + ".");
                assert_eq!(read_line(computer), "");
                assert_eq!(read_line(computer), "Command?");
            }
        }
    } else {
        match line.as_str() {
            "Command?" => (),
            // TODO: manage these somehow?
            r#"A loud, robotic voice says "Alert! Droids on this ship are heavier than the detected value!" and you are ejected back to the checkpoint."# => (),
            r#"A loud, robotic voice says "Alert! Droids on this ship are lighter than the detected value!" and you are ejected back to the checkpoint."# => (),
            r#"A loud, robotic voice says "Analysis complete! You may proceed." and you enter the cockpit."# => (),
            _ => panic!(),
        }
    }

    Room {
        title, description, doors, items
    }
}

struct Map {
    rooms: HashMap<String, Room>, // keyed by title
    edges: HashMap<String, [Option<String>; 4]>,
}

fn crawl_dungeon(computer: &mut Computer, map: &mut Map, inventory: &mut Option<&mut Vec<String>>) -> String {
    let room = read_room(computer, inventory);
    let this_title = room.title.clone();
    let doors = match map.rooms.entry(this_title.clone()) {
        Entry::Vacant(e) => e.insert(room),
        Entry::Occupied(_) => return this_title,
    }.doors.clone();

    let directions = &[
        ("north\n", "south\n"),
        ("south\n", "north\n"),
        ("east\n", "west\n"),
        ("west\n", "east\n"),
    ];

    let mut this_edges = [ None, None, None, None ];

    for ((door_exists, &(walk, back)), edge) in doors.iter().zip(directions.iter()).zip(this_edges.iter_mut()) {
        if !door_exists {
            continue;
        }
        if TRACE_ASCII {
            println!("{}", walk);
        }
        let line = read_line2(computer, walk);
        assert_eq!(line, "");

        let neigh_title = crawl_dungeon(computer, map, inventory);
        *edge = Some(neigh_title);

        if TRACE_ASCII {
            println!("{}", back);
        }
        let line = read_line2(computer, back);
        assert_eq!(line, "");

        let back_here = read_room(computer, inventory);
        assert_eq!(back_here.title, this_title);
    }

    match map.edges.entry(this_title.clone()) {
        Entry::Vacant(e) => e.insert(this_edges),
        Entry::Occupied(_) => panic!("logic error"),
    };

    return this_title;
}

fn raw_bfs(map: &Map, source_room: &str, dest_room: &str) -> (HashMap<String, usize>, HashMap<String, (String, usize)>) {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    let mut parents = HashMap::new();

    queue.push_back((source_room.to_string(), 0));
    distances.insert(source_room.to_string(), 0);

    while let Some(current) = queue.pop_front() {
        let (roomlabel, dist) = current;
        if roomlabel == dest_room {
            // found it
            break;
        }
        let neighs = &map.edges[&roomlabel];
        for (i, neighlabel) in neighs.iter().enumerate().filter_map(|(i, n)| n.as_ref().map(|n| (i, n))) {
            let unknown = !distances.contains_key(neighlabel);
            if unknown {
                queue.push_back((neighlabel.to_string(), dist + 1));
                distances.insert(neighlabel.to_string(), dist);
                parents.insert(neighlabel.to_string(), (roomlabel.to_string(), i));
            }
        }
    }

    (distances, parents)
}

fn find_route(map: &Map, source_room: &str, dest_room: &str) -> Vec<usize> {
    let (dists, parents) = raw_bfs(map, source_room, dest_room);
    let mut current_label = dest_room;
    let mut route = Vec::new();
    while let Some((label, direction)) = parents.get(current_label) {
        current_label = label;
        route.insert(0, *direction);
    }
    route
}

fn enter_checkpoint(computer: &mut Computer, map: &mut Map, inventory: &mut Option<&mut Vec<String>>) {
    // FIXME: strip "=="s off
    let route = find_route(map, "== Hull Breach ==", "== Security Checkpoint ==");
    let dirnames = [ "north\n", "south\n", "east\n", "west\n" ];
    for turn in route {
        if TRACE_ASCII {
            println!("{}", turn);
        }
        let line = read_line2(computer, dirnames[turn]);
        assert_eq!(line, "");
        read_room(computer, inventory);
    }
}

fn nice_key(k: &str) -> String {
    k.chars().filter(|&ch| ch != '=' && ch != ' ' && ch != '-').collect()
}

fn dump_graphviz(map: &Map) {
    let mut keys = map.rooms.keys().cloned().collect::<Vec<_>>();
    keys.sort();
    println!("digraph G {{");
    for title in &keys {
        let room = &map.rooms[title];
        print!("room_{} [label=\"{}\\l{}\\l", nice_key(&room.title), room.title, room.description);
        print!("Items:\\l");
        for i in &room.items {
            print!("{}\\l", i);
        }
        println!("\"]");
    }
    for (room_title, neighs) in &map.edges {
        let dirnames = [ "north", "south", "east", "west" ];
        for (n, label) in neighs.iter().zip(dirnames.iter()) {
            if let Some(n) = n {
                println!("room_{} -> room_{} [label=\"{}\"]", nice_key(room_title), nice_key(n), label);
            }
        }
    }
    println!(r"}}");
}
fn crawl_map(program: &[i64]) -> Map {
    let mut prog = program.to_vec();
    prog.resize(prog.len() + prog.len(), 0);
    let mut computer = Computer {
        program: prog,
        ip: 0,
        base: 0
    };

    assert_eq!(read_line(&mut computer), "");

    let mut map = Map { rooms: HashMap::new(), edges: HashMap::new() };
    crawl_dungeon(&mut computer, &mut map, &mut None);
    dump_graphviz(&map);
    println!("Stuff found:");
    for item in map.rooms.values().flat_map(|room| room.items.iter()) {
        println!("{}", item);
    }
    map
}

fn drop_stuff(computer: &mut Computer, drop: &[&str]) {
    for d in drop {
        let command = "drop ".to_string() + d + "\n";
        if TRACE_ASCII {
            println!("{}", command);
        }
        let line = read_line2(computer, &command);
        assert_eq!(line, "");

        let line = read_line(computer);
        assert_eq!(line, "You drop the ".to_string() + d + ".");

        let line = read_line(computer);
        assert_eq!(line, "");

        let line = read_line(computer);
        assert_eq!(line, "Command?");
    }
}

fn attempt_weight(computer: &Computer, mut keep: &[&str], drop: &[&str]) {
    let mut computer = computer.clone();
    let mut inventory = keep.iter().map(|s| s.to_string()).collect();
    println!("drop this: {:?}", drop);
    println!("try this: {:?}", inventory);

    drop_stuff(&mut computer, drop);

    let command = "south\n";
    if TRACE_ASCII {
        println!("{}", command);
    }
    let line = read_line2(&mut computer, command);
    assert_eq!(line, "");

    // ejected?
    read_room(&mut computer, &mut Some(&mut inventory));
    assert_eq!(read_line(&mut computer), "");
    println!("hmm ==================");
    read_room(&mut computer, &mut Some(&mut inventory));
    //panic!("hmmmm");
}

// floor is in the south
fn reach_correct_weight(computer: &Computer, inventory: &[String]) {
    println!("-----------------------------------------------------------------");
    let n = inventory.len();
    let options = (1 << n) - 1;
    for bitmap in 0..options {
        let keep = inventory.iter().enumerate()
            .filter(|(i, item)| (bitmap & (1 << i)) != 0)
            .map(|(i, item)| item.as_str())
            .collect::<Vec<&str>>();
        let forget = inventory.iter().enumerate()
            .filter(|(i, item)| (bitmap & (1 << i)) == 0)
            .map(|(i, item)| item.as_str())
            .collect::<Vec<&str>>();
        attempt_weight(&computer, &keep, &forget);
    }
}

fn enter_floor(computer: &mut Computer, inventory: &mut Vec<String>) {
    let command = "south\n";
    if TRACE_ASCII {
        println!("{}", command);
    }
    let line = read_line2(computer, command);
    assert_eq!(line, "");

    // ejected?
    read_room(computer, &mut Some(inventory));
}

fn embark(program: &[i64]) {
    let mut prog = program.to_vec();
    prog.resize(prog.len() + prog.len(), 0);
    let mut computer = Computer {
        program: prog,
        ip: 0,
        base: 0
    };

    assert_eq!(read_line(&mut computer), "");

    let mut map = Map { rooms: HashMap::new(), edges: HashMap::new() };
    let mut inventory = Vec::new();
    crawl_dungeon(&mut computer, &mut map, &mut Some(&mut inventory));
    //["weather machine", "polygon", "candy cane", "manifold", "dehydrated water", "hypercube", "dark matter", "bowl of rice"]
    println!("{:?}", inventory);
    enter_checkpoint(&mut computer, &mut map, &mut Some(&mut inventory));
    //let right_inventory = reach_correct_weight(&computer, &inventory);
    let droppings = ["weather machine", "polygon", "manifold", "hypercube"];
    drop_stuff(&mut computer, &droppings);
    for d in &droppings {
        let i = inventory.iter().position(|item| item == d).unwrap();
        inventory.remove(i);
    }
    enter_floor(&mut computer, &mut inventory);
    loop {
        read_line(&mut computer);
    }

}

fn main() {
    let program: Vec<i64> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    //let map = crawl_map(&program);
    embark(&program);
}
