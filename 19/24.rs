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
    let surrounding_bugs = adjacent.into_iter().filter(|&&(x, y)| bugs.bug_at(x, y)).count();
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

fn main() {
    let input: Vec<Vec<char>> = io::stdin().lock().lines().map(|line|
        line.unwrap().chars().collect()).collect();
    let bugs = Eris::from_lines(&input);

    println!("{:?}", first_repetition(bugs));
}
