use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

type Point = (i32, i32, i32, i32);
type Constellation = Vec<Point>;

fn parse_line(input: &str) -> Point {
    // 2,5,-4,-7
    let mut coords = input.split(",");
    let x = coords.next().unwrap().parse().unwrap();
    let y = coords.next().unwrap().parse().unwrap();
    let z = coords.next().unwrap().parse().unwrap();
    let w = coords.next().unwrap().parse().unwrap();
    (x, y, z, w)
}

fn pt_distance(a: &Point, b: &Point) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() + (a.3 - b.3).abs()
}

fn distance(cons: &Constellation, pt: &Point) -> i32 {
    cons.iter().map(|cp| pt_distance(cp, pt)).min().unwrap()
}

fn close_enough(cons: &Constellation, pt: &Point) -> bool {
    distance(cons, pt) <= 3
}

// Find one that can be removed. Removing inside the loop is not possible so we're finding one
// first and doing the removal and modification in the other function, and ultimately joining the
// result of possibly several joins in a row back to the list when no more joins are possible.
fn nearby_constellation(constellations: &mut Vec<Constellation>, joining_pt: Point) -> Option<usize> {
    // seems that a manual loop with "if close enough, return Some(i)" would be faster, but this is
    // a learning experiment and the runtime is just some milliseconds
    constellations.iter().enumerate()
        .find(|&(_i, cons)| close_enough(cons, &joining_pt))
        .map(|(i, _closest_cons)| i)
}

// Attempt to join other to some already formed one by using joining_pt in other as the possibly
// closest point. If a join happened, the found one is removed from the list and returned.
// Otherwise nothing is changed.
fn join_constellation(constellations: &mut Vec<Constellation>, other: &mut Constellation, joining_pt: Point)
        -> Option<Constellation> {
    match nearby_constellation(constellations, joining_pt) {
        Some(i) => {
            let mut dest = constellations.remove(i);
            dest.append(other);
            Some(dest)
        },
        None => None
    }
}

fn constellation_count(world: &mut Constellation) -> usize {
    let mut constellations = vec![];

    // For each new point, it either becomes a lone constellation of just itself or joins an
    // existing constellation. It can be close enough to multiple currently separate constellations
    // though, so pick the found constellation that the point joined (by first being a trivial
    // single-pt constellation that joined another) and keep trying to find another that's now
    // close enough to the bigger constellation due to the new point being part of it.
    while let Some(new_pt) = world.pop() {
        let mut join_candidate = vec![new_pt];
        loop {
            if let Some(joined) = join_constellation(&mut constellations, &mut join_candidate, new_pt) {
                join_candidate = joined;
            } else {
                break;
            }
        }

        // No more joins, so make this a separate constellation. Either we got the original trivial
        // constellation of this new point or extracted out at least one from the list of currently
        // formed constellations. The extracted one got modified a bit; add it back now.
        assert!(!join_candidate.is_empty());
        constellations.push(join_candidate);
    }

    constellations.len()
}

fn main() {
    let mut points = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|l| parse_line(&l.unwrap())).collect::<Vec<_>>();
    println!("{:?}", constellation_count(&mut points));
}
