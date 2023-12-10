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

fn do_fill(sketch: &Sketch, pos: Coord, visit_map: &mut HashMap<Coord, char>) {
    let maxx = sketch[0].len() as i32;
    let maxy = sketch.len() as i32;
    if pos.0 < 0 || pos.0 >= maxx || pos.1 < 0 || pos.1 >= maxy {
        return;
    }
    if visit_map.contains_key(&pos) {
        return;
    }
    visit_map.insert(pos, 'I');
    do_fill(sketch, (pos.0 - 1, pos.1), visit_map);
    do_fill(sketch, (pos.0 + 1, pos.1), visit_map);
    do_fill(sketch, (pos.0, pos.1 - 1), visit_map);
    do_fill(sketch, (pos.0, pos.1 + 1), visit_map);
}

fn cross_product(a: Coord, b: Coord) -> i32 {
    a.0 * b.1 - a.1 * b.0
}

fn winding(positions: &[Coord]) -> i32 {
   positions.iter()
        .zip(positions.iter().cycle().skip(1))
        .zip(positions.iter().cycle().skip(2))
        .map(|((p0, p1), p2)| {
            let d0 = (p1.0 - p0.0, p1.1 - p0.1);
            let d1 = (p2.0 - p1.0, p2.1 - p1.1);
            // 0 for no change, -1 or +1 for 90 degree turns
            cross_product(d0, d1)
        })
        .sum::<i32>()
}

fn inside(sketch: &Sketch, loop_positions: &[Coord]) -> usize {
    if winding(loop_positions) < 0 {
        // make it clockwise
        let flipped = loop_positions.iter().rev().copied().collect::<Vec<_>>();
        return inside(sketch, &flipped);
    }
    let mut visit_map = HashMap::new();
    for &p in loop_positions {
        // stop the fill without a separate test in do_fill()
        visit_map.insert(p, 'x');
    }
    let mut fill = |coord: Coord| do_fill(sketch, coord, &mut visit_map);
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
    if false {
        let maxx = sketch[0].len() as i32;
        let maxy = sketch.len() as i32;
        for y in 0..maxy {
            for x in 0..maxx {
                print!("{}", visit_map.get(&(x, y)).unwrap_or(&'?'));
            }
            println!();
        }
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
                // the plan was to have these in consistent order such that .0 always loops clockwise
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
    println!("{}", inside(&sketch, &positions));
}
