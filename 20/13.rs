use std::io::{self, BufRead};

fn next_bus(timestamp: u64, schedule: &[Option<u64>]) -> (u64, u64) {
    schedule.iter().filter_map(|&maybe_bus| maybe_bus).map(|bus| {
        let since_started = timestamp % bus;
        let until_again = bus - since_started;
        (until_again, bus)
    }).min().unwrap()
}

fn bus_race(schedule: &[Option<u64>]) -> u64 {
    // (bus, index) for the remainders, skip nones
    let sched: Vec<(u64, u64)> = schedule.iter().enumerate()
        .filter_map(|(i, n)| n.map(|n| (n, i as u64)))
        .collect();
    // sieve for chinese remainder theorem on this system of congruences
    let mut prev_start = 0;
    let mut prev_mult = 1;
    // time = offs (mod bus) for each transport line
    for window in sched.windows(2) {
        let ((bus0, _), (bus1, offs1)) = (window[0], window[1]);
        // some buses with a short cycle are relatively far away, so they start after completing
        // many cycles; this method needs the offsets to be within the cycle so make it so
        let offs1 = offs1 % bus1;
        for i in 1.. {
            // this is unreadable so look it up on wikipedia
            if (prev_start + i * bus0 * prev_mult + offs1) % bus1 == 0 {
                prev_start += i * bus0 * prev_mult;
                prev_mult *= bus0;
                break;
            }
        }
    }
    prev_start
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
    println!("{}", bus_race(&schedule));
}
