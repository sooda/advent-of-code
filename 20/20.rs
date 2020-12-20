use std::io::{self, BufRead};
use std::collections::HashMap;

const IMAGEDIM: usize = 8;
type Image = [u16; IMAGEDIM];

// the bool encodes flip
#[derive(Debug, Copy, Clone)]
enum Orientation {
    Up(bool),
    Left(bool),
    Down(bool),
    Right(bool),
}

fn rot_orientation(ori: Orientation) -> Orientation {
    use Orientation::*;
    match ori {
        Up(f)    => Left(f),
        Left(f)  => Down(f),
        Down(f)  => Right(f),
        Right(f) => Up(f),
    }
}

// flip along x axis: upside down
fn flip_orientation(ori: Orientation) -> Orientation {
    use Orientation::*;
    match ori {
        Up(f)    => Down(!f),
        Left(f)  => Left(!f),
        Down(f)  => Up(!f),
        Right(f) => Right(!f),
    }
}

// - top bottom left right, not sure why I didn't make this a struct
// - bits run MSB left to LSB right, MSB top to LSB bottom
// - could also store these in one big u64 for more fun rotations but that's too clever
type Borders = (u16, u16, u16, u16);
#[derive(Debug, Clone, Copy)]
struct Tile {
    name: u16,
    borders: Borders,
    orientation: Orientation,
}

// dim doesn't change but it's handy to keep here
#[derive(Debug, Clone)]
struct State {
    map: Vec<Option<Tile>>,
    dim: usize,
}

impl State {
    fn new(dim: usize) -> State {
        State {
            map: vec![None; dim * dim],
            dim,
        }
    }
    fn coord(&self, x: usize, y: usize) -> usize {
        y * self.dim + x
    }
    fn at(&self, x: usize, y: usize) -> &Option<Tile> {
        &self.map[self.coord(x, y)]
    }
    fn top_border(&self, coord: usize) -> Option<u16> {
        self.map[coord].map(|tile| tile.borders.0)
    }
    fn bottom_border(&self, coord: usize) -> Option<u16> {
        self.map[coord].map(|tile| tile.borders.1)
    }
    fn left_border(&self, coord: usize) -> Option<u16> {
        self.map[coord].map(|tile| tile.borders.2)
    }
    fn right_border(&self, coord: usize) -> Option<u16> {
        self.map[coord].map(|tile| tile.borders.3)
    }
    fn accepts(&self, pos: usize, tile: &Tile) -> bool {
        assert!(self.map[pos].is_none());
        let x = pos % self.dim;
        let y = pos / self.dim;
        if y > 0 && self.bottom_border(self.coord(x, y - 1)).map(|border| border !=
                                                                 tile.borders.0).unwrap_or(false) {
            return false;
        }
        if y < self.dim - 1 && self.top_border(self.coord(x, y + 1)).map(|border| border !=
                                                                         tile.borders.1).unwrap_or(false) {
            return false;
        }
        if x > 0 && self.right_border(self.coord(x - 1, y)).map(|border| border !=
                                                                tile.borders.2).unwrap_or(false) {
            return false;
        }
        if x < self.dim - 1 && self.left_border(self.coord(x + 1, y)).map(|border| border !=
                                                                          tile.borders.3).unwrap_or(false) {
            return false;
        }
        true
    }
}

fn flipbits(mut bits: u16) -> u16 {
    // careful, just the lowest 10 bits, not 16
    // 0123456789
    // 9876543210
    let mut out = 0;
    for _ in 0..(IMAGEDIM + 2) {
        out <<= 1;
        out |= bits & 1;
        bits >>= 1;
    }
    out
}

// counting from the right, MSB is top
fn img_column(image: &Image, col: usize) -> u16 {
    image.iter().fold(0, |dst, srcrow| (dst << 1) | ((srcrow >> col) & 1))
}

fn rotate_img(image: Image) -> Image {
    let mut out = [0; IMAGEDIM];
    for y in 0..IMAGEDIM {
        out[y] = img_column(&image, y);
    }
    out
}

fn flip_img(image: Image) -> Image {
    let mut out = [0; IMAGEDIM];
    for y in 0..IMAGEDIM {
        out[IMAGEDIM - 1 - y] = image[y];
    }
    out
}

fn orient_image(original: Image, ori: Orientation) -> Image {
    use Orientation::*;

    match ori {
        Up(false)    => original,
        Left(false)  => rotate_img(original),
        Down(false)  => rotate_img(rotate_img(original)),
        Right(false) => rotate_img(rotate_img(rotate_img(original))),
        Up(true)     => rotate_img(rotate_img(flip_img(original))),
        Left(true)   => rotate_img(rotate_img(rotate_img(flip_img(original)))),
        Down(true)   => flip_img(original),
        Right(true)  => rotate_img(flip_img(original)),
    }
}

