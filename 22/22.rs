use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Tile {
    Open,
    Wall,
}
use Tile::*;

type Coords = (i64, i64);

type Map = HashMap<Coords, Tile>;

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Left,
    Right,
}
use Rotation::*;

// The input begins and ends with walking, so this is injected with extra rotation in the front.
// Heading is initially up, this begins with Right to bring the heading to the right as specified.
type Route = Vec<(Rotation, i64)>;
type Notes = (Map, Route);

// like ccw
fn left(dir: Coords) -> Coords {
    (dir.1, -dir.0)
}

// like cw
fn right(dir: Coords) -> Coords {
    (-dir.1, dir.0)
}

fn add(a: Coords, b: Coords) -> Coords {
    (a.0 + b.0, a.1 + b.1)
}

fn sub(a: Coords, b: Coords) -> Coords {
    (a.0 - b.0, a.1 - b.1)
}

fn scale(a: Coords, b: Coords) -> Coords {
    (a.0 * b.0, a.1 * b.1)
}

fn wrap_flat(map: &Map, pos: Coords, dir: Coords) -> (Coords, Coords) {
    (*match dir {
        (1,  0) => map.keys().filter(|&&(_, y)| y == pos.1).min_by_key(|&(x, _)| x),
        (-1, 0) => map.keys().filter(|&&(_, y)| y == pos.1).max_by_key(|&(x, _)| x),
        (0,  1) => map.keys().filter(|&&(x, _)| x == pos.0).min_by_key(|&(_, y)| y),
        (0, -1) => map.keys().filter(|&&(x, _)| x == pos.0).max_by_key(|&(_, y)| y),
        _ => panic!()
    }.unwrap(), dir)
}

// row-major order
type Mat = (Coords, Coords);

fn matmul(mat: Mat, v: Coords) -> Coords {
    let ((a, b), (c, d)) = mat;
    let (v0, v1) = v;
    ((a * v0 + b * v1), (c * v0 + d * v1))
}

fn wrap_cube(map: &Map, pos: Coords, dir: Coords) -> (Coords, Coords) {
    // remember, y grows down. these aoc coordinate systems...
    let _flip_y = ((1, 0), (0, -1));
    let _flip_x = ((-1, 0), (0, 1));
    let fullturn = ((-1, 0), (0, -1));
    let turn_ccw = ((0, 1), (-1, 0)); // prev y becomes x, prev x becomes -y
    let turn_cw = ((0, -1), (1, 0)); // prev x becomes y, prev y becomes -x
    // Tform: (dest face index, edge axis rot matrix, offset of origin of new axis)
    // (could also use homogeneous coordinates for just one matrix...)
    // newpos = dest_face_origin + axis_rotate(src_faceoff - src_ax_offset) + dest_ax_offset
    // src_ax_offset derived from exit direction to avoid redundancy
    type Tform = Option<(Coords, Mat, Coords)>;
    let facesize = match map.len() {
        96 /* 6*4*4 */ => 4,
        15000 /* 6*50*50 */ => 50,
        _ => panic!("unusual input"),
    };
    // TODO: make these into lang symbols like U left, U right, upside-U left, just-turn-axis-right
    // TODO: derive dest_ax_offset from edge transition
    let faceend = facesize - 1;
    // (dest face, (right left bottom top))
    let transform_map: Vec<(Coords, (Tform, Tform, Tform, Tform))>
        = match map.len() {
        96 /* 6*4*4 */ => vec![
            ((2, 0), (
                Some(((3, 2), fullturn, (faceend, faceend))),
                Some(((1, 1), turn_ccw, (0, 0))),
                None,
                Some(((0, 1), fullturn, (0, 0))),
            )),
            ((0, 1), (
                None,
                Some(((3, 2), turn_cw, (faceend, faceend))),
                Some(((2, 2), fullturn, (faceend, faceend))),
                Some(((2, 0), fullturn, (faceend, 0))),
            )),
            ((1, 1), (
                None,
                None,
                Some(((2, 2), turn_ccw, (0, 0))),
                Some(((2, 0), turn_cw, (0, 0))),
            )),
            ((2, 1), (
                Some(((3, 2), turn_cw, (faceend, 0))),
                None,
                None,
                None,
            )),
            ((2, 2), (
                None,
                Some(((1, 1), turn_cw, (faceend, 0))),
                Some(((0, 1), fullturn, (faceend, faceend))),
                None,
            )),
            ((3, 2), (
                Some(((2, 0), fullturn, (faceend, faceend))),
                None,
                Some(((0, 1), turn_ccw, (0, faceend))),
                Some(((2, 1), turn_ccw, (0, faceend))),
            )),
        ],
        15000 /* 6*50*50 */ => vec![],
        _ => panic!("unusual input"),
    };
    // note: x,y at the edge, not necessarily overlapping major axes of the face (at 0,0)
    // so src edge vectors also managed in the transform

    let src_faceidx = (pos.0 / facesize, pos.1 / facesize);
    let src_faceoff = (pos.0 % facesize, pos.1 % facesize);

    let transforms = transform_map.iter().find(|t| t.0 == src_faceidx).expect("this face does not flip").1;
    let (tform, src_ax_offset) = match dir {
        (1,  0) => (transforms.0.expect("flip on contig edge?"), (faceend, 0)),
        (-1, 0) => (transforms.1.expect("flip on contig edge?"), (0, 0)),
        (0,  1) => (transforms.2.expect("flip on contig edge?"), (0, faceend)),
        (0, -1) => (transforms.3.expect("flip on contig edge?"), (0, 0)),
        _ => panic!()
    };
    // make src coords run along an axis on the edge so the rotation just works
    let src_edgeoff = sub(src_faceoff, src_ax_offset);

    let (dest_faceidx, axisrotate, dest_ax_offset) = tform;
    let dest_face_origin = scale(dest_faceidx, (facesize, facesize));

    let dest_faceoff = add(matmul(axisrotate, src_edgeoff), dest_ax_offset);

    let finalpos = add(dest_face_origin, dest_faceoff);
    let finaldir = matmul(axisrotate, dir);
    if false {
        println!("transformed {:?} (off {:?}) into {:?} (off {:?})",
            pos, src_faceoff,
            add(dest_face_origin, dest_faceoff), dest_faceoff);
    }

    (finalpos, finaldir)
}

