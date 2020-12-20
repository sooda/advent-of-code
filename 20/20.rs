use std::io::{self, BufRead};

// - top bottom left right, not sure why I didn't make this a struct
// - bits run MSB left to LSB right, MSB top to LSB bottom
// - could also store these in one big u64 for more fun rotations but that's too clever
type Borders = (u16, u16, u16, u16);
#[derive(Debug, Clone, Copy)]
struct Tile {
    name: u16,
    borders: Borders,
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
    for _ in 0..10 {
        out <<= 1;
        out |= bits & 1;
        bits >>= 1;
    }
    out
}

// rotate 90 degrees ccw, keep the bit order. could also store all ccw and do flips in comparisons
fn rotate(tile: Tile) -> Tile {
    Tile {
        name: tile.name,
        // top, bottom, left, right; bits left to right, top to bottom
        borders: (tile.borders.3, tile.borders.2, flipbits(tile.borders.0), flipbits(tile.borders.1))
    }
}

// top and bottom swap, left and right are mirrored
fn flipx(tile: Tile) -> Tile {
    Tile {
        name: tile.name,
        borders: (tile.borders.1, tile.borders.0, flipbits(tile.borders.2), flipbits(tile.borders.3))
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

fn parse_tile(input: &[String]) -> Tile {
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
    Tile { name, borders }
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
    let tiles: Vec<_> = lines.split(|line| line == "").map(parse_tile).collect();

    // assume the image is a square
    let dim = (tiles.len() as f64).sqrt() as usize;

    let state = search(State::new(dim), tiles).unwrap();
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
}
