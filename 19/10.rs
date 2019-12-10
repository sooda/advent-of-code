use std::io::{self, BufRead};

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

fn main() {
    // can't flat_map directly because those line strings are temporary, so two collect()s :(
    let roids: Vec<_> = io::stdin().lock().lines().enumerate().map(
        |(y, line)| line.unwrap().bytes().enumerate().filter_map(
            |(x, ch)| if ch == b'#' { Some((x as i32, y as i32)) } else { None }
        ).collect::<Vec<_>>()
    )
    .flat_map(|line| line.into_iter()).collect();

    println!("{:?}", roids);
    println!("n={}", roids.len());

    let (bestval, bestpos) = roids.iter().map(|roid| (visibility(roid, &roids), roid)).max().unwrap();
    println!("best {} at {:?}", bestval, bestpos);
}
