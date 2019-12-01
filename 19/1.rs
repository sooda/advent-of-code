use std::io::{self, BufRead};

fn fuel_requirements(mass: u32) -> u32 {
    mass / 3 - 2
}

fn main() {
    assert_eq!(fuel_requirements(12), 2);
    assert_eq!(fuel_requirements(14), 2);
    assert_eq!(fuel_requirements(1969), 654);
    assert_eq!(fuel_requirements(100756), 33583);

    let all_spacecrafts_up_high: u32 = io::stdin().lock().lines().map(|massline|
        fuel_requirements(massline.unwrap().parse().unwrap())).sum();
    println!("{}", all_spacecrafts_up_high);
}
