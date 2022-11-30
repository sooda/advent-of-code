use std::io::{self, BufRead};
use std::collections::HashSet;

const SIDE_LENGTH: i32 = 5;
const AREA: i32 = SIDE_LENGTH * SIDE_LENGTH;

// biodiversity rating
type Rating = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Eris(Rating);

impl Eris {
    fn new() -> Self {
        Eris(0)
    }

    fn bug_at(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= SIDE_LENGTH || y < 0 || y >= SIDE_LENGTH {
            false
        } else {
            let position = y * 5 + x;
            (self.0 & (1 << position)) != 0
        }
    }

    fn infect_at(&mut self, x: i32, y: i32) {
        assert!(x >= 0 && x < SIDE_LENGTH && y >= 0 && y < SIDE_LENGTH);
        let position = y * 5 + x;
        self.0 |= 1 << position;
    }

    fn from_lines(lines: &[Vec<char>]) -> Eris {
        let mut rating = 0;
        assert!(lines.len() == 5);
        for row in lines {
            assert!(row.len() == 5);
            for &ch in row {
                assert!(ch == '.' || ch == '#');
                if ch == '#' {
                    rating |= 1 << AREA;
                }
                rating >>= 1;
            }
        }
        Eris(rating)
    }

    fn biodiversity_rating(&self) -> Rating {
        self.0
    }

    fn bugs_alive(&self) -> usize {
        self.0.count_ones() as usize
    }

    fn dump(&self) {
        let mut data = self.0;
        for _y in 0..5 {
            for _x in 0..5 {
                let ch = match (data & 1) == 1 {
                    true => '#',
                    false => '.'
                };
                data >>= 1;
                print!("{}", ch);
            }
            println!();
        }
    }
}

fn next_alive(bugs: Eris, x: i32, y: i32) -> bool {
    // TODO: bitmasks and popcount
    let adjacent = [
        (x - 1, y    ),
        (x + 1, y    ),
        (x    , y - 1),
        (x    , y + 1),
    ];
    let surrounding_bugs = adjacent.into_iter().filter(|&(x, y)| bugs.bug_at(x, y)).count();
    if bugs.bug_at(x, y) {
        surrounding_bugs == 1
    } else {
        surrounding_bugs == 1 || surrounding_bugs == 2
    }
}

fn round(bugs: Eris) -> Eris {
    let mut next = Eris::new();
    for y in 0..5 {
        for x in 0..5 {
            if next_alive(bugs, x, y) {
                next.infect_at(x, y);
            }
        }
    }
    next
}

fn first_repetition(mut bugs: Eris) -> Rating {
    //println!("Initial state: {:?}", bugs);
    let mut seen = HashSet::new();
    bugs.dump();
    while !seen.contains(&bugs) {
        seen.insert(bugs);
        bugs = round(bugs);
        //println!("After {} minutes: {:?}", i, bugs);
        //bugs.dump();
    }
    bugs.biodiversity_rating()
}

fn next_border_count(layers: &[Eris], index: usize, from_x: i32, from_y: i32) -> usize {
    if index == layers.len() {
        // the simulation doesn't last long enough for anything to appear here
        0
    } else if from_x == 2 && from_y == 1 {
        // top
        (0..5).filter(|&x| layers[index].bug_at(x, 0)).count()
    } else if from_x == 2 && from_y == 3 {
        // bottom
        (0..5).filter(|&x| layers[index].bug_at(x, 4)).count()
    } else if from_x == 1 && from_y == 2 {
        // left
        (0..5).filter(|&y| layers[index].bug_at(0, y)).count()
    } else if from_x == 3 && from_y == 2 {
        // right
        (0..5).filter(|&y| layers[index].bug_at(4, y)).count()
    } else {
        panic!("logic error");
    }
}

fn next_alive_recursive(layers: &[Eris], current: usize, x: i32, y: i32) -> bool {
    // TODO: bitmasks and popcount
    let adjacent = [
        (x - 1, y    ),
        (x + 1, y    ),
        (x    , y - 1),
        (x    , y + 1),
    ];
    let surrounding_bugs = adjacent.into_iter().map(|(xi, yi)| {
        if (xi, yi) == (2, 2) {
            next_border_count(layers, current + 1, x, y)
        // the simulation doesn't last long enough for anything to appear in outermost layer 0
        } else if yi == -1 && current > 0 {
            // top
            if layers[current - 1].bug_at(2, 1) { 1 } else { 0 }
        } else if yi == 5 && current > 0 {
            // bottom
            if layers[current - 1].bug_at(2, 3) { 1 } else { 0 }
        } else if xi == -1 && current > 0 {
            // left
            if layers[current - 1].bug_at(1, 2) { 1 } else { 0 }
        } else if xi == 5 && current > 0 {
            // right
            if layers[current - 1].bug_at(3, 2) { 1 } else { 0 }
        } else {
            if layers[current].bug_at(xi, yi) { 1 } else { 0 }
        }
    }).sum::<usize>();
    if layers[current].bug_at(x, y) {
        surrounding_bugs == 1
    } else {
        surrounding_bugs == 1 || surrounding_bugs == 2
    }
}

fn round_recursive(input: &[Eris]) -> Vec<Eris> {
    let depth = input.len();
    let mut output = vec![Eris::new(); depth];
    for (i, (_inlayer, outlayer)) in input.iter().zip(output.iter_mut()).enumerate() {
        for (x, y) in (0..5).flat_map(|y| (0..5).map(move |x| (x, y))) {
            if x == 2 && y == 2 {
                // this bit is never used; it's the portal to the next recursion level
                continue;
            }
            if next_alive_recursive(input, i, x, y) {
                outlayer.infect_at(x, y);
            }
        }
    }
    output
}

fn play_recursive(bugs: Eris, steps: usize) -> usize {
    // "steps/2" number of layers on top and below: border and center are two cells away
    let mut layers = vec![Eris::new(); steps + 1];
    layers[steps / 2] = bugs;
    for i in 0..steps {
        layers = round_recursive(&layers);
        if false {
            println!("after minutes {}", i + 1);
            for (i, l) in layers.iter().enumerate() {
                println!("Depth {}:", (i as i32) - (steps as i32));
                l.dump();
            }
        }
    }
    layers.into_iter().map(|bugs| bugs.bugs_alive()).sum()
}

fn main() {
    let input: Vec<Vec<char>> = io::stdin().lock().lines().map(|line|
        line.unwrap().chars().collect()).collect();
    let bugs = Eris::from_lines(&input);

    println!("{:?}", first_repetition(bugs));
    println!("{:?}", play_recursive(bugs, 200));
}
