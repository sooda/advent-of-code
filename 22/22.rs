use std::io::{self, BufRead};
use std::collections::{HashMap};

#[derive(Copy, Clone, Debug)]
enum Tile {
    Open,
    Wall,
}
use Tile::*;

type Coords = (i64, i64);

type Map = HashMap<Coords, Tile>;
struct Cube {
    map: Map,
    map_to_canon: TransformMap,
    canonical_warps: TransformWarps,
}

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Left,
    Right,
}
use Rotation::*;

// The input begins and ends with walking, so this is injected with extra rotation in the front.
// Heading is initially up, this begins with Right to bring the heading to the right as specified.
type Route = Vec<(Rotation, i64)>;
type Notes = (Cube, Route);

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

fn wrap_flat(cube: &Cube, pos: Coords, dir: Coords) -> (Coords, Coords) {
    (*match dir {
        (1,  0) => cube.map.keys().filter(|&&(_, y)| y == pos.1).min_by_key(|&(x, _)| x),
        (-1, 0) => cube.map.keys().filter(|&&(_, y)| y == pos.1).max_by_key(|&(x, _)| x),
        (0,  1) => cube.map.keys().filter(|&&(x, _)| x == pos.0).min_by_key(|&(_, y)| y),
        (0, -1) => cube.map.keys().filter(|&&(x, _)| x == pos.0).max_by_key(|&(_, y)| y),
        _ => panic!()
    }.unwrap(), dir)
}

fn heading_from_dir(dir: Coords) -> usize {
    match dir {
        (1,  0) => 0, // >
        (0,  1) => 1, // v
        (-1, 0) => 2, // <
        (0, -1) => 3, // ^
        _ => panic!()
    }
}
fn dir_from_heading(heading: usize) -> Coords {
    match heading {
        0 => (1,  0), // >
        1 => (0,  1), // v
        2 => (-1, 0), // <
        3 => (0, -1), // ^
        _ => panic!()
    }
}

// row-major order
type Mat = (Coords, Coords);

fn matmul(mat: Mat, v: Coords) -> Coords {
    let ((a, b), (c, d)) = mat;
    let (v0, v1) = v;
    ((a * v0 + b * v1), (c * v0 + d * v1))
}

fn rotate_vec(v: Coords, heading: usize) -> Coords {
    // could also just do left(left(foo)) etc
    let noop = ((1, 0), (0, 1));
    let turn_cw = ((0, -1), (1, 0)); // right; prev x becomes y, prev y becomes -x
    let fullturn = ((-1, 0), (0, -1));// twice left, or twice back; x and y both negate
    let turn_ccw = ((0, 1), (-1, 0)); // left; prev y becomes x, prev x becomes -y
    let edge_axis_turns = &[noop, turn_cw, fullturn, turn_ccw];

    matmul(edge_axis_turns[heading], v)
}

// this math is trivial but it's good to give it a name
fn rotate_heading(heading: usize, rot: usize) -> usize {
    (heading + rot + 4) % 4
}

// this math is trivial but it's good to give it a name
fn invert_heading(heading: usize) -> usize {
    // can't apply unary minus to usize, urgh
    rotate_heading(0, 4 - heading)
}

fn wrap_canonical(cube: &Cube, src_face: Coords, src_dir: usize, src_edgeoff: Coords)
-> (Coords, usize, Coords) {
    let canon_neighs = cube.canonical_warps.get(&src_face).unwrap();
    let (dest_face, dest_rot_rel) = canon_neighs[src_dir];
    let dest_rot_rel = dest_rot_rel as usize;

    let dest_dir = rotate_heading(src_dir, dest_rot_rel);
    let dest_edgeoff = rotate_vec(src_edgeoff, dest_rot_rel);
    (dest_face, dest_dir, dest_edgeoff)
}

