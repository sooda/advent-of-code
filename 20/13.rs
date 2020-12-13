use std::io::{self, BufRead};

fn next_bus(timestamp: u64, schedule: &[Option<u64>]) -> (u64, u64) {
    schedule.iter().filter_map(|&maybe_bus| maybe_bus).map(|bus| {
        let since_started = timestamp % bus;
        let until_again = bus - since_started;
        (until_again, bus)
    }).min().unwrap()
}

fn main() {
    let stdin = io::stdin();
    // aaargh
    let timestamp = stdin.lock().lines().next().unwrap().unwrap().parse::<u64>().unwrap();
    let schedule = stdin.lock().lines().next().unwrap().unwrap();
    // buses marked with "x" become None, others Some(number)
    let schedule: Vec<Option<_>> = schedule.split(",").map(|b| b.parse::<u64>().ok()).collect();

    let bus = next_bus(timestamp, &schedule);
    println!("{} minutes for bus {} = {}", bus.0, bus.1, bus.0 * bus.1);
}