// rotate 90 degrees ccw, keep the bit order. could also store all ccw and do flips in comparisons
fn rotate(tile: Tile) -> Tile {
    Tile {
        name: tile.name,
        // top, bottom, left, right; bits left to right, top to bottom
        borders: (tile.borders.3, tile.borders.2, flipbits(tile.borders.0), flipbits(tile.borders.1)),
        orientation: rot_orientation(tile.orientation),
    }
}

// along x axis: top and bottom swap, left and right are mirrored
fn flipx(tile: Tile) -> Tile {
    Tile {
        name: tile.name,
        borders: (tile.borders.1, tile.borders.0, flipbits(tile.borders.2), flipbits(tile.borders.3)),
        orientation: flip_orientation(tile.orientation),
    }
}

fn search(current_state: State, remaining_tiles: Vec<Tile>) -> Option<State> {
    if false {
        println!("---");
        for y in 0..current_state.dim {
            for x in 0..current_state.dim {
                if let Some(tile) = current_state.at(x, y) {
                    print!("{}    ", tile.name);
                } else {
                    print!("....    ");
                }
            }
            println!();
        }
    }
    if remaining_tiles.is_empty() {
        // all consumed, this is a valid solution
        return Some(current_state);
    }

    // if remaining tiles, the map also has equivalent number of remaining open slots
    let nextpos = current_state.map.iter().position(|x| x.is_none()).unwrap();

    let run_search = |tile_ix: usize, tile: Tile| {
        if current_state.accepts(nextpos, &tile) {
            let mut next_state = current_state.clone();
            let mut next_tiles = remaining_tiles.clone();
            next_state.map[nextpos] = Some(tile);
            next_tiles.remove(tile_ix);
            search(next_state, next_tiles)
        } else {
            None
        }
    };

    for (tile_ix, &tile) in remaining_tiles.iter().enumerate() {
        for &t1 in &[tile, flipx(tile)] {
            for &t2 in &[t1, rotate(t1), rotate(rotate(t1)), rotate(rotate(rotate(t1)))] {
                let s = run_search(tile_ix, t2);
                if s.is_some() {
                    // many solutions could exist due to symmetry, but any of them is acceptable
                    // because they're equivalent so pick the first when one is found
                    return s;
                }
            }
        }
    }

    None
}

type Sea = Vec<u128>;
/* epic sea monster
 *                    98765432109876543210
 *                                      #
 *                    #    ##    ##    ###
 *                     #  #  #  #  #  #
 */
const MONS0: u128 = 0b00000000000000000010;
const MONS1: u128 = 0b10000110000110000111;
const MONS2: u128 = 0b01001001001001001000;
const MONS_LEN: usize = 20; // bits

fn monster_x_position(a: u128, b: u128, c: u128, x: usize) -> Option<usize> {
    for shift in x..=(128 - MONS_LEN) {
        let abits = (a >> shift) & MONS0;
        let bbits = (b >> shift) & MONS1;
        let cbits = (c >> shift) & MONS2;
        if abits == MONS0 && bbits == MONS1 && cbits == MONS2 {
            return Some(shift);
        }
    }
    None
}

fn sea_monsters(sea: &Sea) -> Vec<(usize, usize)> {
    // can the monsters overlap? Not specified, hopefully it doesn't matter
    let mut mons = Vec::new();
    for (y, rows) in sea.windows(3).enumerate() {
        let mut x0 = 0;
        while let Some(shift) = monster_x_position(rows[0], rows[1], rows[2], x0) {
            mons.push((shift, y));
            x0 = shift + 1;
        }
    }
    mons
}

fn flip_sea(sea: &Sea) -> Sea {
    sea.iter().rev().copied().collect()
}

fn sea_column(sea: &Sea, col: usize) -> u128 {
    sea.iter().fold(0, |dst, srcrow| (dst << 1) | ((srcrow >> col) & 1))
}

fn rotate_sea(sea: &Sea) -> Sea {
    let mut out = Vec::new();
    for y in 0..128 {
        out.push(sea_column(sea, y));
    }
    out
}

fn dump_sea(sea: &Sea) {
    for row in sea.iter() {
        for c in (0..128).rev() {
            print!("{}", if (row & (1 << c)) != 0 { '#' } else { '.' });
        }
        println!();
    }
}

