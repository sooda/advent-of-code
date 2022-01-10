use std::io::{self, BufRead};
use std::collections::HashSet;

// six in a tuple would get too indexy
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cuboid {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Cuboid {
    fn from_inclusive(c: &Cuboid) -> Cuboid {
        Cuboid {
            x: (c.x.0, c.x.1 + 1),
            y: (c.y.0, c.y.1 + 1),
            z: (c.z.0, c.z.1 + 1),
        }
    }
    // FIXME consider this in the csg stuff, maybe empty should be max > min:
    // in order to be able to represent empty volumes, the end is considered to be exclusive; so
    // (a, a) is an empty range and (a, a+1) includes only a. (Not in the first toy puzzle.)
    fn volume(&self) -> i64 {
        // (or x.1 - x.0 + 1 if inclusive)
        (self.x.1 - self.x.0) *
            (self.y.1 - self.y.0) *
            (self.z.1 - self.z.0)
    }
    // may produce nonphysical cuboids if these do not intersect
    fn intersect_raw(&self, other: &Cuboid) -> Cuboid {
        let x = (self.x.0.max(other.x.0), self.x.1.min(other.x.1));
        let y = (self.y.0.max(other.y.0), self.y.1.min(other.y.1));
        let z = (self.z.0.max(other.z.0), self.z.1.min(other.z.1));
        Cuboid { x, y, z }
    }

    // volume is at least one in each direction?
    fn is_physical(&self) -> bool {
        // or <= if inclusive
        self.x.0 < self.x.1 && self.y.0 < self.y.1 && self.z.0 < self.z.1
    }

    fn try_intersect(&self, other: &Cuboid) -> Option<Cuboid> {
        let c = self.intersect_raw(other);
        if c.is_physical() {
            Some(c)
        } else {
            None
        }
    }

    fn intersect3_raw(a: &Cuboid, b: &Cuboid, c: &Cuboid) -> Cuboid {
        a.intersect_raw(&b.intersect_raw(c))
    }

    // or other.x.0 - 1 if inclusive coords
    fn left_part_of(&self, other: &Cuboid) -> Cuboid {
        Cuboid { x: (self.x.0, other.x.0), y: self.y, z: self.z }
    }
    // or other.x.0 + 1 if inclusive coords
    fn right_part_of(&self, other: &Cuboid) -> Cuboid {
        Cuboid { x: (other.x.1, self.x.1), y: self.y, z: self.z }
    }
    fn bottom_part_of(&self, other: &Cuboid) -> Cuboid {
        Cuboid { x: self.x, y: (self.y.0, other.y.0), z: self.z }
    }
    fn top_part_of(&self, other: &Cuboid) -> Cuboid {
        Cuboid { x: self.x, y: (other.y.1, self.y.1), z: self.z }
    }
    fn front_part_of(&self, other: &Cuboid) -> Cuboid {
        Cuboid { x: self.x, y: self.y, z: (self.z.0, other.z.0) }
    }
    fn back_part_of(&self, other: &Cuboid) -> Cuboid {
        Cuboid { x: self.x, y: self.y, z: (other.z.1, self.z.1) }
    }
    fn middle_x_part_of(&self, other: &Cuboid) -> Cuboid {
        Cuboid { x: other.x, y: self.y, z: self.z }
    }
    fn middle_y_part_of(&self, other: &Cuboid) -> Cuboid {
        Cuboid { x: self.x, y: other.y, z: self.z }
    }
    fn middle_z_part_of(&self, other: &Cuboid) -> Cuboid {
        Cuboid { x: self.x, y: self.y, z: other.z }
    }
}

type Region = (bool, Cuboid);
type Coord = (i64, i64, i64);

fn mutate(space: &mut HashSet::<Coord>, region: &Region) {
    let cub = &region.1;
    (cub.x.0..=cub.x.1).flat_map(|x| {
        (cub.y.0..=cub.y.1).flat_map(move |y| {
            (cub.z.0..=cub.z.1).map(move |z| {
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

fn bruteforce_steps(regions: &[Region]) -> usize {
    let mut space = HashSet::new();
    let coords_ok = |(a, b)| a >= -50 && b <= 50;
    for r in regions.iter().filter(|r| coords_ok(r.1.x) && coords_ok(r.1.y) && coords_ok(r.1.z)) {
        mutate(&mut space, r);
    }
    space.len()
}

// fist may punch a hole to the blob; divide blob into 3*3*3 regions (of which some may connect to
// each other directly, but they still become separate for generality). Assuming that you'd punch a
// dimple to a cuboid in front of you, perhaps all the way through, there is:
// * something above the fist, or none if the fist scratches the top
// * similarly for left, right and bottom scratching
// * something behind the fist, or none if the fist goes all the way through
// * something in front of the fist if the punch starts inside the blob (similarly to punching a
//   blob that is behind you)
// * the fist may also
//   * just visit the inside so there's something all over it
//   * scratch multiple edges so there's nothing in several sides of it
//   * be equal to the blob entirely so it consumes the entire blob
// Note that the fist does not extend outside the blob; it's the intersection of this blob and
// something else, so any face of it is either inside the volume or lies on a face exactly.
// The union of blob and fist would be the larger of these exactly.
// The intersection of blob and fist would be smaller of these exactly.
// Regarding one dimension at a time, the fist cuts the front, the back, or all the way.
#[allow(unused_variables)]
fn csg_sub(blob: &Cuboid, fist: &Cuboid) -> Vec<Cuboid> {
    // any of these might be an empty set
    let xleft = blob.left_part_of(fist);
    let xright = blob.right_part_of(fist);
    let ybottom = blob.bottom_part_of(fist);
    let ytop = blob.top_part_of(fist);
    let zfront = blob.front_part_of(fist);
    let zback = blob.back_part_of(fist);
    // these aren't empty because we intersect
    let xmid = blob.middle_x_part_of(fist);
    let ymid = blob.middle_y_part_of(fist);
    let zmid = blob.middle_z_part_of(fist);

    // view from the top: y doesn't change

    // corners: left to right, front to back
    let top_front_left = Cuboid::intersect3_raw(&ytop, &xleft, &zfront);
    let top_front_right = Cuboid::intersect3_raw(&ytop, &xright, &zfront);
    let top_back_left = Cuboid::intersect3_raw(&ytop, &xleft, &zback);
    let top_back_right = Cuboid::intersect3_raw(&ytop, &xright, &zback);
    // middles: nearest, farthest, left, right
    let top_front_mid = Cuboid::intersect3_raw(&ytop, &xmid, &zfront);
    let top_back_mid = Cuboid::intersect3_raw(&ytop, &xmid, &zback);
    let top_left_mid = Cuboid::intersect3_raw(&ytop, &xleft, &zmid);
    let top_right_mid = Cuboid::intersect3_raw(&ytop, &xright, &zmid);
    // this might very well be empty
    let top_mid_mid = Cuboid::intersect3_raw(&ytop, &xmid, &zmid);

    // now would be nice to be able to rotate the coordinate system (= swap two axes) appropriately
    // and do this for the other six faces easily because of symmetry. oh well... let's swap the
    // axes carefully by hand.

    // view from the front: z doesn't change.
    // like top but front is the new top and y is the new z

    // corners
    let front_bottom_left = Cuboid::intersect3_raw(&zfront, &xleft, &ybottom);
    let front_bottom_right = Cuboid::intersect3_raw(&zfront, &xright, &ybottom);
    let front_top_left = Cuboid::intersect3_raw(&zfront, &xleft, &ytop);
    let front_top_right = Cuboid::intersect3_raw(&zfront, &xright, &ytop);
    // middles
    let front_bottom_mid = Cuboid::intersect3_raw(&zfront, &xmid, &ybottom);
    let front_top_mid = Cuboid::intersect3_raw(&zfront, &xmid, &ytop);
    let front_left_mid = Cuboid::intersect3_raw(&zfront, &xleft, &ymid);
    let front_right_mid = Cuboid::intersect3_raw(&zfront, &xright, &ymid);
    // this might very well be empty
    let front_mid_mid = Cuboid::intersect3_raw(&zfront, &xmid, &ymid);

    // view from the left: x doesn't change.
    // like top but uhhhh

    // corners
    let left_bottom_back = Cuboid::intersect3_raw(&xleft, &zback, &ybottom);
    let left_bottom_front = Cuboid::intersect3_raw(&xleft, &zfront, &ybottom);
    let left_top_back = Cuboid::intersect3_raw(&xleft, &zback, &ytop);
    let left_top_front = Cuboid::intersect3_raw(&xleft, &zfront, &ytop);
    // middles
    let left_bottom_mid = Cuboid::intersect3_raw(&xleft, &zmid, &ybottom);
    let left_top_mid = Cuboid::intersect3_raw(&xleft, &zmid, &ytop);
    let left_back_mid = Cuboid::intersect3_raw(&xleft, &zback, &ymid);
    let left_front_mid = Cuboid::intersect3_raw(&xleft, &zfront, &ymid);
    // this might very well be empty
    let left_mid_mid = Cuboid::intersect3_raw(&xleft, &zmid, &ymid);

    // the same three but their mirror faces...

    // view from top through the bottom: y doesn't change

    // corners: left to right, front to back
    let bot_front_left = Cuboid::intersect3_raw(&ybottom, &xleft, &zfront);
    let bot_front_right = Cuboid::intersect3_raw(&ybottom, &xright, &zfront);
    let bot_back_left = Cuboid::intersect3_raw(&ybottom, &xleft, &zback);
    let bot_back_right = Cuboid::intersect3_raw(&ybottom, &xright, &zback);
    // middles: nearest, farthest, left, right
    let bot_front_mid = Cuboid::intersect3_raw(&ybottom, &xmid, &zfront);
    let bot_back_mid = Cuboid::intersect3_raw(&ybottom, &xmid, &zback);
    let bot_left_mid = Cuboid::intersect3_raw(&ybottom, &xleft, &zmid);
    let bot_right_mid = Cuboid::intersect3_raw(&ybottom, &xright, &zmid);
    // this might very well be empty
    let bot_mid_mid = Cuboid::intersect3_raw(&ybottom, &xmid, &zmid);

    // view from the front through the back: z doesn't change.

    // corners
    let back_bottom_left = Cuboid::intersect3_raw(&zback, &xleft, &ybottom);
    let back_bottom_right = Cuboid::intersect3_raw(&zback, &xright, &ybottom);
    let back_top_left = Cuboid::intersect3_raw(&zback, &xleft, &ytop);
    let back_top_right = Cuboid::intersect3_raw(&zback, &xright, &ytop);
    // middles
    let back_bottom_mid = Cuboid::intersect3_raw(&zback, &xmid, &ybottom);
    let back_top_mid = Cuboid::intersect3_raw(&zback, &xmid, &ytop);
    let back_left_mid = Cuboid::intersect3_raw(&zback, &xleft, &ymid);
    let back_right_mid = Cuboid::intersect3_raw(&zback, &xright, &ymid);
    // this might very well be empty
    let back_mid_mid = Cuboid::intersect3_raw(&zback, &xmid, &ymid);

    // view from the left through the right: x doesn't change.

    // corners
    let right_bottom_back = Cuboid::intersect3_raw(&xright, &zback, &ybottom);
    let right_bottom_front = Cuboid::intersect3_raw(&xright, &zfront, &ybottom);
    let right_top_back = Cuboid::intersect3_raw(&xright, &zback, &ytop);
    let right_top_front = Cuboid::intersect3_raw(&xright, &zfront, &ytop);
    // middles
    let right_bottom_mid = Cuboid::intersect3_raw(&xright, &zmid, &ybottom);
    let right_top_mid = Cuboid::intersect3_raw(&xright, &zmid, &ytop);
    let right_back_mid = Cuboid::intersect3_raw(&xright, &zback, &ymid);
    let right_front_mid = Cuboid::intersect3_raw(&xright, &zfront, &ymid);
    // this might very well be empty
    let right_mid_mid = Cuboid::intersect3_raw(&xright, &zmid, &ymid);

    // note: a lot of duplication in the above logic; this shall be cleaned up one day
    [
        top_front_left, top_front_right, top_back_left,
        top_back_right, top_front_mid, top_back_mid,
        top_left_mid, top_right_mid, top_mid_mid,
        bot_front_left, bot_front_right, bot_back_left,
        bot_back_right, bot_front_mid, bot_back_mid,
        bot_left_mid, bot_right_mid, bot_mid_mid,
        front_left_mid, front_right_mid, front_mid_mid,
        back_left_mid, back_right_mid, back_mid_mid,
        left_mid_mid,
        right_mid_mid
    ].iter().cloned().filter(|blob| blob.volume() != 0).collect()
    // wtf cloned, I need intoiterator

    // or could have split by just one axis at a time and then recurse to do the others to limit
    // the explosive combinations. oh well.
}

fn insert_region_into(space: &HashSet::<Cuboid>, next_space: &mut HashSet<Cuboid>, region: Cuboid) {
    for area in space.iter() {
        if let Some(intersection) = region.try_intersect(area) {
            let splits = csg_sub(&region, &intersection);
            // recurse: the original insertion region cannot be used anymore, so try the smaller
            // ones. No need to resplit this one with any other area in this loop.
            for part in splits {
                insert_region_into(space, next_space, part);
            }
            return;
        }
    }
    // nothing cut it anymore
    next_space.insert(region);
}

fn insert_region(space: &HashSet::<Cuboid>, region: Cuboid) -> HashSet::<Cuboid> {
    let mut next_space = space.clone();
    insert_region_into(space, &mut next_space, region);
    next_space
}

fn remove_region(space: &HashSet::<Cuboid>, region: &Cuboid) -> HashSet::<Cuboid> {
    let mut next_space = HashSet::new();
    for area in space.iter() {
        if let Some(intersection) = region.try_intersect(area) {
            let cut_result = csg_sub(&area, &intersection);
            // no need to recurse because new objects are created only within the original blob
            for new_blob in cut_result {
                next_space.insert(new_blob);
            }
        } else {
            next_space.insert(area.clone());
        }
    }
    next_space
}

fn cube_population(space: &HashSet::<Cuboid>) -> i64 {
    space.iter().map(|cub| cub.volume()).sum()
}

fn execute_steps_fast(regions: &[Region], filter_larges: bool) -> i64 {
    let mut space = HashSet::new();
    let coords_ok = if filter_larges {
        |(a, b)| a >= -50 && b <= 50
    } else {
        |_pt| true
    };
    for r in regions.iter().filter(|r| coords_ok(r.1.x) && coords_ok(r.1.y) && coords_ok(r.1.z)) {
        if r.0 {
            space = insert_region(&space, Cuboid::from_inclusive(&r.1));
        } else {
            space = remove_region(&space, &Cuboid::from_inclusive(&r.1));
        }
    }
    cube_population(&space)
}

fn parse_region(line: &str) -> Region {
    let region = |s: &str| -> (i64, i64) {
        let mut sp = s.split("..");
        (sp.next().unwrap().parse().unwrap(), sp.next().unwrap().parse().unwrap())
    };
    let mut sp = line.split(" x=");
    let onoff = sp.next().unwrap() == "on";
    let mut sp = sp.next().unwrap().split(",y=");
    let x = region(sp.next().unwrap());
    let mut sp = sp.next().unwrap().split(",z=");
    let y = region(sp.next().unwrap());
    let z = region(sp.next().unwrap());
    (onoff, Cuboid { x, y, z })
}

fn main() {
    let regions: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_region(&line.unwrap()))
        .collect();
    println!("{:?}", bruteforce_steps(&regions));
    println!("{:?}", execute_steps_fast(&regions, true));
    println!("{:?}", execute_steps_fast(&regions, false));
}