fn wrap_discovered_cube(cube: &Cube, pos: Coords, dir: Coords) -> (Coords, Coords) {
    let map = &cube.map;
    let map_to_canon = &cube.map_to_canon;

    let face_size = ((map.len() / 6) as f64).sqrt() as i64;
    let face_end = face_size - 1;
    let src_faceidx = (pos.0 / face_size, pos.1 / face_size);
    let src_faceoff = (pos.0 % face_size, pos.1 % face_size);

    let src_map_heading = heading_from_dir(dir);

    // make src coords run along an axis on the edge so the rotation just works
    let src_edge_ax_origins = [(face_end, 0), (0, face_end), (0, 0), (0, 0)];
    let src_ax_origin = src_edge_ax_origins[src_map_heading];
    let src_edgeoff = sub(src_faceoff, src_ax_origin);
    assert!(src_edgeoff.0 == 0 || src_edgeoff.1 == 0);

    // - which face this is in canonical world
    // - which edge of a canonical face is facing right on the map
    let &(src_canon_face, src_canon_rot2map) = map_to_canon.get(&src_faceidx).unwrap();
    // vector along the equivalent edge in canonical world
    let src_canon_edgeoff = rotate_vec(src_edgeoff, src_canon_rot2map);
    // current map heading as seen in canonical world
    let src_canon_heading = rotate_heading(src_map_heading, src_canon_rot2map);

    if false {
        println!("ahoyyy src face {:?}, src edgeoff {:?}, src dir {:?}",
                 src_faceidx, src_edgeoff, src_map_heading);
        println!("ahoyyy canon face {:?}, canon rot2map {:?}, canon edgeoff {:?}, canon dir {:?}",
                 src_canon_face, src_canon_rot2map, src_canon_edgeoff, src_canon_heading);
    }

    // where the edge wraps to
    let (dest_canon_face, dest_canon_heading, dest_canon_edgeoff) =
        wrap_canonical(cube, src_canon_face, src_canon_heading, src_canon_edgeoff);
    // inverse-lookup the destination
    let (&dest_faceidx, &(_, dest_canon_rot2map)) =
        map_to_canon.iter().find(|(_, v)| v.0 == dest_canon_face).unwrap();
    let dest_face_origin = scale(dest_faceidx, (face_size, face_size));
    let dest_map_heading = rotate_heading(dest_canon_heading, invert_heading(dest_canon_rot2map));
    let dest_map_edgeoff = rotate_vec(dest_canon_edgeoff, invert_heading(dest_canon_rot2map));

    if false {
        println!("ahoya dest face in map: {:?} rotated {}, new heading {}, on canon edge: {:?}",
                 dest_faceidx, dest_canon_rot2map, dest_map_heading, dest_canon_edgeoff);
        println!("and on map edge: {:?}",
                 dest_map_edgeoff);
    }

    let (tl, tr, bl, br) = ((0, 0), (face_end, 0), (0, face_end), (face_end, face_end));

    // corner connectivity: get destination face corner that's the origin of this axis
    // where source heading goes when rotating that way
    // could also build this with dest heading, it's equivalent
    // FIXME: this is probably cyclic, simplify
    let corner_map = [
        [tl, tl, tr, bl], // rot 0: noop
        [tr, tr, br, tl], // rot 1: cw right
        [br, br, bl, tr], // rot 2: flip
        [bl, bl, tl, br], // rot 3: ccw left
    ];
    let relative_rotation = (dest_map_heading + 4 - src_map_heading) % 4;
    let dest_ax_origin = corner_map[relative_rotation][src_map_heading];

    // final position within destination face
    let dest_faceoff = add(dest_ax_origin, dest_map_edgeoff);

    if false {
        println!("dest ax corner on map {:?}", dest_ax_origin);
        println!("corner chosen by {:?} and {:?}", relative_rotation, src_map_heading);
        println!("point within new box is {:?}", dest_faceoff);
    }

    let new_pos = add(dest_face_origin, dest_faceoff);
    let new_dir = dir_from_heading(dest_map_heading);

    if false {
        println!("discovery-based transform {:?} (off {:?}) with dir {:?} into {:?} (off {:?}) with dir {:?}",
            pos, src_faceoff, dir,
            new_pos, dest_faceoff, new_dir);
        println!();
    }

    (new_pos, new_dir)
}

