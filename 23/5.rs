use std::io::{self, Read};

extern crate regex;
use regex::Regex;

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

fn lowest_location(almanac: &Almanac) -> u32 {
    almanac.seeds.iter().map(|&s| map_to_location(almanac, s, 0)).min().unwrap()
}

fn lowest_location_ranged(almanac: &Almanac) -> u32 {
    almanac.seeds.chunks(2).map(|spec| {
        (spec[0] .. spec[0] + spec[1]).map(|s| map_to_location(almanac, s, 0)).min().unwrap()
    }).min().unwrap()
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
