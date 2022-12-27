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

// map face -> canon face, rotation (which canon face heads right in map world)
type TransformMap = HashMap<Coords, (Coords, usize)>;
// canonical face -> canonical face, axis rotation (per each edge)
type TransformWarps = HashMap<Coords, [(Coords, usize); 4]>;

struct Cube {
    map: Map,
    map_to_canon: TransformMap,
    canonical_warps: TransformWarps,
}

#[derive(Copy, Clone, Debug)]
enum RouteRotation {
    Left,
    Right,
}
use RouteRotation::*;

// The input begins and ends with walking, so this is injected with extra rotation in the front.
// Heading is initially up, this begins with Right to bring the heading to the right as specified.
type Route = Vec<(RouteRotation, i64)>;
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

// could use heading everywhere, but these direction vectors existed first...
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
    let noop     = ((1, 0),  (0, 1));
    let turn_cw  = ((0, -1), (1, 0));  // right; prev x becomes y, prev y becomes -x
    let fullturn = ((-1, 0), (0, -1)); // twice left, or twice back; x and y both negate
    let turn_ccw = ((0, 1),  (-1, 0)); // left; prev y becomes x, prev x becomes -y
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

fn wrap_canonical(cube: &Cube, src_face: Coords, src_heading: usize)
-> (Coords, usize) {
    let canon_neighs = cube.canonical_warps.get(&src_face).unwrap();
    let (dest_face, dest_rot_rel) = canon_neighs[src_heading];
    let dest_heading = rotate_heading(src_heading, dest_rot_rel);

    (dest_face, dest_heading)
}

fn canonical_from_map(cube: &Cube, map_face: Coords, map_heading: usize)
-> (Coords, usize) {
    // - which face this is in canonical world
    // - which edge of a canonical face is facing right on the map
    let &(canon_face, rotation) = cube.map_to_canon.get(&map_face).unwrap();
    let canon_heading = rotate_heading(map_heading, rotation);

    (canon_face, canon_heading)
}

fn map_from_canonical(cube: &Cube, canon_face: Coords, canon_heading: usize)
-> (Coords, usize) {
    let (&faceidx, &(_, rotation_inv)) =
        cube.map_to_canon.iter().find(|(_, v)| v.0 == canon_face).unwrap();
    let map_heading = rotate_heading(canon_heading, invert_heading(rotation_inv));

    (faceidx, map_heading)
}