fn wrap_cube(cube: &Cube, pos: Coords, dir: Coords) -> (Coords, Coords) {
    let map = &cube.map;
    // remember, y grows down. these aoc coordinate systems...
    let _flip_y = ((1, 0), (0, -1));
    let _flip_x = ((-1, 0), (0, 1));
    let fullturn = ((-1, 0), (0, -1));
    let turn_ccw = ((0, 1), (-1, 0)); // prev y becomes x, prev x becomes -y
    let turn_cw = ((0, -1), (1, 0)); // prev x becomes y, prev y becomes -x
    let noop = ((1, 0), (0, 1));
    type Tform = Option<(Coords, Mat, Coords)>;
    let facesize = match map.len() {
        96 /* 6*4*4 */ => 4,
        15000 /* 6*50*50 */ => 50,
        _ => panic!("unusual input"),
    };
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
        15000 /* 6*50*50 */ => vec![
            ((1, 0), (
                None,
                Some(((0, 2), fullturn, (0, faceend))),
                None,
                Some(((0, 3), turn_cw, (0, 0))),
            )),
            ((2, 0), (
                Some(((1, 2), fullturn, (faceend, faceend))),
                None,
                Some(((1, 1), turn_cw, (faceend, 0))),
                Some(((0, 3), noop, (0, faceend))),
            )),
            ((1, 1), (
                Some(((2, 0), turn_ccw, (0, faceend))),
                Some(((0, 2), turn_ccw, (0, 0))),
                None,
                None,
            )),
            ((0, 2), (
                None,
                Some(((1, 0), fullturn, (0, faceend))),
                None,
                Some(((1, 1), turn_cw, (0, 0))),
            )),
            ((1, 2), (
                Some(((2, 0), fullturn, (faceend, faceend))),
                None,
                Some(((0, 3), turn_cw, (faceend, 0))),
                None,
            )),
            ((0, 3), (
                Some(((1, 2), turn_ccw, (0, faceend))),
                Some(((1, 0), turn_ccw, (0, 0))),
                Some(((2, 0), noop, (0, 0))),
                None,
            )),
        ],
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
    assert!(src_edgeoff.0 == 0 || src_edgeoff.1 == 0);

    let (dest_faceidx, axisrotate, dest_ax_offset) = tform;
    let dest_face_origin = scale(dest_faceidx, (facesize, facesize));

    let dest_faceoff = add(matmul(axisrotate, src_edgeoff), dest_ax_offset);
    assert!(dest_faceoff.0 >= 0);
    assert!(dest_faceoff.0 < facesize);
    assert!(dest_faceoff.1 >= 0);
    assert!(dest_faceoff.1 < facesize);

    let finalpos = add(dest_face_origin, dest_faceoff);
    let finaldir = matmul(axisrotate, dir);
    if false {
        println!("transformed {:?} (off {:?}) with dir {:?} into {:?} (off {:?}) with dir {:?}",
            pos, src_faceoff, dir,
            add(dest_face_origin, dest_faceoff), dest_faceoff, finaldir);
    }

    let better = wrap_discovered_cube(cube, pos, dir);
    assert_eq!(better.0, finalpos);
    assert_eq!(better.1, finaldir);
    (finalpos, finaldir)
}

