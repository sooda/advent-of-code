use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
struct Layer {
    depth: u32,
    range: u32
}

fn parse_line(line: &str) -> Layer {
    let mut parts = line.split(": ");
    let depth = parts.next().unwrap().parse().unwrap();
    let range = parts.next().unwrap().parse().unwrap();
    Layer { depth: depth, range: range }
}

// we only care about when it's in pos 0; the scanner bounces off the edges and a position of
// range + 1 means it's at range - 1 going up.
fn scanner_pos(layer: &Layer, time: u32) -> u32 {
    let movement_steps = 2 * layer.range - 2;
    time % movement_steps
}

fn severity(firewall: &[Layer]) -> u32 {
    // my depth is the same as the layer depth in each timestep
    firewall.iter()
        .filter(|layer| scanner_pos(layer, layer.depth) == 0)
        .map(|layer| layer.depth * layer.range)
        .sum()
}

fn safe_time(firewall: &[Layer]) -> u32 {
    // the first collision would give severity 0, but it's not allowed either
    (0..).find(|delay|
               firewall.iter()
               .find(|layer| scanner_pos(layer, delay + layer.depth) == 0)
               .is_none()
    ).unwrap()
}

fn main() {
    let firewall = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    println!("{}", severity(&firewall));
    println!("{}", safe_time(&firewall));
}
