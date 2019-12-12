use std::io::{self, BufRead};
use std::collections::HashSet;

extern crate regex;
use regex::Regex;

// The coords stay rather small. Keep the datatype small for less memory traffic with the history
type Vek = (i16, i16, i16);

fn gravity(moons: &[Vek], velocities: &[Vek]) -> Vec<Vek> {
    let mut out = Vec::new();
    for (i, (&a, &av)) in moons.iter().zip(velocities).enumerate() {
        let mut newvel = av;
        for (j, (&b, &_bv)) in moons.iter().zip(velocities).enumerate() {
            if i == j {
                continue;
            }
            if a.0 < b.0 {
                newvel.0 += 1;
            } else if a.0 > b.0 {
                newvel.0 -= 1;
            }
            if a.1 < b.1 {
                newvel.1 += 1;
            } else if a.1 > b.1 {
                newvel.1 -= 1;
            }
            if a.2 < b.2 {
                newvel.2 += 1;
            } else if a.2 > b.2 {
                newvel.2 -= 1;
            }
        }
        out.push(newvel);
    }
    out
}

fn velocity(moons: &[Vek], velocities: &[Vek]) -> Vec<Vek> {
    let mut out = Vec::new();
    for (&a, &av) in moons.iter().zip(velocities) {
        out.push((a.0 + av.0, a.1 + av.1, a.2 + av.2));
    }
    out
}

fn kinetic_energy(moon: &Vek, vel: &Vek) -> i16 {
    (moon.0.abs() + moon.1.abs() + moon.2.abs()) *
        (vel.0.abs() + vel.1.abs() + vel.2.abs())
}

fn total_energy(moons: &[Vek], velocities: &[Vek]) -> i16 {
    moons.iter().zip(velocities).map(|(m, v)| kinetic_energy(m, v)).sum()
}

fn energy_after(mut moons: Vec<Vek>, n: usize) -> i16 {
    let mut vels = vec![(0, 0, 0); moons.len()];

    for _step in 0..n {
        /*
        println!("After {} steps:", _step);
        for (&m, &v) in moons.iter().zip(&vels) {
            println!("pos=<x={:-2}, y={:-2}, z={:-2}>, vel=<x={:-2}, y={:-2}, z={:-2}>",
                m.0, m.1, m.2, v.0, v.1, v.2);
        }
        println!();
        */
        vels = gravity(&moons, &vels);
        moons = velocity(&moons, &vels);
    }

    total_energy(&moons, &vels)
}

fn fully_overlapping<F: Fn(&(Vek, Vek)) -> i16>(hist: &[Vec<(Vek, Vek)>],
        a: usize, b: usize, len: usize, moon_ix: usize, moon_prop: F) -> bool {
    (&hist[a..(a + len)]).iter().zip(&hist[b..(b + len)])
        .all(|(amoons, bmoons)| moon_prop(&amoons[moon_ix]) == moon_prop(&bmoons[moon_ix]))
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != 0 {
        let c = b % a;
        b = a;
        a = c;
    }
    b
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn find_common_cycle(hist: &[Vec<(Vek, Vek)>]) -> usize {
    // this much overlap can be detected
    let cyc_len_max = hist.len() / 2;
    let moon_count = hist[0].len();

    // (SoA might be faster than AoS due to gaps in memory but a few seconds is good enough)
    let moon_props: &[Box<dyn Fn(&(Vek, Vek)) -> i16>] = &[
        Box::new(|&(a, _)| a.0 ),
        Box::new(|&(a, _)| a.1 ),
        Box::new(|&(a, _)| a.2 ),
        Box::new(|&(_, b)| b.0 ),
        Box::new(|&(_, b)| b.1 ),
        Box::new(|&(_, b)| b.2 ),
    ];
    // the cycle may appear more than once in the log
    let mut scalar_cycles = vec![Vec::new(); moon_count * moon_props.len()];
    // certainly no shorter cycles than 1000 or so but whatever
    for cyc_len in 2..=cyc_len_max {
        let mut cycling_props = 0;
        // find the cycle of each scalar separately
        let mut found_iter = scalar_cycles.iter_mut();
        for moon_ix in 0..moon_count {
            for moon_prop in moon_props.iter() {
                let start_a = hist.len() - 2 * cyc_len;
                let start_b = hist.len() - cyc_len;
                let foundvec = found_iter.next().unwrap();
                if fully_overlapping(hist, start_a, start_b, cyc_len, moon_ix, moon_prop) {
                    cycling_props += 1;
                    foundvec.push(cyc_len);
                }
            }
        }
        if cycling_props == 4 * 6 {
            println!("oops, found all already");
            return cyc_len;
        }
    }

    // sanity check
    for ith_cycles in &scalar_cycles {
        if false {
            println!("cycles found: {:?}", ith_cycles);
        }
        let smallest = ith_cycles[0];
        for cyc in &ith_cycles[1..] {
            if cyc % smallest != 0 {
                panic!("consider increasing the smallest allowed cycle, these don't add up");
            }
        }
    }

    // remove dupes
    let unique_cycles = scalar_cycles.into_iter().map(|v| v[0]).collect::<HashSet<_>>();

    println!("lcm {:?}", unique_cycles);
    unique_cycles.into_iter().fold(1, lcm)
}

fn world_period(mut moons: Vec<Vek>) -> usize {
    let mut vels = vec![(0, 0, 0); moons.len()];
    let mut hist = Vec::new();
    // from [[moonpos], [moonvel]] into [(moonpos, moonvel)]
    let history_transform = |moons: &Vec<Vek>, vels: &Vec<Vek>| moons.iter().zip(vels)
        .map(|(&x, &y)| (x, y)).collect::<Vec<(Vek, Vek)>>();

    // this is surely long enough for each cycle
    for _step in 0..500000 {
        hist.push(history_transform(&moons, &vels));
        vels = gravity(&moons, &vels);
        moons = velocity(&moons, &vels);
    }

    find_common_cycle(&hist)
}

fn parse_moon(line: String) -> Vek {
    // <x=-1, y=0, z=2>
    let re = Regex::new(r"<x=([-\d]+), y=([-\d]+), z=([-\d]+)>").unwrap();
    let cap = re.captures(&line).unwrap();
    let x = cap.get(1).unwrap().as_str().parse().unwrap();
    let y = cap.get(2).unwrap().as_str().parse().unwrap();
    let z = cap.get(3).unwrap().as_str().parse().unwrap();
    (x, y, z)
}

fn main() {
    let moons: Vec<_> = io::stdin().lock().lines().map(
        |line| parse_moon(line.unwrap())
    ).collect();

    println!("{}", energy_after(moons.clone(), 1000));
    println!("{}", world_period(moons.clone()));
}
