use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Unit {
    x: i32,
    y: i32,
    hp: usize,
    charclass: char, // 'E' | 'G'
}

impl Unit {
    fn clone(&self) -> Unit {
        Unit {
            x: self.x, y: self.y,
            hp: self.hp, charclass: self.charclass
        }
    }
}

fn dump(map: &[Vec<char>], units: &[Unit]) {
    for (y, row) in map.iter().enumerate() {
        for (x, map) in row.iter().enumerate() {
            if let Some(unit) = units.iter().find(|u| u.x == x as i32 && u.y == y as i32 && u.hp > 0) {
                print!("\x1b[1;32m{}\x1b[0m", unit.charclass);
            } else {
                print!("{}", map);
            }
        }
        println!("");
    }
    println!("");
}

fn reorder(units: &mut [Unit]) {
    units.sort_unstable_by(|a, b| (a.y, a.x).cmp(&(b.y, b.x)));
}

fn walkable(map: &[Vec<char>], units: &[Unit], x: i32, y: i32) -> bool {
    let ok_terrain = map[y as usize][x as usize] == '.';
    let free_pos = !units.iter().any(|u| u.x == x && u.y == y && u.hp > 0);
    ok_terrain && free_pos
}

fn pathfind(map: &[Vec<char>], units: &[Unit], x: i32, y: i32) -> HashMap<(i32, i32), (usize, i32, i32)> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();

    let needs_check = |x, y, distances: &HashMap<_, _>|
        walkable(map, units, x, y) && !distances.contains_key(&(x, y));

    for &(xi, yi) in &[(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)] {
        if walkable(map, units, xi, yi) {
            queue.push_back((xi, yi, xi, yi, 0));
            distances.insert((xi, yi), (0, xi, yi));
        }
    }

    while let Some(current) = queue.pop_front() {
        let (xi, yi, x0, y0, dist) = current;

        for &(xj, yj) in &[(xi - 1, yi), (xi + 1, yi), (xi, yi - 1), (xi, yi + 1)] {
            if needs_check(xj, yj, &distances) {
                queue.push_back((xj, yj, x0, y0, dist + 1));
                distances.insert((xj, yj), (dist + 1, x0, y0));
            }
        }
    }

    distances
}

fn punchable<'a>(units: &'a mut Vec<Unit>, player: &Unit) -> Option<&'a mut Unit> {
    let enemies = units.iter_mut().filter(|e| e.charclass != player.charclass && e.hp > 0);
    let in_range = enemies.filter(|e| (e.x - player.x).abs() + (e.y - player.y).abs() == 1);
    let mut targets: Vec<&'a mut Unit> = in_range.collect();
    // reverse order so pop works
    targets.sort_unstable_by(|a, b| (b.hp, b.y, b.x).cmp(&(a.hp, a.y, a.x)));
    targets.pop()
}

fn punch(target: &mut Unit, attack_power: usize) {
    if target.hp <= attack_power {
        target.hp = 0;
    } else {
        target.hp -= attack_power;
    }
}

// may find no enemies in this mode as well; in that case, shortcut out of combat
fn attack(units: &mut Vec<Unit>, player: usize, elf_power: usize) -> bool {
    let current = units[player].clone();
    let out = units.iter().filter(|e| e.charclass != current.charclass && e.hp > 0).count() == 0;
    if out {
        // a previous unit killed the last one
        false
    } else {
        // enemy nearby?
        if let Some(badguy) = punchable(units, &current) {
            let power = if current.charclass == 'E' { elf_power } else { 3 };
            punch(badguy, power);
        }
        true
    }
}