fn final_pos<F: Fn(&Map, Coords, Coords) -> (Coords, Coords)>(notes: &Notes, wrap: F) -> (Coords, Coords) {
    let mut pos: Coords = *notes.0.keys().filter(|&&(_, y)| y == 0).min_by_key(|&(x, _)| x).unwrap();
    let mut dir = (0, -1); // up
    for &(rot, steps) in &notes.1 {
        dir = match rot {
            Left => left(dir),
            Right => right(dir),
        };
        for _ in 0..steps {
            let next = add(pos, dir);
            let (next, ndir, tile) = match notes.0.get(&next) {
                Some(tile) => (next, dir, tile),
                None => {
                    if false {
                        println!("found nothin at from {:?} to {:?}", pos, next);
                    }
                    let (next, nextdir) = wrap(&notes.0, pos, dir);
                    (next, nextdir, notes.0.get(&next).unwrap())
                }
            };
            let (newpos, newdir) = match tile {
                Open => (next, ndir),
                Wall => (pos, dir),
            };
            pos = newpos;
            dir = newdir;
        }
    }
    (pos, dir)
}

fn password((pos, dir): (Coords, Coords)) -> i64 {
    let facing = match dir {
        (1,  0) => 0,
        (0,  1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!()
    };
    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + facing
}

fn final_password_flat(notes: &Notes) -> i64 {
    password(final_pos(notes, wrap_flat))
}

/*
 * Map into this form, depending on if the input is taller than wide or not:
 * .F.  ..G.
 * .E.  FEAD
 * BAG  ..B.
 * .D.
 */
fn canonical_cube(map: Map) -> Map {
    // FIXME: non-hardcoded face mapping
    // - perhaps map only the wrapping rules, not the data
    map
}

fn final_password_cube(notes: Notes) -> i64 {
    password(final_pos(&(canonical_cube(notes.0), notes.1), wrap_cube))
}

fn parse_map(note: &[String]) -> Map {
    note.iter().enumerate().flat_map(|(y, row)| {
        row.chars().enumerate().filter_map(move |(x, ch)| {
            match ch {
                ' ' => None,
                '.' => Some(((x as i64, y as i64), Open)),
                '#' => Some(((x as i64, y as i64), Wall)),
                _ => panic!("bad input"),
            }
        })
    }).collect::<Map>()
}

// 10R5L5R10L4R5L5
fn parse_route(mut note: &str) -> Route {
    // begins with a number
    let mut route = Vec::new();
    let mut prev_rot = Right;
    while let Some(pos) = note.as_bytes().iter().position(|&a| a == b'L' || a == b'R') {
        let (steps, rest) = note.split_at(pos);
        route.push((prev_rot, steps.parse().unwrap()));
        prev_rot = match rest.chars().next().unwrap() {
            'L' => Left,
            'R' => Right,
            _ => panic!()
        };
        note = rest.split_at(1).1;
    }
    route.push((prev_rot, note.parse().unwrap()));
    route
}

fn parse_notes(data: &[String]) -> Notes {
    let mut sp = data.split(|row| row == "");
    let map = parse_map(sp.next().unwrap());
    let route = parse_route(&sp.next().unwrap()[0]);
    (map, route)
}

fn main() {
    let data: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let notes = parse_notes(&data);
    println!("{}", final_password_flat(&notes));
    println!("{}", final_password_cube(notes));
}
