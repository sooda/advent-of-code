use std::io::{self, BufRead};

extern crate regex;
use regex::Regex;

type Vek = (i32, i32, i32);

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

fn kinetic_energy(moon: &Vek, vel: &Vek) -> i32 {
    (moon.0.abs() + moon.1.abs() + moon.2.abs()) *
        (vel.0.abs() + vel.1.abs() + vel.2.abs())
}

fn total_energy(moons: &[Vek], velocities: &[Vek]) -> i32 {
    moons.iter().zip(velocities).map(|(m, v)| kinetic_energy(m, v)).sum()
}

fn energy_after(mut moons: Vec<Vek>, n: usize) -> i32 {
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
    println!("m {:?}", moons);
    println!("v {:?}", vels);

    total_energy(&moons, &vels)
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
}
