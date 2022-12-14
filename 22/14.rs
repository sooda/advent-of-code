use std::io::{self, BufRead};

// x0, x1, y0, y1 (inclusive)
type Vein = (usize, usize, usize, usize);

fn parse_line(input: &str) -> Vec<Vein> {
    /*
     * 498,4 -> 498,6 -> 496,6
     * 503,4 -> 502,4 -> 502,9 -> 494,9
     */
    let parts = input.split(" -> ").map(|p| {
        let mut sp = p.split(',').map(|n| n.parse::<usize>().unwrap());
        (sp.next().unwrap(), sp.next().unwrap())
    });
    let parts2 = parts.clone().skip(1);
    parts.zip(parts2).map(|(a, b)| {
        (a.0.min(b.0), a.0.max(b.0), a.1.min(b.1), a.1.max(b.1))
    }).collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LandSquare {
    Air,
    Rock,
    Sand,
    Channel, // recycled from 2018 day 17, useful for visualization
}
use LandSquare::*;

#[derive(Clone)]
struct Ground {
    map: Vec<LandSquare>,
    w: usize,
    h: usize,
    pour_x: usize,
}

impl Ground {
    fn at(&self, x: usize, y: usize) -> LandSquare {
        assert!(x < self.w);
        assert!(y < self.h);

        self.map[y * self.w + x]
    }

    fn put(&mut self, x: usize, y: usize, sq: LandSquare) {
        assert!(x < self.w);
        assert!(y < self.h);

        self.map[y * self.w + x] = sq;
    }
}

fn dunp(g: &Ground) {
    for y in 0..g.h {
        for x in 0..g.w {
            print!("{}", match g.at(x, y) {
                Rock => '#',
                Air => '.',
                Sand => 'o',
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
                Rock => "0 0 0", // white
                Air => "1 1 1", // black
                Sand => "0 0 1", // blue
                Channel => "0 1 1", // cyan
            });
        }
        println!("");
    }
}

fn mapscan(veins: &[Vein]) -> Ground {
    let pour_coord = 500;
    let miny = 0;
    let maxy = veins.iter().map(|v| v.2.max(v.3)).max().unwrap();
    let border = 2;
    let minx = pour_coord - maxy - border;
    let maxx = pour_coord + maxy + border;
    let w = maxx - minx + 1;
    let h = maxy - miny + 1 + border;
    let pour_x = pour_coord - minx;

    let mut map = vec![Air; w * h];
    for v in veins {
        // either y or x stays constant
        for y in v.2..=v.3 {
            for x in v.0..=v.1 {
                map[(y - miny) * w + (x - minx + 0)] = Rock;
            }
        }
    }

    Ground { map, w, h, pour_x }
}

#[derive(Debug, PartialEq)]
enum SandPlacement {
    Stopped,
    Exit, // out of this world
}
use SandPlacement::*;

fn flowable(g: &Ground, x: usize, y: usize) -> Option<usize> {
    for &xnew in &[x, x - 1, x + 1] {
        let cell = g.at(xnew, y + 1);
        if cell == Air || cell == Channel {
            return Some(xnew);
        }
    }
    None
}

fn droplet(g: &mut Ground, mut x: usize, mut y: usize) -> SandPlacement {
    loop {
        g.put(x, y, Channel);
        if y == g.h - 1 {
            // fell off
            return Exit;
        } else {
            if let Some(xnew) = flowable(g, x, y) {
                x = xnew;
                y += 1;
            } else {
                break;
            }
        }
    }
    g.put(x, y, Sand);

    if (x, y) == (g.pour_x, 0) {
        // no more space to fall, this is the last grain
        Exit
    } else {
        Stopped
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
    // should equal to i though
    let sand = g.map.iter().filter(|&&x| x == Sand).count();
    sand
}

fn main() {
    let veins: Vec<_> = io::stdin().lock().lines()
        .flat_map(|line| parse_line(&line.unwrap()).into_iter())
        .collect();

    let mut ground = mapscan(&veins);
    let mut ground_with_floor = ground.clone();
    for x in 0..ground.w {
        ground_with_floor.put(x, ground.h - 1, Rock);
    }
    if false {
        dunp(&ground);
        dunp(&ground_with_floor);
    }
    let score = pour(&mut ground);
    let floor_score = pour(&mut ground_with_floor);
    if false {
        dunp(&ground);
    }
    println!("{:?}", score);
    println!("{:?}", floor_score);
    if false {
        dump_ppm(&ground);
    }
}
