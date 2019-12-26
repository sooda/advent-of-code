use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap, VecDeque};
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
    iodebug: bool,
}

fn execute_dungeon(computer: &mut Computer, inputs: &str) -> Option<char> {
    let mut input = inputs.bytes().map(|b| b as i64);
    while let Some((newip, newbase, newout)) =
            step(&mut computer.program, computer.ip, computer.base, &mut input) {
        computer.ip = newip;
        computer.base = newbase;
        if let Some(out) = newout {
            assert!(out <= 127);
            return Some(out as u8 as char);
        }
    }
    None
}

fn game_io(computer: &mut Computer, input: &str) -> String {
    let mut chars = Vec::new();

    while let Some(ch) = execute_dungeon(computer, input) {
        if computer.iodebug {
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

// TODO: iterator or something
fn read_line(computer: &mut Computer) -> String {
    game_io(computer, "")
}

fn read_empty_line(computer: &mut Computer) {
    let line = game_io(computer, "");
    assert_eq!(line, "");
}

fn read_expected_line(computer: &mut Computer, should_be: &str) {
    let line = game_io(computer, "");
    assert_eq!(line, should_be);
}

fn communicate_line(computer: &mut Computer, input: &str) {
    if computer.iodebug {
        println!("> {}", input);
    }
    let reply = game_io(computer, &(input.to_string() + "\n"));
    assert_eq!(reply, "");
}

#[derive(Debug, PartialEq)]
struct Room {
    title: String,
    description: String,
    doors: [bool; 4], // north, south, east, west
    items: Vec<String>,
}

const DIR_COMMANDS: &[&str] = &[ "north\n", "south\n", "east\n", "west\n" ];
const DIR_COMMANDS_OPPOSITE: &[&str] = &[ "south\n", "north\n", "west\n", "east\n" ];
const DIR_NAMES: &[&str] = &[ "north", "south", "east", "west" ];
/*
const DOOR_NORTH: usize = 0;
const DOOR_SOUTH: usize = 1;
const DOOR_EAST: usize = 2;
const DOOR_WEST: usize = 3;
*/

const FORBIDDEN_ITEMS: &[&str] = &[
    // It is suddenly completely dark! You are eaten by a Grue!
    "photons",
    // The giant electromagnet is stuck to you.  You can't move!!
    "giant electromagnet",
    // You're launched into space! Bye!
    "escape pod",
    // The molten lava is way too hot! You melt!
    "molten lava",
    // You take the infinite loop.
    // You take the infinite loop.
    // You take the infinite loop.
    "infinite loop",
];

/*
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
    read_empty_line(computer);
    read_empty_line(computer);
    let title = read_line(computer);
    let description = read_line(computer);
    read_empty_line(computer);
    read_expected_line(computer, "Doors here lead:");

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
        read_expected_line(computer, "Command?");
        if let Some(deep_pockets) = inventory {
            for grab in &items {
                let grab = &grab[2..];
                if !FORBIDDEN_ITEMS.contains(&grab) {
                    deep_pockets.push(grab.to_string());
                    communicate_line(computer, &("take ".to_string() + grab + "\n"));
                    read_expected_line(computer, &("You take the ".to_string() + grab + "."));
                    read_empty_line(computer);
                    read_expected_line(computer, "Command?");
                }
            }
        }
    } else {
        match line.as_str() {
            "Command?" => (),
            // special pressure sensitive floor
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

    let directions = DIR_COMMANDS.iter().zip(DIR_COMMANDS_OPPOSITE.iter());

    let mut this_edges = [ None, None, None, None ];

    for ((&door_exists, edge), (&walk, &back)) in doors.iter().zip(this_edges.iter_mut()).zip(directions) {
        if !door_exists {
            continue;
        }
        communicate_line(computer, walk);

        let neigh_title = crawl_dungeon(computer, map, inventory);
        *edge = Some(neigh_title);

        communicate_line(computer, back);
        let back_here = read_room(computer, inventory);
        assert_eq!(back_here.title, this_title);
    }

    match map.edges.entry(this_title.clone()) {
        Entry::Vacant(e) => e.insert(this_edges),
        Entry::Occupied(_) => panic!("logic error"),
    };

    return this_title;
}

fn raw_bfs<'a>(map: &'a Map, source_room: &'a str, dest_room: &'a str) -> HashMap<&'a str, (&'a str, usize)> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parents = HashMap::new();

    queue.push_back(source_room);
    visited.insert(source_room);

    while let Some(current) = queue.pop_front() {
        let roomlabel = current;
        if roomlabel == dest_room {
            break;
        }
        let neighs = &map.edges[roomlabel];
        for (i, neighlabel) in neighs.iter().enumerate().filter_map(|(i, n)| n.as_ref().map(|n| (i, n))) {
            let unknown = !visited.contains(neighlabel.as_str());
            if unknown {
                queue.push_back(neighlabel);
                visited.insert(neighlabel);
                parents.insert(neighlabel.as_str(), (roomlabel, i));
            }
        }
    }

    parents
}

fn find_route(map: &Map, source_room: &str, dest_room: &str) -> Vec<&'static str> {
    let parents = raw_bfs(map, source_room, dest_room);
    let mut current_label = dest_room;
    let mut route = Vec::new();
    while let Some(&(label, direction)) = parents.get(current_label) {
        current_label = label;
        route.insert(0, DIR_COMMANDS[direction]);
    }
    route
}

fn enter_checkpoint(computer: &mut Computer, map: &Map, inventory: &mut Option<&mut Vec<String>>) {
    // FIXME: strip "=="s off
    let route = find_route(map, "== Hull Breach ==", "== Security Checkpoint ==");
    for turn in route {
        communicate_line(computer, turn);
        read_room(computer, inventory);
    }
}

fn drop_stuff(computer: &mut Computer, drop: &[&str]) {
    for d in drop {
        let command = "drop ".to_string() + d + "\n";
        communicate_line(computer, &command);

        let line = read_line(computer);
        assert_eq!(line, "You drop the ".to_string() + d + ".");

        read_empty_line(computer);

        let line = read_line(computer);
        assert_eq!(line, "Command?");
    }
}

fn attempt_weight(computer: &Computer, drop: &[&str], direction: &str) -> bool {
    let mut computer = computer.clone();

    drop_stuff(&mut computer, drop);

    communicate_line(&mut computer, direction);

    read_room(&mut computer, &mut None);
    //A loud, robotic voice says "Analysis complete! You may proceed." and you enter the cockpit.
    let line = read_line(&mut computer);
    if line == "" {
        // ejected back to checkpoint
        read_room(&mut computer, &mut None);
        return false;
    } else {
        // "Santa notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly."
        return true;
    }
}

// floor is in the south
fn reach_correct_weight<'a>(computer: &Computer, inventory: &'a [String], direction: &str) -> Vec<&'a str> {
    let n = inventory.len();
    let options = 1 << n;
    for bitmap in 0..options {
        let forget = inventory.iter().enumerate()
            .filter(|(i, _item)| (bitmap & (1 << i)) == 0)
            .map(|(_i, item)| item.as_str())
            .collect::<Vec<&str>>();
        if attempt_weight(&computer, &forget, direction) {
            return forget;
        }
    }
    panic!("not sure what to drop");
}

fn floor_direction(map: &Map) -> &'static str {
    let route = find_route(map, "== Security Checkpoint ==", "== Pressure-Sensitive Floor ==");
    assert_eq!(route.len(), 1);
    route[0]
}

fn embark(computer: &mut Computer) -> String {
    read_empty_line(computer);

    // search for a complete map and collect all items that don't cause game over
    let mut map = Map { rooms: HashMap::new(), edges: HashMap::new() };
    let mut inventory = Vec::new();
    crawl_dungeon(computer, &mut map, &mut Some(&mut inventory));

    // found these funny things in my spacecraft:
    // ["weather machine", "polygon", "candy cane", "manifold", "dehydrated water", "hypercube", "dark matter", "bowl of rice"]
    // Nuutti said he got these: polygon, mutex, manifold, klein bottle, mug, loom, hypercube, pointer

    // the map dfs got us back in the origin again; go to the checkpoint
    enter_checkpoint(computer, &map, &mut Some(&mut inventory));
    // what's the way to the magical chamber with the floor?
    let direction = floor_direction(&map);

    // figure out what's the right weight allowed by the pressure-sensitive floor
    let useless_items = reach_correct_weight(&computer, &inventory, &direction);
    // on my setup: ["weather machine", "polygon", "manifold", "hypercube"]
    // leave some items here and step on the floor
    drop_stuff(computer, &useless_items);
    communicate_line(computer, &direction);
    // parse where we went, but the inventory is no longer useful
    read_room(computer, &mut None);

    // end of story
    let line = read_line(computer);
    assert_eq!(line, "Santa notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly.");
    let pw_line = read_line(computer);
    assert!(pw_line.starts_with("\"Oh, hello!"));
    pw_line.chars().filter(|&ch| ch >= '0' && ch <= '9').collect()
}

fn nice_key(k: &str) -> String {
    k.chars().filter(|&ch| ch != '=' && ch != ' ' && ch != '-').collect()
}

fn dump_graphviz(map: &Map) {
    let mut keys: Vec<&str> = map.rooms.keys().map(|s| s.as_str()).collect();
    // graphviz cares about order, so make it consistent because hashmaps aren't
    keys.sort();

    println!("digraph G {{");

    for title in keys {
        let room = &map.rooms[title];
        print!("    room_{} [label=\"{}\\l{}\\l", nice_key(&room.title), room.title, room.description);
        print!("Items:\\l");
        for i in &room.items {
            print!("{}\\l", i);
        }
        println!("\"]");
    }

    for (room_title, neighs) in &map.edges {
        for (n, label) in neighs.iter().zip(DIR_NAMES.iter()) {
            if let Some(n) = n {
                println!("    room_{} -> room_{} [label=\"{}\"]", nice_key(room_title), nice_key(n), label);
            }
        }
    }

    println!(r"}}");
}

fn play_game(program: &[i64]) -> String {
    let mut prog = program.to_vec();
    prog.resize(prog.len() + prog.len(), 0);
    let mut computer = Computer {
        program: prog,
        ip: 0,
        base: 0,
        iodebug: false,
    };
    embark(&mut computer)
}

fn crawl_map_graphviz(program: &[i64]) {
    let mut prog = program.to_vec();
    prog.resize(prog.len() + prog.len(), 0);
    let mut computer = Computer {
        program: prog,
        ip: 0,
        base: 0,
        iodebug: false,
    };

    read_empty_line(&mut computer);

    let mut map = Map { rooms: HashMap::new(), edges: HashMap::new() };
    crawl_dungeon(&mut computer, &mut map, &mut None);
    dump_graphviz(&map);
    println!("# Stuff found:");
    for item in map.rooms.values().flat_map(|room| room.items.iter()) {
        println!("# {}", item);
    }
}

fn main() {
    let program: Vec<i64> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',').map(|n| n.parse().unwrap()).collect();

    crawl_map_graphviz(&program);
    println!("# main airlock password: {}", play_game(&program));
}
