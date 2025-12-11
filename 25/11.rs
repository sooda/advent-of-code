use std::io::{self, BufRead};
use std::collections::HashMap;

type Net = HashMap<String, Vec<String>>;

fn find_paths<'a>(net: &'a Net, node: &'a str, end: &str, paths: &mut HashMap<&'a str, usize>) -> usize {
    if node == end {
        1
    } else if let Some(&n) = paths.get(node) {
        n
    } else if let Some(edges) = net.get(node) {
        let n = edges.iter().map(|e| find_paths(net, e, end, paths)).sum();
        paths.insert(node, n);
        n
    } else {
        0
    }
}

fn total_paths_between(net: &Net, start: &str, end: &str) -> usize {
    find_paths(net, start, end, &mut HashMap::new())
}

fn total_paths(net: &Net) -> usize {
    total_paths_between(net, "you", "out")
}

fn total_svr_paths(net: &Net) -> usize {
    let svr_dac = total_paths_between(net, "svr", "dac");
    let svr_fft = total_paths_between(net, "svr", "fft");
    let dac_fft = total_paths_between(net, "dac", "fft");
    let fft_dac = total_paths_between(net, "fft", "dac");
    let fft_out = total_paths_between(net, "fft", "out");
    let dac_out = total_paths_between(net, "dac", "out");
    svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out
}

fn parse(line: &str) -> (String, Vec<String>) {
    let (node, edgestr) = line.split_once(": ").unwrap();
    let edges = edgestr.split(' ').map(|e| e.to_string()).collect();
    (node.to_string(), edges)
}

fn main() {
    let net: Net = io::stdin().lock().lines()
        .map(|line| parse(&line.unwrap())
            ).collect();
    println!("{}", total_paths(&net));
    println!("{}", total_svr_paths(&net));
}
