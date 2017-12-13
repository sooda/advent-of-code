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
    let mut severity = 0;
    for layer in firewall {
        // "skip" to this depth -- severity does not accumulate in empty slots and scanner_pos has
        // no state
        let my_depth_and_time = layer.depth;
        if scanner_pos(layer, my_depth_and_time) == 0 {
            severity += layer.depth * layer.range;
        }
    }

    severity
}

fn main() {
    let firewall = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    println!("{}", severity(&firewall));
}