fn find_movement(map: &[Vec<char>], units: &mut Vec<Unit>, player: usize) -> Option<(i32, i32)> {
    let current = &units[player];
    let enemies = units.iter().filter(|&u| u.charclass != current.charclass && u.hp > 0).collect::<Vec<_>>();
    if enemies.is_empty() {
        // no targets remain, combat ends
        return None;
    }

    let mut in_range = Vec::new();

    let pathinfos = pathfind(map, units, current.x, current.y);

    // already in the range?
    for e in &enemies {
        let near_lr = current.x == e.x && (current.y - e.y).abs() == 1;
        let near_ud = current.y == e.y && (current.x - e.x).abs() == 1;
        if near_lr || near_ud {
            // go to combat, movement is trivial
            return Some((current.x, current.y));
        }
        for dst in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let e_x = e.x + dst.0;
            let e_y = e.y + dst.1;
            if let Some(&(dist, sx, sy)) = pathinfos.get(&(e_x, e_y)) {
                in_range.push((dist, e_x, e_y, sx, sy));
            }
        }
    }

    // sort by distance, then by reading order of goal (i.e., y then x), then reading order of move
    in_range.sort_unstable_by(|a, b| (a.0, a.2, a.1, a.4, a.3).cmp(&(b.0, b.2, b.1, b.4, b.3)));

    if false && player == 4 {
        println!("dist ex ey sx sy");
        for x in &in_range {
            println!("  {:?}", x);
        }
    }

    if let Some(route) = in_range.get(0) {
        Some((route.3, route.4))
    } else {
        Some((current.x, current.y))
    }
}

fn turn(map: &[Vec<char>], units: &mut Vec<Unit>, player: usize, elf_power: usize) -> bool {
    if let Some(next_pos) = find_movement(map, units, player) {
        let current = &mut units[player];
        // movement for attack is trivial
        current.x = next_pos.0;
        current.y = next_pos.1;
    }

    attack(units, player, elf_power)
}

fn step(map: &[Vec<char>], units: &mut Vec<Unit>, elf_power: usize) -> bool {
    for i in 0..units.len() {
        if units[i].hp == 0 {
            continue;
        }
        if false {
            println!("u {} to move from {},{} in:", i, units[i].x, units[i].y);
            dump(map, units);
        }
        if !turn(map, units, i, elf_power) {
            // no enemies found
            return false;
        }
    }

    true
}

fn combat(map: &[Vec<char>], units: &mut Vec<Unit>, elf_power: usize) -> usize {
    for round in 0.. {
        reorder(units);
        if false {
            println!("round {}", round);
            dump(map, units);
            for (i, u) in units.iter().enumerate() {
                println!("u {}: {:?}", i, u);
            }
        }
        if !step(map, units, elf_power) {
            // unfinished round doesn't count
            return round;
        } else {
            let sides = units.iter()
                .filter(|u| u.hp > 0)
                .map(|u| u.charclass).collect::<HashSet<char>>();
            if sides.len() == 1 {
                // this full round ended with a combat and is now finished
                return round + 1;
            }
        }
    }
    unreachable!()
}

fn play(map: &[Vec<char>], units: &Vec<Unit>) -> usize {
    let mut game_units = units.iter().map(|u| u.clone()).collect::<Vec<_>>();
    let full_rounds = combat(map, &mut game_units, 3);
    let winner_score = game_units.iter().map(|u| u.hp).sum::<usize>();
    println!("full rounds {}, pts {}", full_rounds, winner_score);
    dump(map, &game_units);
    return full_rounds * winner_score;
}

fn elves_ftw(map: &[Vec<char>], units: &Vec<Unit>) -> usize {
    for elf_power in 3.. {
        let mut game_units = units.iter().map(|u| u.clone()).collect::<Vec<_>>();
        let full_rounds = combat(map, &mut game_units, elf_power);
        let winner_score = game_units.iter().map(|u| u.hp).sum::<usize>();
        println!("pow {}, full rounds {}, pts {}", elf_power, full_rounds, winner_score);
        dump(map, &game_units);
        let losses = game_units.iter()
            .filter(|u| u.charclass == 'E' && u.hp == 0)
            .count();
        if losses == 0 {
            return full_rounds * winner_score;
        }
    }
    unreachable!()
}

fn main() {
    let mut map = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| x.unwrap().chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut units = Vec::new();
    for (y, row) in map.iter_mut().enumerate() {
        for (x, charclass) in row.iter_mut().enumerate().filter(|(_, &mut b)| "EG".contains(b)) {
            units.push(Unit {
                x: x as i32,
                y: y as i32,
                hp: 200,
                charclass: *charclass,
            });
            *charclass = '.';
        }
    }

    println!("{}", play(&map, &units));

    println!("{}", elves_ftw(&map, &units));
}
