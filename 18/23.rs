use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64
}

enum Axis {
    X,
    Y,
    Z
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Vec3 { x: x, y: y, z: z }
    }

    fn min(&self, b: Vec3) -> Vec3 {
        Vec3::new(self.x.min(b.x), self.y.min(b.y), self.z.min(b.z))
    }

    fn max(&self, b: Vec3) -> Vec3 {
        Vec3::new(self.x.max(b.x), self.y.max(b.y), self.z.max(b.z))
    }

    fn largest(&self) -> Axis {
        if self.x >= self.y && self.x >= self.z {
            Axis::X
        } else if self.y >= self.z {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    fn manhattan(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn dot(&self, b: Vec3) -> i64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Div<i64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i64) -> Vec3 {
        assert!(rhs != 0);
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

// an octahedron
#[derive(Debug)]
struct Bot {
    p: Vec3,
    r: i64
}

// axis-aligned bounding box for binary space partitioning
#[derive(Debug)]
struct Aabb {
    // both inclusive: { min: -1, max: 1 } covers discrete "cells" -1, 0 and 1 in space
    // min == max would mean a volume of one unit
    min: Vec3,
    max: Vec3
}

impl Bot {
    fn min(&self) -> Vec3 {
        Vec3::new(self.p.x - self.r, self.p.y - self.r, self.p.z - self.r)
    }

    fn max(&self) -> Vec3 {
        Vec3::new(self.p.x + self.r, self.p.y + self.r, self.p.z + self.r)
    }

    // conservative aabb that fully contains this bot
    fn aabb(&self) -> Aabb {
        Aabb::new(self.min(), self.max())
    }

    fn in_range(&self, p: Vec3) -> bool {
        (self.p - p).manhattan() <= self.r
    }
}

impl Aabb {
    fn new(min: Vec3, max: Vec3) -> Aabb {
        Aabb { min: min, max: max }
    }

    fn union(&self, b: &Aabb) -> Aabb {
        Aabb::new(self.min.min(b.min), self.max.max(b.max))
    }

    // amax > bmin  amax < bmin
    // m  M         m  M
    // AAAA         AAAA
    //   BBBB            BBBB
    //   m  M            m  M
    // amin >= bmax  amin < bmax
    fn intersects(&self, b: &Aabb) -> bool {
        self.max.x >= b.min.x
            && self.min.x <= b.max.x
            && self.max.y >= b.min.y
            && self.min.y <= b.max.y
            && self.max.z >= b.min.z
            && self.min.z <= b.max.z
    }

    fn size(&self) -> Vec3 {
        // discrete size: [0, 1] occupies two cells
        self.max - self.min + Vec3::new(1, 1, 1)
    }

    fn center(&self) -> Vec3 {
        // doesn't make sense to do this for unit sizes or smaller in this particular application
        assert!(self.size().manhattan() > 1);
        // Sort of (self.min + self.max) / 2 but round towards negative infinity so that splitting
        // works consistently in both sides of zero. One aabb can be split into [min, center] and
        // [center + 1, max] where min < max in both as long as the split dir has a size of >= 2.
        // For even sizes, both have same size. For odd sizes, center cell becomes part of left.
        //
        // test cases:
        // [10, 11] -> 10 + 1 / 2 = 10, [10, 12] -> 10 + 2 / 2 = 11, [10, 13] -> 10 + 3 / 2 = 11
        // [10, 14] -> 10 + 4 / 2 = 12
        // [0, 1] -> 0 + 1 / 2 = 0, [0, 2] -> 0 + 2 / 2 = 1, [0, 3] -> 0 + 3 / 2 = 1
        // [-2, -1] -> -2 + 1 / 2 = -2, [-2, 0] -> -2 + 2 / 2 = -1, [-2, 1] -> -2 + 3 / 2 = -1
        self.min + (self.max - self.min) / 2
    }

    fn corners(&self) -> [Vec3; 8] {
        [
            // floor
            Vec3::new(self.min.x, self.min.y, self.min.z),
            Vec3::new(self.max.x, self.min.y, self.min.z),
            Vec3::new(self.max.x, self.min.y, self.max.z),
            Vec3::new(self.min.x, self.min.y, self.max.z),
            // ceiling
            Vec3::new(self.min.x, self.max.y, self.min.z),
            Vec3::new(self.max.x, self.max.y, self.min.z),
            Vec3::new(self.max.x, self.max.y, self.max.z),
            Vec3::new(self.min.x, self.max.y, self.max.z),
        ]
    }
}

fn parse_line(input: &str) -> Bot {
    // pos=<-39857152,26545464,51505035>, r=86328482
    let pos = input.split("pos=<").nth(1).unwrap().split(">,").nth(0).unwrap();
    let mut coords = pos.split(",");
    let x = coords.next().unwrap().parse().unwrap();
    let y = coords.next().unwrap().parse().unwrap();
    let z = coords.next().unwrap().parse().unwrap();
    let r = input.split("r=").nth(1).unwrap().parse().unwrap();
    Bot { p: Vec3::new(x, y, z), r: r }
}

// (not used for part two)
fn biggest_contains(bots: &[Bot]) -> usize {
    let biggest = bots.iter().max_by(|&a, &b| a.r.cmp(&b.r)).unwrap();
    bots.iter().filter(|b| biggest.in_range(b.p)).count()
}

// notes:
// - axis-aligned planar projection of aabb is an aligned square
// - axis-aligned planar projection of bot (octahedron) is a 45 degree rotated square
// - if all axis projections overlap, these _might_ touch
// - if no overlap, there is either an axis-aligned line or a 45 degree oriented line on a 2d
//   projection that separates the objects:
//   ----          ----
//   |  |          |  |  ~
//   ----          ---- ~ / \
//  ~~~~~~~    or:     ~ /   \  or: any of mirror similarities
//    /\              ~  \   /
//    \/                  \ /
// NOTE: 45 degree things done in the 3d thing now

fn projection_separates(aabb: &Aabb, bot: &Bot) -> bool {
    // separating axis-aligned plane (one of the aabb faces)?
    let bot_corner_out_box = aabb.min.x > bot.p.x + bot.r // box east
        || aabb.max.x < bot.p.x - bot.r // box west
        || aabb.min.y > bot.p.y + bot.r // box north
        || aabb.max.y < bot.p.y - bot.r; // box south
    if bot_corner_out_box {
        return true;
    }

    if false {
        // separating major-axes-by-45-degrees plane (none of the faces, sort of a bot edge)
        // if a box corner touches or nearly touches a face of the bot, each projection would
        // think it's inside, so these checks aren't enough...
        // --------
        // |      |
        // |      /\
        // |     /| \
        // -----/--X \ <- X at center of bot, box just slightly inside (pic not to scale)
        //      \    /
        //       \  /
        //        \/
        //
        let ne_corner = aabb.max;
        let se_corner = Vec3::new(aabb.max.x, aabb.min.y, 0);
        let sw_corner = aabb.min;
        let nw_corner = Vec3::new(aabb.min.x, aabb.max.y, 0);
        // hmm, wasting some computation for the z coordinates here
        let box_corner_out_bot = (sw_corner - bot.p).manhattan() > bot.r // box is northeast
            || (nw_corner - bot.p).manhattan() > bot.r // box is southeast
            || (ne_corner - bot.p).manhattan() > bot.r // box is southwest
            || (se_corner - bot.p).manhattan() > bot.r; // box is northwest
        if box_corner_out_bot {
            return true;
        }
    }
    return false;
}

fn major_axis_plane_separates(aabb: &Aabb, bot: &Bot) -> bool {
    let xy = |v: Vec3| Vec3::new(v.x, v.y, 0);
    let zy = |v: Vec3| Vec3::new(v.z, v.y, 0);
    let xz = |v: Vec3| Vec3::new(v.x, v.z, 0);

    // looking from front
    if projection_separates(&Aabb::new(xy(aabb.min), xy(aabb.max)),
            &Bot { p: xy(bot.p), r: bot.r }) {
        return true;
    }

    // looking from left
    if projection_separates(&Aabb::new(zy(aabb.min), zy(aabb.max)),
            &Bot { p: zy(bot.p), r: bot.r }) {
        return true;
    }

    // looking from below
    if projection_separates(&Aabb::new(xz(aabb.min), xz(aabb.max)),
            &Bot { p: xz(bot.p), r: bot.r }) {
        return true;
    }

    return false;
}

fn bot_face_separates(aabb: &Aabb, bot: &Bot) -> bool {
    // three adjacent corners of the bot octahedron sit on the same plane and they exist on the
    // major axes one unit away each; clearly the normals are of the form (1a, 1b, 1c) where a,b,c
    // are 1 or -1.
    // (x,y) right and up, z towards viewer. better names for the axes:
    let (north, south, east, west, front, back) = (1, -1, 1, -1, 1, -1);
    // _not_ unit vectors, we don't need the actual distance but just the sign
    // ax + by + cz + d = 0
    // d = -n . p for fixed p (a corner)
    // q . n + d = 0 for any q on plane
    // note:
    //   bot.x + bot.r hits all east faces
    //   bot.x - bot.r hits all west faces
    // so, normals and origin "distances" for the faces:
    // (not sure if it makes sense to spell them out explicitly here other than for my own sanity)
    let planes = [
        (Vec3::new(east, north, front), -(east * (bot.p.x + bot.r) + north * bot.p.y + front * bot.p.z)),
        (Vec3::new(east, north, back),  -(east * (bot.p.x + bot.r) + north * bot.p.y + back * bot.p.z)),
        (Vec3::new(east, south, front), -(east * (bot.p.x + bot.r) + south * bot.p.y + front * bot.p.z)),
        (Vec3::new(east, south, back),  -(east * (bot.p.x + bot.r) + south * bot.p.y + back * bot.p.z)),

        (Vec3::new(west, north, front), -(west * (bot.p.x - bot.r) + north * bot.p.y + front * bot.p.z)),
        (Vec3::new(west, north, back),  -(west * (bot.p.x - bot.r) + north * bot.p.y + back * bot.p.z)),
        (Vec3::new(west, south, front), -(west * (bot.p.x - bot.r) + south * bot.p.y + front * bot.p.z)),
        (Vec3::new(west, south, back),  -(west * (bot.p.x - bot.r) + south * bot.p.y + back * bot.p.z)),
    ];

    some_plane_separates(aabb, &planes)
}

// planes sitting on edges of the bot, rotated 45 degrees from a plane defined by two major axes
fn bot_edge_separates(aabb: &Aabb, bot: &Bot) -> bool {
    let (north, south, east, west, front, back) = (1, -1, 1, -1, 1, -1);
    // note:
    //   bot.x + bot.r hits all east edges
    //   bot.x - bot.r hits all west edges
    // then we have the completely front and back facing edges (only north and south remaining)
    //   bot.p.z + bot.r hits the front edges
    //   bot.p.z - bot.r hits the back edges
    let planes = [
        (Vec3::new(east, north, 0), -(east * (bot.p.x + bot.r) + north * bot.p.y + 0 * bot.p.z)), // y=-x, z free
        (Vec3::new(east, south, 0), -(east * (bot.p.x + bot.r) + south * bot.p.y + 0 * bot.p.z)), // y=x, z free
        (Vec3::new(east, 0, front), -(east * (bot.p.x + bot.r) + 0 * bot.p.y + front * bot.p.z)), // z=-x, y free
        (Vec3::new(east, 0, back),  -(east * (bot.p.x + bot.r) + 0 * bot.p.y + back * bot.p.z)), // z=x, y free

        (Vec3::new(west, north, 0), -(west * (bot.p.x - bot.r) + north * bot.p.y + 0 * bot.p.z)), // y=x, z free
        (Vec3::new(west, south, 0), -(west * (bot.p.x - bot.r) + south * bot.p.y + 0 * bot.p.z)), // y=-x, z free
        (Vec3::new(west, 0, front), -(west * (bot.p.x - bot.r) + 0 * bot.p.y + front * bot.p.z)), // z=x, y free
        (Vec3::new(west, 0, back),  -(west * (bot.p.x - bot.r) + 0 * bot.p.y + back * bot.p.z)), // z=-x, y free

        (Vec3::new(0, north, front), -(0 * bot.p.x + north * bot.p.y + front * (bot.p.z + bot.r))), // z=-y, x free
        (Vec3::new(0, south, front), -(0 * bot.p.x + south * bot.p.y + front * (bot.p.z + bot.r))), // z=y, x free

        (Vec3::new(0, north, back),  -(0 * bot.p.x + north * bot.p.y + back * (bot.p.z - bot.r))), // z=y, x free
        (Vec3::new(0, south, back),  -(0 * bot.p.x + south * bot.p.y + back * (bot.p.z - bot.r))), // z=-y, x free
    ];

    // XXX: how about using planes crossing the bot center? would need half the number of plane
    // tests if just checked that the distance is more than r and of the same sign for all corners?

    some_plane_separates(aabb, &planes)
}

// based on the out-pointing normals we know that the plane-bounded object is inside (dist <= 0).
// if we can find a plane that separates both objects based on all points then it surely separates;
// otherwise, we can't be quite sure if the objects are separated based on just these planes and
// need to test other planes as well
fn some_plane_separates(aabb: &Aabb, planes: &[(Vec3, i64)]) -> bool {
    for &(normal, d) in planes {
        let corners = aabb.corners();
        let mut distances = [0; 8]; // avoid allocation; the size is known
        for (p, dist) in corners.into_iter().zip(distances.iter_mut()) {
            *dist = normal.dot(p) + d;
        }

        // dist == 0 would sit on the space cells and be inside
        if distances.iter().all(|&dist| dist > 0) {
            return true;
        }
    }

    false
}

fn intersect_exact(aabb: &Aabb, bot: &Bot) -> bool {
    // this is likely a common case and fast to calc, so test first
    // (doesn't seem to have a big impact though)
    if major_axis_plane_separates(aabb, bot) {
        return false;
    }

    if bot_face_separates(aabb, bot) {
        return false;
    }

    if bot_edge_separates(aabb, bot) {
        return false;
    }

    true
}

// exact density of the whole aabb, but not necessarily the best pointwise density within this aabb
fn plausible_density_exact(bots: &[Bot], container: &Aabb) -> usize {
    bots.iter().filter(|bot| intersect_exact(container, bot)).count()
}

// just an approximate density; appears to be too conservative
fn plausible_density_aabb(bots: &[Bot], container: &Aabb) -> usize {
    bots.iter().filter(|bot| container.intersects(&bot.aabb())).count()
}

fn plausible_density(bots: &[Bot], container: &Aabb) -> usize {
    if true {
        // gives better data but is much slower to compute
        plausible_density_exact(bots, container)
    } else {
        // faster to compute but way too optimistic so the search becomes slow
        plausible_density_aabb(bots, container)
    }
}

fn point_density(bots: &[Bot], pt: Vec3) -> usize {
    bots.iter().filter(|bot| bot.in_range(pt)).count()
}

// maybe {two_boxes,split}<ScoreType, EvalScore> where EvalScore = Fn(&[Bot], &Aabb) -> Score would
// be more elegant...
type DistFn = fn(&Aabb) -> i64;
type Score = (usize, i64);

// estimate the score in these two bounding boxes and recurse, perhaps prune if one of them is
// clearly bad enough to not contain a better score. "left" and "right" don't mean anything
// actually but are there just to have some names for the two.
fn two_boxes(best: Score, bots: &[Bot], left_box: &Aabb, right_box: &Aabb, distfn: DistFn) -> Score {
    // hopefully the guess correlates with the actually best point density
    let (looks_better, looks_worse, better_score, worse_score) = {
        // overly optimistic heuristic, i.e., the density of the whole box. precise point density
        // is at most this; dists are negated so that comparisons work as expected (bigger better)
        let at_most_left = (plausible_density(bots, left_box), -distfn(left_box));
        let at_most_right = (plausible_density(bots, right_box), -distfn(right_box));

        if at_most_left >= at_most_right {
            (left_box, right_box, at_most_left, at_most_right)
        } else {
            (right_box, left_box, at_most_right, at_most_left)
        }
    };

    if better_score <= best {
        // prune early if a better one cannot be found - maybe the caller thought that these two
        // boxes might have n >= best in total but separately they are <= best each so can skip
        // completely after the caller's optimistic guess
        best
    } else {
        // ok, one of these could have a more dense spot; start with the better-looking guess
        let best = split(best, bots, looks_better, distfn);
        if worse_score <= best {
            // if this is clearly better than the worse guess, the other half can be pruned
            best
        } else {
            // might still be better although the optimistic guess was same or worse
            split(best, bots, looks_worse, distfn)
        }
    }
}

// find max of "best" and the highest point density in this container recursively
fn split(best: Score, bots: &[Bot], container: &Aabb, distfn: DistFn) -> Score {
    let min = container.min;
    let max = container.max;
    let center = container.center();
    let dimensions = container.size();

    if dimensions == Vec3::new(1, 1, 1) {
        // don't actually split; recursion ends here. note: min == max
        best.max((point_density(bots, container.min), -distfn(container)))
    } else {
        // just spatial median split; effective enough
        let (left_box, right_box) = match dimensions.largest() {
            Axis::X => (
                Aabb::new(min, Vec3::new(center.x, max.y, max.z)),
                Aabb::new(Vec3::new(center.x + 1, min.y, min.z), max),
            ),
            Axis::Y => (
                Aabb::new(min, Vec3::new(max.x, center.y, max.z)),
                Aabb::new(Vec3::new(min.x, center.y + 1, min.z), max),
            ),
            Axis::Z => (
                Aabb::new(min, Vec3::new(max.x, max.y, center.z)),
                Aabb::new(Vec3::new(min.x, min.y, center.z + 1), max),
            )
        };
        two_boxes(best, bots, &left_box, &right_box, distfn)
    }
}

fn dist_origin_1dim(a: i64, b: i64) -> i64 {
    if a.signum() != b.signum() {
        // origin is inside this range
        0
    } else {
        a.abs().min(b.abs())
    }
}

fn aabb_dist_origin_manhattan(aabb: &Aabb) -> i64 {
    let dx = dist_origin_1dim(aabb.min.x, aabb.max.x);
    let dy = dist_origin_1dim(aabb.min.y, aabb.max.y);
    let dz = dist_origin_1dim(aabb.min.z, aabb.max.z);
    dx + dy + dz
}

// shortest distance between origin and the location of a point that's in the range of as many
// nanobots as possible; build a bsp tree on the go while searching
fn highest_density(bots: &[Bot]) -> (usize, i64) {
    // note: might speed up the search to sort bots by x, y and z in three reference lists.
    // this is fast enough, so don't bother.
    let universe = bots.iter().fold(bots[0].aabb(), |acc, bot| acc.union(&bot.aabb()));

    let placeholder_dist = |_: &Aabb| 0;
    let real_dist = aabb_dist_origin_manhattan;

    // ignore the distances first to avoid going too deeper in a very small corner even when the
    // density wouldn't be the best one in the universe
    let (best_density, _) = split((0, 0), bots, &universe, placeholder_dist);
    // now prune all but the boxes with the best possible density and figure out the distance
    let (density, dist) = split((best_density - 1, 0), bots, &universe, real_dist);
    assert!(density == best_density);

    (density, -dist)
}

fn main() {
    let bots = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|l| parse_line(&l.unwrap())).collect::<Vec<_>>();
    println!("a: {:?}", biggest_contains(&bots));
    println!("b: {:?}", highest_density(&bots));
}
