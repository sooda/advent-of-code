fn fuel_cell_power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    (rack_id * y + serial) * rack_id / 100 % 10 - 5
}

fn square_power(x0: i32, y0: i32, serial: i32, sz: i32) -> i32 {
    (0..sz).flat_map(|y| (0..sz).map(move |x| {
        fuel_cell_power_level(x0 + x, y0 + y, serial)
    })).sum()
}

fn powerest_square(serial: i32, sz: i32) -> (i32, i32, i32) {
    (1..=300-sz+1).flat_map(|y| (1..=300-sz+1).map(move |x| {
        (square_power(x, y, serial, sz), x, y)
    })).max().unwrap()
}

fn largest_total_square(serial: i32) -> ((i32, i32, i32), i32) {
    (0..300).map(|sz| (powerest_square(serial, sz), sz)).max().unwrap()
}

fn main() {
    assert!(fuel_cell_power_level(3, 5, 8) == 4);
    assert!(fuel_cell_power_level(122, 79, 57) == -5);
    assert!(fuel_cell_power_level(217, 196, 39) == 0);
    assert!(fuel_cell_power_level(101, 153, 71) == 4);
    assert!(square_power(33, 45, 18, 3) == 29);
    assert!(square_power(21, 61, 42, 3) == 30);
    assert!(powerest_square(18, 3) == (29, 33, 45));
    assert!(powerest_square(42, 3) == (30, 21, 61));

    let puzzle_input = 9110;
    println!("{:?}", powerest_square(puzzle_input, 3));

    println!("{:?}", largest_total_square(puzzle_input));
    assert!(largest_total_square(18) == ((113, 90, 269), 16));
    assert!(largest_total_square(42) == ((119, 232, 251), 12));
}
