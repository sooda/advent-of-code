use std::io::{self, BufRead};
use std::collections::HashSet;

const GROWTH: i32 = 2;

type Image = HashSet<(i32, i32)>;

// FIXME the growing image should be a struct that would hold the extents and bg color

fn color(image: &Image, x: i32, y: i32, extents: (i32, i32, i32, i32), bg_color: bool) -> bool {
    let (minx, maxx, miny, maxy) = extents;
    if x < minx || x > maxx || y < miny || y > maxy {
        bg_color
    } else {
        image.contains(&(x, y))
    }
}

fn image_code(image: &Image, x: i32, y: i32, extents: (i32, i32, i32, i32), bg_color: bool) -> usize {
    let coords = [
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y    ), (x, y    ), (x + 1, y    ),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
    ];
    coords.iter().fold(0, |result, coords| {
        result << 1 | (color(image, coords.0, coords.1, extents, bg_color) as usize)
    })
}

fn iterate(image: &Image, algo: &[bool], extents: (i32, i32, i32, i32), bg_color: bool) -> Image {
    let (minx, maxx, miny, maxy) = extents;
    let mut output = Image::new();
    for y in miny-GROWTH..=maxy+GROWTH {
        for x in minx-GROWTH..=maxx+GROWTH {
            if algo[image_code(image, x, y, extents, bg_color)] {
                output.insert((x, y));
            }
        }
    }
    output
}

fn debugdump(image: &Image, extents: (i32, i32, i32, i32)) {
    let (minx, maxx, miny, maxy) = extents;
    for y in miny..=maxy {
        for x in minx..=maxx {
            let ch = if image.contains(&(x, y)) { '#' } else { '.' };
            print!("{}", ch);
        }
        println!();
    }
    println!();
}

fn pixels_after(mut image: Image, algo: &[bool], iterations: usize) -> usize {
    let mut minx = image.iter().map(|(x, _)| *x).min().unwrap();
    let mut maxx = image.iter().map(|(x, _)| *x).max().unwrap();
    let mut miny = image.iter().map(|(_, y)| *y).min().unwrap();
    let mut maxy = image.iter().map(|(_, y)| *y).max().unwrap();
    if false {
        debugdump(&image, (minx, maxx, miny, maxy));
    }
    let mut bg_color = false;
    for _ in 0..iterations {
        image = iterate(&image, algo, (minx, maxx, miny, maxy), bg_color);
        minx -= GROWTH;
        miny -= GROWTH;
        maxx += GROWTH;
        maxy += GROWTH;
        if false {
            debugdump(&image, (minx, maxx, miny, maxy));
        }
        // blink
        if algo[0] && !algo[511] {
            bg_color = !bg_color;
        }
    }
    image.len()
}

fn parse_spec(spec: &[String]) -> (Vec<bool>, Image) {
    let mut sp = spec.split(|l| l == "");
    let algo = sp.next().unwrap().first().unwrap().chars().map(|ch| ch == '#').collect();
    let image = sp.next().unwrap().iter().enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, ch)| {
                if ch == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();
    (algo, image)
}

fn main() {
    // TODO: iterator
    let spec: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let (algo, image) = parse_spec(&spec);
    println!("{:?}", pixels_after(image.clone(), &algo, 2));
    println!("{:?}", pixels_after(image, &algo, 50));
}
