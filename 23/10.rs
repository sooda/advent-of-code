use std::io::{self, Read};
use std::collections::HashMap;

type Coord = (i32, i32);

type Map = HashMap<Coord, (Coord, Coord)>;

fn loop_len(map: &Map, spos: Coord) -> usize {
    let mut current = spos;
    let mut next = map.get(&spos).unwrap().0;
    for i in 1.. {
        let steps = map.get(&next).unwrap();
        let n = if steps.0 == current { steps.1 } else { steps.0 };
        current = next;
        next = n;
        if current == spos {
            return i;
        }
    }
    unreachable!()
}

fn parse(file: &str) -> (Map, Coord) {
    let mut spos = None;
    let mut map = Map::new();
    for (y, line) in file.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            let xy = (x, y);
            match ch {
                // these pairs must be in consistent order such that .0 always loops clockwise
                // but nope, straight pipes do not have direction
                '|' => { map.insert(xy, ((x  , y-1), (x  , y+1))); },
                '-' => { map.insert(xy, ((x+1, y  ), (x-1, y  ))); },
                'L' => { map.insert(xy, ((x  , y-1), (x+1, y  ))); },
                'J' => { map.insert(xy, ((x-1, y  ), (x  , y-1))); },
                '7' => { map.insert(xy, ((x  , y+1), (x-1, y  ))); },
                'F' => { map.insert(xy, ((x+1, y  ), (x  , y+1))); },
                '.' => (),
                // start pos will get fixed after
                'S' => {
                    map.insert(xy, ((-1, -1), (-1, -1)));
                    spos = Some(xy);
                },
                _ => panic!()
            }
        }
    }
    let spos = spos.unwrap();
    // also these so that .0 is cw
    // but no, straights are hard
    let mut neighs = map.iter().filter(|(&_, &v)| v.0 == spos || v.1 == spos);
    let a = *neighs.next().unwrap().0;
    let b = *neighs.next().unwrap().0;
    assert_eq!(neighs.next(), None);
    map.insert(spos, (a, b));
    (map, spos)
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let (map, spos) = parse(&file);
    println!("{}", loop_len(&map, spos) / 2);
}
