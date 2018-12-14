fn fuel_cell_power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    (rack_id * y + serial) * rack_id / 100 % 10 - 5
}

fn square_power(x0: i32, y0: i32, serial: i32) -> i32 {
    (0..3).flat_map(|y| (0..3).map(move |x| {
        fuel_cell_power_level(x0 + x, y0 + y, serial)
    })).sum()
}

fn powerest_square(serial: i32) -> (i32, i32, i32) {
    (1..=300-2).flat_map(|y| (1..=300-2).map(move |x| {
        (square_power(x, y, serial), x, y)
    })).max().unwrap()
}

fn main() {
    assert!(fuel_cell_power_level(3, 5, 8) == 4);
    assert!(fuel_cell_power_level(122, 79, 57) == -5);
    assert!(fuel_cell_power_level(217, 196, 39) == 0);
    assert!(fuel_cell_power_level(101, 153, 71) == 4);
    assert!(square_power(33, 45, 18) == 29);
    assert!(square_power(21, 61, 42) == 30);
    assert!(powerest_square(18) == (29, 33, 45));
    assert!(powerest_square(42) == (30, 21, 61));
    let puzzle_input = 9110;
    println!("{:?}", powerest_square(puzzle_input));
}