fn wrap_discovered_cube(cube: &Cube, pos: Coords, dir: Coords) -> (Coords, Coords) {
    let face_size = ((cube.map.len() / 6) as f64).sqrt() as i64;
    let face_end = face_size - 1;
    let src_faceidx = (pos.0 / face_size, pos.1 / face_size);
    let src_faceoff = (pos.0 % face_size, pos.1 % face_size);

    let src_map_heading = heading_from_dir(dir);

    // make src coords run along an axis on the edge so the rotation just works
    let src_edge_ax_origins = [(face_end, 0), (0, face_end), (0, 0), (0, 0)];
    let src_ax_origin = src_edge_ax_origins[src_map_heading];
    let src_edgeoff = sub(src_faceoff, src_ax_origin);
    assert!(src_edgeoff.0 == 0 || src_edgeoff.1 == 0);

    // move to the coordinate system where the wraps are defined
    let (src_canon_face, src_canon_heading)
        = canonical_from_map(cube, src_faceidx, src_map_heading);

    // where the edge wraps to
    let (dest_canon_face, dest_canon_heading)
        = wrap_canonical(cube, src_canon_face, src_canon_heading);

    // inverse-lookup the destination
    let (dest_faceidx, dest_map_heading)
        = map_from_canonical(cube, dest_canon_face, dest_canon_heading);

    // note: the above three and some of this below could also be just one transform precalculated

    let dest_face_origin = scale(dest_faceidx, (face_size, face_size));

    // clockwise from origin: top left, top right, bottom right, bottom left
    let corners = [(0, 0), (face_end, 0), (face_end, face_end), (0, face_end)];
    // src heading right down left up: which dest corner represents origin in noop moves
    let heading_offs = [0, 0, 1, 3];

    // "dest - src" in heading math
    let relative_rotation = rotate_heading(dest_map_heading, invert_heading(src_map_heading));
    let dest_edgeoff = rotate_vec(src_edgeoff, relative_rotation);
    let dest_ax_origin = corners[(heading_offs[src_map_heading] + relative_rotation) % 4];

    // final position within destination face
    let dest_faceoff = add(dest_ax_origin, dest_edgeoff);
    assert!(dest_faceoff.0 >= 0);
    assert!(dest_faceoff.0 < face_size);
    assert!(dest_faceoff.1 >= 0);
    assert!(dest_faceoff.1 < face_size);

    let new_pos = add(dest_face_origin, dest_faceoff);
    let new_dir = dir_from_heading(dest_map_heading);

    (new_pos, new_dir)
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

// map_face now determined to be canon_face, rotation known; if exists, save and repeat for neighbors
// either this is a primary canonical face, or coming to this from a known face
fn face_discovery(map: &Map, face_size: i64, cwarps: &TransformWarps, map_face: Coords, canon_face: Coords, rot_on_map: usize, map_to_c_warps: &mut TransformMap) {
    let topleft_pixel = scale(map_face, (face_size, face_size));

    if !map.contains_key(&topleft_pixel) {
        // there was an attempt, but this is not a face we're looking for
        return;
    }

    let tform = (canon_face, rot_on_map);
    if let Some(tform_old) = map_to_c_warps.insert(map_face, tform) {
        // already visited
        assert_eq!(tform_old, tform);
        return;
    }

    // right, down, left, top
    let map_neighs = &[(1, 0), (0, 1), (-1, 0), (0, -1)];
    let canonical_neighs = cwarps.get(&canon_face).unwrap();
    // when the face is rotated, these edges rotate correspondingly
    for (&nface_delta, &(n_canon_face, n_axis_rot)) in map_neighs.iter().zip(
            canonical_neighs.iter().cycle().skip(rot_on_map)) {
        let n_map_face = add(map_face, nface_delta);
        let n_rot = (rot_on_map + n_axis_rot) % 4;

        face_discovery(map, face_size, cwarps, n_map_face, n_canon_face, n_rot, map_to_c_warps);
    }
}

/*
 * Make a face lookup into this form, depending on if the input is taller than wide or not:
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
    // ###
    //   ###
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
    let (r_0, r_r, r_f, r_l) = (0, 1, 2, 3);
    // relative positions and rotations across face pairs on edges
    // same as heading score: right bot left top
    // rotations are from this face to the neighbor for the edge axis and the heading
    let canonical_warps: TransformWarps = [
        (f, [(g, r_f), (e, r_0), (b, r_f), (d, r_0)]),
        (e, [(g, r_r), (a, r_0), (b, r_l), (f, r_0)]),
        (b, [(a, r_0), (d, r_l), (f, r_f), (e, r_r)]),
        (a, [(g, r_0), (d, r_0), (b, r_0), (e, r_0)]),
        (g, [(f, r_f), (d, r_r), (a, r_0), (e, r_l)]),
        (d, [(g, r_l), (f, r_0), (b, r_r), (a, r_0)]),
    ].into_iter().collect();

    // some pair validation, not completely waterproof though if both are wrong identically
    let mut npairs = 0;
    for (face, edges) in &canonical_warps {
        for (e, er) in edges.iter() {
            for (ne, ner) in canonical_warps.get(e).unwrap() {
                if *ne == *face {
                    assert_eq!((er + ner) % 4, 0);
                    npairs += 1;
                }
            }
        }
    }
    assert_eq!(npairs, 24);

    // 1. canonical face order is a known fact
    // 2. need map face edge/face edge mapping to wrap at map discontiguities
    // 3. first determine map face / canon face correspondence to know where faces are
    // 4. when all canon face poses are known on map, transform canon edges vs map edges

    // which face do the tiles provide data for
    let mut map_to_canon = TransformMap::new();
    if w_faces < h_faces {
        /*
         * .F.
         * .E.
         * BAG
         * .D.
         */
        // Not sure which ones of these are set, so try each one to ensure the world is visited.
        // The map will be visited entirely from the first hit, though; the rest are validation.
        for &face in canonical_faces {
            face_discovery(&map, face_size, &canonical_warps, face, face, r_0, &mut map_to_canon);
        }
    } else if w_faces > h_faces {
        /*
         * ..G.
         * FEAD
         * ..B.
         */
        for &face in canonical_faces {
            let rotated_face = (face.1, 2 - face.0);
            face_discovery(&map, face_size, &canonical_warps, rotated_face, face, r_r, &mut map_to_canon);
        }
    } else {
        panic!("square input not possible");
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
    password(final_pos(&(canonical_cube(notes.0), notes.1), wrap_discovered_cube))
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
