use std::io::{self, Read};

extern crate regex;
use regex::Regex;

// consume 3m46s to do naive computation vs 2.5 seconds with ranges
// (which too is quite lot, probably should prune a bit)
const DOUBLE_CHECK: bool = false;

#[derive(Debug)]
struct MapRange {
    dst_start: u32,
    src_start: u32,
    len: u32,
}

type Map = Vec<MapRange>;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Map>,
}

fn map_lookup(map: &Map, property: u32) -> u32 {
    for range in map {
        if property >= range.src_start {
            let delta = property - range.src_start;
            if delta < range.len {
                return range.dst_start + delta;
            }
        }
    }
    return property;
}

fn map_to_location(almanac: &Almanac, property: u32, lookup: usize) -> u32 {
    if lookup == almanac.maps.len() {
        return property;
    }
    let next = map_lookup(&almanac.maps[lookup], property);
    map_to_location(almanac, next, lookup + 1)
}

fn overlap(a: (u32, u32), b: (u32, u32)) -> Option<(u32, u32)> {
    let left = a.0.max(b.0);
    let right = a.1.min(b.1);
    if left <= right {
        Some((left, right))
    } else {
        None
    }
}

// end is inclusive
fn map_to_location_range(almanac: &Almanac, prop_start: u32, prop_end: u32, lookup: usize) -> u32 {
    if lookup == almanac.maps.len() {
        return prop_start;
    }
    let mut smol = std::u32::MAX;
    for range in &almanac.maps[lookup] {
        let src_end = range.src_start + range.len - 1;
        if let Some(together) = overlap((prop_start, prop_end), (range.src_start, src_end)) {
            let off_l = together.0 - range.src_start;
            let off_r = together.1 - range.src_start;
            smol = smol.min(map_to_location_range(almanac, range.dst_start + off_l, range.dst_start + off_r, lookup + 1));

            if prop_start < range.src_start {
                // split the range and retry the left only
                smol = smol.min(map_to_location_range(almanac, prop_start, range.src_start - 1, lookup));
            }

            if prop_end > src_end {
                // split the range and retry the right only
                smol = smol.min(map_to_location_range(almanac, src_end + 1, prop_end, lookup));
            }
        }
    }
    if smol == std::u32::MAX {
        // nothing matched, entire range maps 1:1
        smol = smol.min(map_to_location_range(almanac, prop_start, prop_end, lookup + 1));
    }
    smol
}

fn lowest_location(almanac: &Almanac) -> u32 {
    almanac.seeds.iter().map(|&s| map_to_location(almanac, s, 0)).min().unwrap()
}

fn _lowest_location_ranged_naive(almanac: &Almanac) -> u32 {
    almanac.seeds.chunks(2).map(|spec| {
        (spec[0] .. spec[0] + spec[1]).map(|s| map_to_location(almanac, s, 0)).min().unwrap()
    }).min().unwrap()
}

fn lowest_location_ranged(almanac: &Almanac) -> u32 {
    let ret = almanac.seeds.chunks(2).map(|spec| {
        map_to_location_range(almanac, spec[0], spec[0] + spec[1] - 1, 0)
    }).min().unwrap();
    if DOUBLE_CHECK {
        assert_eq!(ret, _lowest_location_ranged_naive(almanac));
    }
    ret
}

fn parse(almanac: &str) -> Almanac {
    let fmt = r"(?m)seeds: ([\d ]+)

seed-to-soil map:
([\d \n]+)

soil-to-fertilizer map:
([\d \n]+)

fertilizer-to-water map:
([\d \n]+)

water-to-light map:
([\d \n]+)

light-to-temperature map:
([\d \n]+)

temperature-to-humidity map:
([\d \n]+)

humidity-to-location map:
([\d \n]+)";

    let re = Regex::new(fmt).unwrap();
    let cap = re.captures(almanac).unwrap();
    let seeds = cap.get(1).unwrap().as_str().split(' ').map(|n| n.parse().unwrap()).collect::<Vec<_>>();

    /*
     * 50 98 2\n \
     * 52 50 48\n\n
     */
    let maps = (2..=8).map(|i| {
        cap.get(i).unwrap().as_str().trim_end().split('\n').map(|rule_spec| {
            let mut sp = rule_spec.split(' ');
            let dst_start = sp.next().unwrap().parse().unwrap();
            let src_start = sp.next().unwrap().parse().unwrap();
            let len = sp.next().unwrap().parse().unwrap();
            MapRange { dst_start, src_start, len }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    Almanac { seeds, maps }
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let almanac = parse(&file);
    println!("{}", lowest_location(&almanac));
    println!("{}", lowest_location_ranged(&almanac));
}
