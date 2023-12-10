use std::io::{self, Read};
use std::collections::HashMap;

type Coord = (i32, i32);

type Map = HashMap<Coord, (Coord, Coord)>;

type Sketch = Vec<Vec<char>>;

fn loop_positions(map: &Map, spos: Coord) -> Vec<Coord> {
    let mut positions = Vec::new();
    let mut current = spos;
    let mut next = map.get(&spos).unwrap().0;
    positions.push(current);
    while next != spos {
        positions.push(next);
        let steps = map.get(&next).unwrap();
        let n = if steps.0 == current { steps.1 } else { steps.0 };
        current = next;
        next = n;
    }
    positions
}

fn do_fill(sketch: &Sketch, map: &Map, loop_positions: &[Coord], pos: Coord, visit_map: &mut HashMap<Coord, char>) {
    let maxx = sketch[0].len() as i32;
    let maxy = sketch.len() as i32;
    if pos.0 < 0 || pos.0 >= maxx || pos.1 < 0 || pos.1 >= maxy {
        return;
    }
    if loop_positions.iter().find(|&&lp| lp == pos).is_some() {
        return;
    }
    if visit_map.contains_key(&pos) {
        return;
    }
    visit_map.insert(pos, 'I');
    do_fill(sketch, map, loop_positions, (pos.0 - 1, pos.1), visit_map);
    do_fill(sketch, map, loop_positions, (pos.0 + 1, pos.1), visit_map);
    do_fill(sketch, map, loop_positions, (pos.0, pos.1 - 1), visit_map);
    do_fill(sketch, map, loop_positions, (pos.0, pos.1 + 1), visit_map);
}

fn inside(sketch: &Sketch, map: &Map, loop_positions: &[Coord]) -> usize {
    let maxx = sketch[0].len() as i32;
    let maxy = sketch.len() as i32;
    let mut visit_map = HashMap::new();
    let mut fill = |coord: Coord| do_fill(sketch, map, loop_positions, coord, &mut visit_map);
    for (p0, p1) in loop_positions.iter().zip(loop_positions.iter().skip(1)) {
        // compass directions would be more appropriate but I'm not good with them
        let dir = (p1.0 - p0.0, p1.1 - p0.1);
        let d_up = dir == ( 0, -1);
        let d_dn = dir == ( 0,  1);
        let d_l =  dir == (-1,  0);
        let d_r =  dir == ( 1,  0);
        let left =  (p1.0 - 1, p1.1    );
        let right = (p1.0 + 1, p1.1    );
        let up =    (p1.0    , p1.1 - 1);
        let down =  (p1.0    , p1.1 + 1);
        // clockwise; right-of is inside
        match sketch[p1.1 as usize][p1.0 as usize] {
            '|' if d_up => fill(right),
            '|' if d_dn => fill(left),
            '-' if d_l  => fill(up),
            '-' if d_r  => fill(down),
            'L' if d_dn => { fill(left); fill(down); },
            'L' if d_l  => {}, // tight fill corner?
            'J' if d_r  => { fill(down); fill(right); },
            'J' if d_dn => {},
            '7' if d_up => { fill(right); fill(up); },
            '7' if d_r  => {},
            'F' if d_l  => { fill(up); fill(left); },
            'F' if d_up => {},
            '.' => panic!(),
            'S' => (), // hopefully this is handled by the rest of the cells
            _ => panic!("wat {} {:?}", sketch[p1.1 as usize][p1.0 as usize], dir),
        }
    }
    for y in 0..maxy {
        for x in 0..maxx {
            print!("{}", visit_map.get(&(x, y)).unwrap_or(&'?'));
        }
        println!();
    }
    visit_map.iter().filter(|&(_, &v)| v == 'I').count()
}

fn parse(file: &str) -> (Sketch, Map, Coord) {
    let mut spos = None;
    let mut map = Map::new();
    let mut sketch = Sketch::new();
    for (y, line) in file.lines().enumerate() {
        let mut row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            row.push(ch);
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
        sketch.push(row);
    }
    let spos = spos.unwrap();
    // also these so that .0 is cw
    // but no, straights are hard
    let mut neighs = map.iter().filter(|(&_, &v)| v.0 == spos || v.1 == spos);
    let a = *neighs.next().unwrap().0;
    let b = *neighs.next().unwrap().0;
    assert_eq!(neighs.next(), None);
    map.insert(spos, (a, b));
    (sketch, map, spos)
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let (sketch, map, spos) = parse(&file);
    let positions = loop_positions(&map, spos);
    println!("{}", positions.len() / 2);
    println!("{}", inside(&sketch, &map, &positions));
    println!("{}", inside(&sketch, &map, &positions.iter().rev().copied().collect::<Vec<_>>()));
}
