use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// x0, x1, y0, y1 (inclusive)
fn parse_line(input: &str) -> (usize, usize, usize, usize) {
    /*
     * x=504, y=333..358
     * y=102, x=587..593
     */
    // regex might be nicer, but let's do this for a change
    let mut parts = input.split(", ");
    let single_part = parts.next().unwrap();
    let range_part = parts.next().unwrap();

    let coord = single_part.split("=").nth(1).unwrap().parse().unwrap();

    let (r0, r1) = {
        let mut range = range_part.split("=").nth(1).unwrap().split("..");
        (range.next().unwrap().parse().unwrap(), range.next().unwrap().parse().unwrap())
    };

    if single_part.as_bytes()[0] == b'x' {
        (coord, coord, r0, r1)
    } else {
        (r0, r1, coord, coord)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LandSquare {
    Clay,
    Sand,
    Water,
    Channel, // water channel that has dried up
}
use LandSquare::*;


struct Ground {
    map: Vec<LandSquare>,
    w: usize,
    h: usize,
    pour_x: usize,
}

impl Ground {
    fn at(&self, x: usize, y: usize) -> LandSquare {
        // can't go off the grid by spec (extra spaces left and right)
        assert!(x < self.w);
        assert!(y < self.h);

        self.map[y * self.w + x]
    }
    fn fill(&mut self, xl: usize, xr: usize, y: usize, data: LandSquare) {
        assert!(xl < self.w);
        assert!(xr < self.w);

        for x in &mut self.map[(y * self.w + xl)..=(y * self.w + xr)] {
            *x = data;
        }
    }
}

fn dunp(g: &Ground) {
    for y in 0..g.h {
        for x in 0..g.w {
            print!("{}", match g.at(x, y) {
                Clay => '#',
                Sand => '.',
                Water => '~',
                Channel => '|',
            });
        }
        println!("");
    }
    println!("");
}

fn dump_ppm(g: &Ground) {
    println!("P3");
    println!("{} {}", g.w, g.h);
    println!("1");
    for y in 0..g.h {
        for x in 0..g.w {
            print!("{} ", match g.at(x, y) {
                Clay => "0 0 0", // white
                Sand => "1 1 1", // black
                Water => "0 0 1", // blue
                Channel => "0 1 1", // cyan
            });
        }
        println!("");
    }
}

fn mapscan(veins: &[(usize, usize, usize, usize)]) -> Ground {
    let minx = veins.iter().map(|v| v.0.min(v.1)).min().unwrap();
    let maxx = veins.iter().map(|v| v.0.max(v.1)).max().unwrap();
    let miny = veins.iter().map(|v| v.2.min(v.3)).min().unwrap();
    let maxy = veins.iter().map(|v| v.2.max(v.3)).max().unwrap();

    // note! left and right borders are expanded open to let water flow there
    let w = maxx - minx + 1 + 2;
    let h = maxy - miny + 1;
    let pour_x = 500 - minx + 1; // offset for border

    let mut map = vec![Sand; w * h];
    for v in veins {
        for y in v.2..=v.3 {
            for x in v.0..=v.1 {
                map[(y - miny) * w + (x - minx + 1)] = Clay;
            }
        }
    }

    Ground { map: map, w: w, h: h, pour_x: pour_x }
}

#[derive(Debug, PartialEq)]
enum WaterPlacement {
    BounceWall, // horizontal scan found a wall
    FellCliff,  // horizontal scan fell off downwards
    Exit, // out of this world, y == h
}
use WaterPlacement::*;

fn spread(g: &mut Ground, mut x: usize, y: usize, dx: usize) -> (WaterPlacement, usize) {
    if g.at(x, y) != Sand && g.at(x, y) != Channel {
        // this (x,y) is an attempt; backtrack, don't fill here
        return (BounceWall, x - dx);
    }

    loop {
        if g.at(x, y + 1) == Sand || g.at(x, y + 1) == Channel {
            // fall down if can
            return match droplet(g, x, y + 1) {
                FellCliff => (FellCliff, x),
                Exit => (Exit, x),
                BounceWall => (FellCliff, x),
            };
        } else {
            match g.at(x + dx, y) {
                Sand | Channel => {
                    x += dx;
                },
                Clay => {
                    return (BounceWall, x);
                },
                _ => unreachable!()
            }
        }
    }
}

fn left(g: &mut Ground, x: usize, y: usize) -> (WaterPlacement, usize) {
    // should use i32 for these coordinates maybe? this seems nasty
    spread(g, x, y, (-1i32) as usize)
}

fn right(g: &mut Ground, x: usize, y: usize) -> (WaterPlacement, usize) {
    spread(g, x, y, 1)
}

fn droplet(g: &mut Ground, x: usize, mut y: usize) -> WaterPlacement {
    loop {
        g.map[y * g.w + x] = Channel;
        if y == g.h - 1 {
            return Exit;
        } else if g.at(x, y + 1) == Sand {
            y += 1;
        } else if g.at(x, y + 1) == Channel {
            y += 1;
        } else {
            break;
        }
    }
    // cannot fall anymore, so disperse around
    let l = left(g, x - 1, y);
    let r = right(g, x + 1, y);
    match (l, r) {
        ((BounceWall, xl), (BounceWall, xr)) => {
            g.fill(xl, xr, y, Water);
            return BounceWall;
        },
        ((FellCliff, xl), (FellCliff, xr))
            | ((Exit, xl), (FellCliff, xr))
            | ((FellCliff, xl), (Exit, xr))
            | ((BounceWall, xl), (FellCliff, xr))
            | ((FellCliff, xl), (BounceWall, xr)) => {
                g.fill(xl, xr, y, Channel);
                return FellCliff;
        }
        ((Exit, xl), (Exit, xr))
            | ((BounceWall, xl), (Exit, xr))
            | ((Exit, xl), (BounceWall, xr)) => {
                g.fill(xl, xr, y, Channel);
                return Exit;
        }
    }
}

fn pour(g: &mut Ground) -> usize {
    let x = g.pour_x;
    let y = 0;
    let mut i = 0;
    while droplet(g, x, y) != Exit {
        i += 1;
        if false {
            println!("{}", i);
            dunp(g);
        }
    }
    g.map.iter().filter(|&&x| x == Water || x == Channel).count()
}

fn main() {
    let veins = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|l| parse_line(&l.unwrap())).collect::<Vec<_>>();
    let mut ground = mapscan(&veins);
    if false {
        dunp(&ground);
    }
    let score = pour(&mut ground);
    if false {
        dunp(&ground);
    }
    println!("{}", score);
    if false {
        dump_ppm(&ground);
    }
}