fn final_pos<F: Fn(&Cube, Coords, Coords) -> (Coords, Coords)>(notes: &Notes, wrap: F) -> (Coords, Coords) {
    let cube = &notes.0;
    let map = &cube.map;
    let route = &notes.1;
    let mut pos: Coords = *map.keys().filter(|&&(_, y)| y == 0).min_by_key(|&(x, _)| x).unwrap();
    let mut dir = (0, -1); // up
    for &(rot, steps) in route {
        dir = match rot {
            Left => left(dir),
            Right => right(dir),
        };
        for _ in 0..steps {
            let next = add(pos, dir);
            let (next, ndir, tile) = match map.get(&next) {
                Some(tile) => (next, dir, tile),
                None => {
                    if false {
                        println!("found nothin at from {:?} to {:?}", pos, next);
                    }
                    let (next, nextdir) = wrap(cube, pos, dir);
                    (next, nextdir, map.get(&next).unwrap())
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
    1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + heading_from_dir(dir) as i64
}

fn final_password_flat(notes: &Notes) -> i64 {
    password(final_pos(notes, wrap_flat))
}

// face index, rotation
type FaceTform = (Coords, usize);
type TransformMap = HashMap<Coords, FaceTform>;
type TransformWarps = HashMap<Coords, [(Coords, usize); 4]>;

// map_face now determined to be canon_face, rotation known; if exists, save and repeat for neighbors
// either this is a primary canonical face, or coming to this from a known face
fn face_discovery(map: &Map, face_size: i64, map_face: Coords, cwarps: &TransformWarps, canon_face: Coords, rot_on_map: usize, map_to_c_warps: &mut TransformMap, debugprefix: String) {
    if map_face.0 < 0 || map_face.1 < 0 {
        return;
    }
    // FIXME map extents
    if map_face.0 > 4 || map_face.1 > 4 {
        return;
    }
    let topleft_pixel = scale(map_face, (face_size, face_size));

    if !map.contains_key(&topleft_pixel) {
        // there was an attempt, but this is not a face we're looking for
        return;
    }

    if false {
        println!("{}visiting face: map {:?} canon {:?} maprot {}",
                 debugprefix,
                 map_face,
                 canon_face,
                 rot_on_map
                 );
    }

    let tform = (canon_face, rot_on_map);
    if let Some(tform_old) = map_to_c_warps.insert(map_face, tform) {
        if false {
            println!("{}it already exists as canon {:?} rot {}",
                     debugprefix,
                     tform_old.0, tform_old.1);
        }
        assert_eq!(tform_old, tform);
        // already visited
        return;
    }

    // 0 right, 1 down, 2 left, 3 top
    let map_neighs = &[(1, 0), (0, 1), (-1, 0), (0, -1)];
    let canonical_neighs = cwarps.get(&canon_face).unwrap();
    // when the face is rotated, these edges rotate correspondingly
    for (&nface_delta, &(n_canon_face, n_axis_rot)) in map_neighs.iter().zip(
            canonical_neighs.iter().cycle().skip(rot_on_map)) {
        let n_map_face = add(map_face, nface_delta);
        let n_rot = (rot_on_map + n_axis_rot) % 4;
        if false {
            println!("{}neighy {:?} maps to canon {:?} with neigh rot {} and resulting rot {}",
                     debugprefix,
                     n_map_face, n_canon_face, n_axis_rot, n_rot);
        }

        face_discovery(map, face_size, n_map_face, cwarps, n_canon_face, n_rot, map_to_c_warps, debugprefix.clone() + "  ");
    }
}

/*
 * Make a face lookup  into this form, depending on if the input is taller than wide or not:
 * .F.  ..G.
 * .E.  FEAD
 * BAG  ..B.
 * .D.
 */
fn canonical_cube(cube: Cube) -> Cube {
    let map = cube.map;
    // the input contains six faces exactly
    let face_size = ((map.len() / 6) as f64).sqrt() as i64;
    // minx and miny both 0; the input is not left-padded
    let w = map.keys().max_by_key(|pos| pos.0).unwrap().0 - 0 + 1;
    let h = map.keys().max_by_key(|pos| pos.1).unwrap().1 - 0 + 1;
    // dimensions 4x3 or 3x4, except for this untested shape:
    // XXX
    //   XXX
    let w_faces = w / face_size;
    let h_faces = h / face_size;

    let (f, e, b, a, g, d) = (
                (1, 0),
                (1, 1),
        (0, 2), (1, 2), (2, 2),
                (1, 3),
    );
    let canonical_faces = &[f, e, b, a, g, d];
    // right = clockwise, left = ccw; f as flip
    let (r_0, r_r, r_f, r_l) = (0, 1, 2, 3); // same as heading score
    // relative positions and rotations
    // same as heading score: right bot left top (NOT same as current wrap_cube())
    // rotations are from this face to the neighbor for the edge axis and the heading
    // XXX dest edge is in the face info of the destination to avoid redundancy
    // XXX edge axis rot is inverse of cube heading rot
    // XXX also rot probably means: which edge matches map right edge when placed at that neigh spot
    let canonical_warps: TransformWarps = [
        (f, [(g, r_f), (e, r_0), (b, r_f), (d, r_0)]),
        (e, [(g, r_r), (a, r_0), (b, r_l), (f, r_0)]),
        (b, [(a, r_0), (d, r_l), (f, r_f), (e, r_r)]),
        (a, [(g, r_0), (d, r_0), (b, r_0), (e, r_0)]),
        (g, [(f, r_f), (d, r_r), (a, r_0), (e, r_l)]),
        (d, [(g, r_l), (f, r_0), (b, r_r), (a, r_0)]),
    ].into_iter().collect();

    // some pair validation, not completely waterproof though if both are wrong identically
    for (face, edges) in &canonical_warps {
        for (e, er) in edges.iter() {
            for (ne, ner) in canonical_warps.get(e).unwrap() {
                if *ne == *face {
                    assert_eq!((er + ner) % 4, 0);
                }
            }
        }
    }

    /*
    // 1. canonical face order is a known fact
    // 2. need map face edge/face edge mapping to wrap at map discontiguities
    // 3. first determine map face / canon face correspondence to know where faces are
    // 4. when all canon face poses are known on map, transform canon edges on map edges
    */
    // Not sure which ones of these are set, so try each one to ensure the world is visited.
    // It'll be visited entirely from the first hit, though.
    // XXX not necessarily as the above has to see the real ones first?
    // hmm just four uncanonical corners XXX do one by one?
    // which face do the tiles provide data for
    let mut map_to_canon = TransformMap::new();
    if w_faces < h_faces {
        /*
         * .F.
         * .E.
         * BAG
         * .D.
         */
        for &face in canonical_faces {
            face_discovery(&map, face_size, face, &canonical_warps, face, r_0, &mut map_to_canon, "".to_string());
        }
    } else if w_faces > h_faces {
        /*
         * ..G.
         * FEAD
         * ..B.
         */
        for &face in canonical_faces {
            let rotated_face = (face.1, 2 - face.0);
            face_discovery(&map, face_size, rotated_face, &canonical_warps, face, r_r, &mut map_to_canon, "".to_string());
        }
    } else {
        panic!("square input not possible");
    }
    if false {
        println!("discovered map_to_canon :: mapface -> (canonface, rotated) {:?}", map_to_canon);
    }
    Cube { map, map_to_canon, canonical_warps }
}


/*
 * These fold patterns are possible, but only two are apparently used:
 *
 *   #   #     #     #    #
 * #### #### #### #### ####
 *   #  #     #   #     #
 *
 * ###   ##   ##     #     # ##
 *   ###  ###  ### ###  ####  ##
 *         #     #   ##    #   ##
 */
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
    let cube = Cube { map, map_to_canon: Default::default(), canonical_warps: Default::default() };
    let route = parse_route(&sp.next().unwrap()[0]);
    (cube, route)
}

fn main() {
    let data: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let notes = parse_notes(&data);
    println!("{}", final_password_flat(&notes));
    println!("{}", final_password_cube(notes));
}
