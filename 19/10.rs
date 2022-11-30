use std::io::{self, BufRead};
use std::cmp::Ordering;

fn blocked_i(src: &(i32, i32), dxd: i32, dyd: i32, m: &(i32, i32)) -> bool {
    // dist to "middle" candidate
    let dxm = m.0 - src.0;
    let dym = m.1 - src.1;

    if dxm == 0 {
        // vertical, see if dest is in the same direction and further away than middle
        dxd == 0 && dyd.signum() == dym.signum() && dyd.abs() > dym.abs()
    } else if dym == 0 {
        // horizontal, see if dest is in the same direction and further away than middle
        dyd == 0 && dxd.signum() == dxm.signum() && dxd.abs() > dxm.abs()
    } else if dxd * dym == dyd * dxm {
        // similar triangles in size and orientation, and the middle one must be smaller
        dxd.signum() == dxm.signum() && dyd.signum() == dym.signum()
            && dxd.abs() + dyd.abs() > dxm.abs() + dym.abs()
    } else {
        false
    }
}

fn blocked(src: &(i32, i32), dst: &(i32, i32), map: &Vec<(i32, i32)>) -> bool {
    // dist to destination
    let dxd = dst.0 - src.0;
    let dyd = dst.1 - src.1;

    map.iter().filter(|&x| x != src && x != dst).any(|middle| blocked_i(src, dxd, dyd, middle))
}

fn visibility(src: &(i32, i32), space: &Vec<(i32, i32)>) -> usize {
    space.iter().filter(|&x| x != src).filter(|x| !blocked(src, x, space)).count()
}

fn delta(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    // XXX y goes up for math thinking
    (a.0 - b.0, -(a.1 - b.1))
}

// "atan" (not 2) but for integer stuff, left here as a historical document on what might be done
// in a much larger universe where floating points would not have enough precision
fn _earlier_angle(a: &(i32, i32), b: &(i32, i32)) -> Ordering {
    // both vertically up? return the closer one
    if a.0 == 0 && b.0 == 0 {
        a.1.cmp(&b.1)
    } else if a.0 == 0 {
        Ordering::Less
    } else if b.0 == 0 {
        Ordering::Greater
    } else {
        // NOTE: larger numbers are smaller in angle
        let x = (b.1 * a.0).cmp(&(a.1 * b.0));
        if x != Ordering::Equal {
            x
        } else {
            // closer to origin is better (these coords are positive)
            (a.0 + a.1).cmp(&(b.0 + b.1))
        }
    }
}

// Probably incomplete. This has only been tested but not proved correct.
fn atan3(a: &(i32, i32)) -> f64 {
    use std::f64::consts::PI;
    let q = (a.1 as f64).atan2(a.0 as f64);
    let w = if q >= 0.0 {
        q
    } else {
        q + 2.0 * PI
    };
    // rotate so up becomes 0, keep to 0..2pi
    let e = (w - 0.5 * PI + 2.0 * PI) % (2.0 * PI);
    // inverse so we go clockwise, keep in 0..2pi (for 0 e)
    (2.0 * PI - e) % (2.0 * PI)
}

fn anglecmp(a: &(i32, i32), b: &(i32, i32)) -> Ordering {
    let aa = atan3(a);
    let ab = atan3(b);
    // if same angle, order by distance
    if blocked_i(&(0, 0), b.0, b.1, a) {
        Ordering::Less
    } else if blocked_i(&(0, 0), a.0, a.1, b) {
        Ordering::Greater
    } else if aa < ab {
        Ordering::Less
    } else if aa > ab {
        Ordering::Greater
    } else {
        panic!("{:?} cmp {:?} ??", a, b);
    }
}

fn pew_round(origin: &(i32, i32), roids: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut zapped = Vec::new();

    while !roids.is_empty() {
        roids.sort_by(|a, b| anglecmp(&delta(&a, &origin), &delta(&b, &origin)));
        let rm = roids[0];
        roids.remove(0);
        zapped.push(rm);

        // would be shadowed by rm? leave for the next round, but don't zap
        while !roids.is_empty() && blocked_i(&origin, roids[0].0 - origin.0, roids[0].1 - origin.1, &rm) {
            roids.remove(0);
        }
    }

    zapped
}

fn zap_roid(roids: &mut Vec<(i32, i32)>, victim: &(i32, i32)) {
    let pos = roids.iter().position(|&r| r == *victim).unwrap();
    roids.remove(pos);
}

fn pewpew(origin: &(i32, i32), roids: &mut Vec<(i32, i32)>, limit: usize) -> (i32, i32) {
    zap_roid(roids, origin);
    let mut gone = 0;
    loop {
        let zapped = pew_round(origin, roids);
        if gone + zapped.len() >= limit {
            return zapped[limit - 1 - gone]; // limit is 1-indexed
        }
        gone += zapped.len();
        for z in zapped {
            zap_roid(roids, &z);
        }
    }
}

fn main() {
    let mut roids: Vec<_> = io::stdin().lock().lines().enumerate().flat_map(
        |(y, line)| line.unwrap().into_bytes().into_iter().enumerate().filter_map(
            move |(x, ch)| if ch != b'.' { Some((x as i32, y as i32)) } else { None }
        )
    )
    .collect();

    let (bestval, bestpos) = roids.iter().map(|roid| (visibility(roid, &roids), *roid)).max().unwrap();
    println!("best {} at {:?}", bestval, bestpos);

    let correct_stardust = pewpew(&bestpos, &mut roids, 200);
    println!("{:?} {}", correct_stardust, 100 * correct_stardust.0 + correct_stardust.1);
}
