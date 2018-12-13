use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn parse(line: &str) -> (i32, i32) {
    let mut sp = line.split(", ");
    (sp.next().unwrap().parse().unwrap(), sp.next().unwrap().parse().unwrap())
}

fn distance(place: (i32, i32), x: i32, y: i32) -> i32 {
    (place.0 - x).abs() + (place.1 - y).abs()
}

fn closest_coord(coords: &[(i32, i32)], x: i32, y: i32) -> Option<usize> {
    let mut dists = coords.iter().enumerate()
        .map(|(i, &c)| (distance(c, x, y), i)).collect::<Vec<_>>();
    dists.sort_unstable();
    if dists[0].0 == dists[1].0 {
        // same distances? not counted
        None
    } else {
        // unique? return the index
        Some(dists[0].1)
    }
}

fn render(coords: &[(i32, i32)], map: &mut [usize], x0: i32, y0: i32, w: i32, h: i32) -> Vec<usize> {
    let mut counts = vec![0; coords.len()];
    for y in 0..h {
        for x in 0..w {
            // equally far from two or more are not included in the sum
            if let Some(c_id) = closest_coord(coords, x0 + x, y0 + y) {
                map[(y * w + x) as usize] = c_id;
                counts[c_id] += 1;
            }
        }
    }
    counts
}

fn friendly_region(coords: &[(i32, i32)], x0: i32, y0: i32, w: i32, h: i32) -> i32 {
    let mut count = 0;
    for y in 0..h {
        for x in 0..w {
            let dist_sum = coords.iter().map(|&c| distance(c, x0 + x, y0 + y)).sum::<i32>();
            if dist_sum < 10000 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let coords = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse(&x.unwrap())).collect::<Vec<_>>();
    let x0 = coords.iter().map(|c| c.0).min().unwrap();
    let x1 = coords.iter().map(|c| c.0).max().unwrap();
    let y0 = coords.iter().map(|c| c.1).min().unwrap();
    let y1 = coords.iter().map(|c| c.1).max().unwrap();
    println!("{} {} {} {}", x0, x1, y0, y1);
    let w = x1 - x0 + 1;
    let h = y1 - y0 + 1;
    let mut map = vec![0; (w * h) as usize];
    let counts = render(&coords, &mut map, x0, y0, w, h);
    println!("{:?}", counts);
    // hmm, cheating: happened to get the first part right even without checking whether this area
    // is infinite (i.e., on the border of the map just right)
    println!("{}", counts.iter().max().unwrap());
    // the region is probably nicely contiguous
    let blotch = friendly_region(&coords, x0, y0, w, h);
    println!("{}", blotch);
}
