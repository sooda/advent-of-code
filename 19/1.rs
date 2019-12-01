use std::io::{self, BufRead};

fn fuel_requirements(mass: u32) -> u32 {
    if mass > 2 * 3 {
        mass / 3 - 2
    } else {
        // now we instead wish really hard
        0
    }
}

fn total_fuel_requirements(mass: u32) -> u32 {
    let fuel = fuel_requirements(mass);
    if fuel > 0 {
        fuel + total_fuel_requirements(fuel)
    } else {
        fuel
    }
}

fn main() {
    assert_eq!(fuel_requirements(12), 2);
    assert_eq!(fuel_requirements(14), 2);
    assert_eq!(fuel_requirements(1969), 654);
    assert_eq!(fuel_requirements(100756), 33583);

    assert_eq!(total_fuel_requirements(14), 2);
    assert_eq!(total_fuel_requirements(1969), 966);
    assert_eq!(total_fuel_requirements(100756), 50346);

    let all_spacecrafts_up_high: (u32, u32) = io::stdin().lock().lines()
        .map(|massline| massline.unwrap().parse().unwrap())
        .map(|mass| (fuel_requirements(mass), total_fuel_requirements(mass)))
        .fold((0, 0), |sum, i| (sum.0 + i.0, sum.1 + i.1));
    println!("{:?}", all_spacecrafts_up_high);
}
