use std::io::{self, BufRead};
use std::collections::HashSet;

type Coord = (i32, i32, i32);

fn csub(a: Coord, b: Coord) -> Coord {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

fn cadd(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
}

fn manhattan_norm(a: Coord) -> i32 {
    a.0.abs() + a.1.abs() + a.2.abs()
}

// a lot of technically possible combinations in total, but mirroring is not allowed
fn orientation_permutations(report: &HashSet<Coord>) -> Vec<HashSet<Coord>> {
    // rotation: ccw across z
    let turn1 = |(x, y, z): (i32, i32, i32)| (-y,  x,  z); // roll left
    let turn2 = |(x, y, z): (i32, i32, i32)| (-x, -y,  z); // roll upside down
    let turn3 = |(x, y, z): (i32, i32, i32)| ( y, -x,  z); // roll right

    // facing direction
    let left  = |(x, y, z): (i32, i32, i32)| ( z,  y, -x); // pan left
    let back  = |(x, y, z): (i32, i32, i32)| (-x,  y, -z); // behind you
    let right = |(x, y, z): (i32, i32, i32)| (-z,  y,  x); // pan right
    let up    = |(x, y, z): (i32, i32, i32)| ( x, -z,  y); // tilt up
    let down  = |(x, y, z): (i32, i32, i32)| ( x,  z, -y); // tilt down
    let permutators: &[Box<dyn Fn((i32, i32, i32)) -> (i32, i32, i32)>] = &[
        Box::new(|p| p),
        Box::new(|p| turn1(p)),
        Box::new(|p| turn2(p)),
        Box::new(|p| turn3(p)),

        Box::new(|p| left(p)),
        Box::new(|p| turn1(left(p))),
        Box::new(|p| turn2(left(p))),
        Box::new(|p| turn3(left(p))),

        Box::new(|p| back(p)),
        Box::new(|p| turn1(back(p))),
        Box::new(|p| turn2(back(p))),
        Box::new(|p| turn3(back(p))),

        Box::new(|p| right(p)),
        Box::new(|p| turn1(right(p))),
        Box::new(|p| turn2(right(p))),
        Box::new(|p| turn3(right(p))),

        Box::new(|p| up(p)),
        Box::new(|p| turn1(up(p))),
        Box::new(|p| turn2(up(p))),
        Box::new(|p| turn3(up(p))),

        Box::new(|p| down(p)),
        Box::new(|p| turn1(down(p))),
        Box::new(|p| turn2(down(p))),
        Box::new(|p| turn3(down(p))),
    ];

    permutators.iter().map(|f| {
        report.iter().map(|&pt| f(pt)).collect()
    }).collect()
}

fn relative_transform(report: &HashSet<Coord>, origin: Coord) -> HashSet<Coord> {
    report.iter().map(|&c| csub(c, origin)).collect()
}

fn try_match(arep: &HashSet<Coord>, brep: &HashSet<Coord>) -> Option<(Coord, HashSet<Coord>)> {
    // TODO: less repetition, cache all the transforms and then intersect all
    let breps = orientation_permutations(brep);
    for &attempted_origin_a in arep {
        let arep_transformed = relative_transform(&arep, attempted_origin_a);
        for brep in &breps {
            for &attempted_origin_b in brep {
                let brep_transformed = relative_transform(&brep, attempted_origin_b);
                let shared_pts = arep_transformed.intersection(&brep_transformed).count();
                if shared_pts >= 12 {
                    let coord_sys_shift = csub(attempted_origin_a, attempted_origin_b);
                    let result = brep_transformed.into_iter().map(|p| cadd(attempted_origin_a, p)).collect();
                    println!("{} pts. b must be {:?} btw", shared_pts, coord_sys_shift);
                    return Some((coord_sys_shift, result));
                }
            }
        }
    }
    None
}

fn find_match(master: &HashSet<Coord>, reports: &Vec<HashSet<Coord>>) -> Option<(usize, (Coord, HashSet<Coord>))> {
    reports.iter()
        .enumerate()
        .filter_map(|(i, report)| {
            try_match(master, report).map(|found| (i, found))
        })
        .next()
}

fn apply_match(master: &HashSet<Coord>, reports: &mut Vec<HashSet<Coord>>) -> Option<(Coord, HashSet<Coord>)> {
    if let Some((i, found)) = find_match(master, reports) {
        reports.remove(i);
        Some(found)
    } else {
        None
    }
}

fn total_beacon_count_and_ocean_span(mut reports: Vec<HashSet<Coord>>) -> (usize, i32) {
    let mut master_reference = reports.remove(0);
    let mut positions = vec![(0, 0, 0)];
    while let Some((shift, found_pts)) = apply_match(&master_reference, &mut reports) {
        positions.push(shift);
        master_reference.extend(found_pts.into_iter());
    }
    assert!(reports.is_empty());
    let ocean_span = positions.iter().flat_map(|&apos| {
        positions.iter().map(move |&bpos| manhattan_norm(csub(apos, bpos)))
    }).max().unwrap();

    (master_reference.len(), ocean_span)
}

fn parse_reports(reports: &[String]) -> Vec<HashSet<Coord>> {
    // --- scanner 0 ---
    // -809,-750,623
    // -853,-746,517
    // ...
    reports
        .split(|l| l == "")
        // ignore the header
        .map(|report| report.iter().skip(1).map(|line| {
            let mut pts = line.split(',');
            (
                pts.next().unwrap().parse().unwrap(),
                pts.next().unwrap().parse().unwrap(),
                pts.next().unwrap().parse().unwrap(),
            )
        }).collect())
        .collect()
}

fn main() {
    // TODO use iterator directly, not vec
    let raw_reports: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let reports = parse_reports(&raw_reports);
    println!("{:?}", total_beacon_count_and_ocean_span(reports));
}