fn water_roughness(sea: &Sea) -> usize {
    let mut seas = [
        sea.clone(),
        rotate_sea(sea),
        rotate_sea(&rotate_sea(sea)),
        rotate_sea(&rotate_sea(&rotate_sea(sea))),
        flip_sea(sea),
        rotate_sea(&flip_sea(sea)),
        rotate_sea(&rotate_sea(&flip_sea(sea))),
        rotate_sea(&rotate_sea(&rotate_sea(&flip_sea(sea)))),
    ];
    let monster_locations: Vec<Vec<_>> = seas.iter().map(sea_monsters).collect();
    assert!(monster_locations.iter().filter(|x| !x.is_empty()).count() == 1);

    let (sea, monsters): (&mut Sea, &Vec<_>) = seas.iter_mut().zip(monster_locations.iter())
        .find(|(_s, m)| !m.is_empty()).unwrap();

    let initial_roughness: usize = sea.iter().map(|waves| waves.count_ones() as usize).sum();
    println!("rouff with monsters {}, {} total", initial_roughness, monsters.len());

    if false {
        dump_sea(sea);
        println!();
    }

    let monster_weight = (MONS0.count_ones() + MONS1.count_ones() + MONS2.count_ones()) as usize;
    println!("quick check: {}", initial_roughness - monsters.len() * monster_weight);

    for (y, row) in sea.iter().enumerate() {
        for c in (0..128).rev() {
            let m = monsters.iter().any(|&(ms, my)| {
                let m0 = y == my     && ((MONS0 << ms) & (1 << c)) != 0;
                let m1 = y == my + 1 && ((MONS1 << ms) & (1 << c)) != 0;
                let m2 = y == my + 2 && ((MONS2 << ms) & (1 << c)) != 0;
                m0 || m1 || m2
            });
            if m {
                print!("O");
            } else {
                print!("{}", if (row & (1 << c)) != 0 { '#' } else { '.' });
            }
        }
        println!();
    }
    println!();

    // if any monsters overlap, this could be more reliable than the quick estimate
    for &(ms, my) in monsters.iter() {
        sea[my    ] &= !(MONS0 << ms);
        sea[my + 1] &= !(MONS1 << ms);
        sea[my + 2] &= !(MONS2 << ms);
    }
    sea.iter().map(|waves| waves.count_ones() as usize).sum()
}

fn form_actual_image(tilemap: &HashMap<u16, &(Tile, Image)>, state: &State) -> Sea {
    let mut sea: Sea = vec![0; state.dim * IMAGEDIM];
    for y in 0..state.dim {
        for x in 0..state.dim {
            let tile = state.at(x, y).unwrap();
            let img = orient_image(tilemap[&tile.name].1, tile.orientation);
            for (rowi, &rowbits) in img.iter().enumerate() {
                sea[y * IMAGEDIM + rowi] |= (rowbits as u128) << ((state.dim - 1 - x) * IMAGEDIM);
            }
        }
    }
    sea
}

// MSB is left or top
fn parse_tile(input: &[String]) -> (Tile, Image) {
    let name = input[0].strip_prefix("Tile ").unwrap().strip_suffix(":").unwrap().parse().unwrap();
    let top = input[1].as_bytes().iter().fold(0, |bits, &ch| {
        (bits << 1) | ((ch == b'#') as u16)
    });
    let bottom = input.last().unwrap().as_bytes().iter().fold(0, |bits, &ch| {
        (bits << 1) | ((ch == b'#') as u16)
    });
    let left = input[1..].iter().fold(0, |bits, row| {
        (bits << 1) | ((*row.as_bytes().first().unwrap() == b'#') as u16)
    });
    let right = input[1..].iter().fold(0, |bits, row| {
        (bits << 1) | ((*row.as_bytes().last().unwrap() == b'#') as u16)
    });
    let borders = (top, bottom, left, right);
    let mut image = [0; IMAGEDIM];
    for (srcstr, dstbits) in input[2..].iter().zip(image.iter_mut()) {
        *dstbits = srcstr.as_bytes()[1..(1+IMAGEDIM)].iter().fold(0, |bits, &ch| {
            (bits << 1) | ((ch == b'#') as u16)
        });
    }
    (Tile { name, borders, orientation: Orientation::Up(false) }, image.clone())
}

// note: my input has 144 tiles - that would be 12*12, or 9*16, or 8*18, or 6*24, etc. but the
// specs hint that the final arrangement will be square
fn main() {
    assert!(flipbits(1) == (1 << 9));
    assert!(flipbits(1 << 9) == (1));
    assert!(flipbits(0x1e) == (0x0f << 5));
    assert!(flipbits(0x0f << 5) == (0x1e));

    let lines: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();

    let tiles: Vec<(Tile, Image)> = lines.split(|line| line == "").map(parse_tile).collect();

    // assume the image is a square
    let dim = (tiles.len() as f64).sqrt() as usize;

    let puzzle_tiles: Vec<Tile> = tiles.iter().map(|(t, _i)| *t).collect();
    let state = search(State::new(dim), puzzle_tiles).unwrap();

    for y in 0..dim {
        for x in 0..dim {
            print!("{}    ", state.at(x, y).unwrap().name);
        }
        println!();
    }

    let corners = [
        state.at(0,             0).unwrap().name as u64,
        state.at(dim - 1,       0).unwrap().name as u64,
        state.at(0,       dim - 1).unwrap().name as u64,
        state.at(dim - 1, dim - 1).unwrap().name as u64,
    ];
    println!("{}", corners[0] * corners[1] * corners[2] * corners[3]);

    // indexed by name for easier lookup
    let tilemap: HashMap<u16, &(Tile, Image)> = tiles.iter().map(|ti| {
        (ti.0.name, ti)
    }).collect();

    let sea = form_actual_image(&tilemap, &state);
    println!("{}", water_roughness(&sea));
}
