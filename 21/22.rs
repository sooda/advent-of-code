use std::io::{self, BufRead};
use std::collections::HashSet;

type Region = (bool, (i32, i32), (i32, i32), (i32, i32));
type Coord = (i32, i32, i32);

fn mutate(space: &mut HashSet::<Coord>, region: &Region) {
    (region.1.0..=region.1.1).flat_map(|x| {
        (region.2.0..=region.2.1).flat_map(move |y| {
            (region.3.0..=region.3.1).map(move |z| {
                (x, y, z)
            })
        })
    }).for_each(|c| {
        if region.0 {
            space.insert(c);
        } else {
            space.remove(&c);
        }
    });
}

fn execute_steps(regions: &[Region]) -> usize {
    let mut space = HashSet::new();
    let coords_ok = |(a, b)| a >= -50 && b <= 50;
    for r in regions.iter().filter(|r| coords_ok(r.1) && coords_ok(r.2) && coords_ok(r.3)) {
        mutate(&mut space, r);
    }
    space.len()
}

fn parse_region(line: &str) -> Region {
    let region = |s: &str| -> (i32, i32) {
        let mut sp = s.split("..");
        (sp.next().unwrap().parse().unwrap(), sp.next().unwrap().parse().unwrap())
    };
    let mut sp = line.split(" x=");
    let onoff = sp.next().unwrap() == "on";
    let mut sp = sp.next().unwrap().split(",y=");
    let xregion = region(sp.next().unwrap());
    let mut sp = sp.next().unwrap().split(",z=");
    let yregion = region(sp.next().unwrap());
    let zregion = region(sp.next().unwrap());
    (onoff, xregion, yregion, zregion)
}

fn main() {
    let regions: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_region(&line.unwrap()))
        .collect();
    println!("{:?}", execute_steps(&regions));
}
