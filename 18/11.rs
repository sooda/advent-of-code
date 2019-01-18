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

fn cumulative_powers(serial: i32) -> Vec<i32> {
    let mut map = vec![0; 300 * 300];
    let at = |x: i32, y: i32| (y * 300 + x) as usize;

    // prime the borders
    map[at(0, 0)] = fuel_cell_power_level(1, 1, serial);
    for i in 1..300 {
        // remember, coords for fuel cells start from (1, 1)
        map[at(i, 0)] = map[at(i - 1,     0)] + fuel_cell_power_level(1 + i, 1, serial);
        map[at(0, i)] = map[at(0    , i - 1)] + fuel_cell_power_level(1, 1 + i, serial);
    }

    // rest of the table is recursive in 2d
    for y in 1..300 {
        for x in 1..300 {
            let x0y0 = fuel_cell_power_level(1 + x, 1 + y, serial);
            let x1y0 = map[at(x - 1, y    )];
            let x0y1 = map[at(x    , y - 1)];
            let x1y1 = map[at(x - 1, y - 1)];
            map[at(x, y)] = x0y0 + x1y0 + x0y1 - x1y1;
        }
    }

    map
}

fn subsquare(map: &[i32], x: i32, y: i32, sz: i32) -> i32 {
    // bottom right corner is in, others are just out
    // P = (x, y), D = (x + sz - 1, y + sz - 1)
    // +----+---+
    // |....|...|
    // |....|...|
    // |...A|...B
    // +----P---+
    // |....|...|
    // +---C+---D
    // (might be easier to just add zero padding to the array)
    let get = |x, y| if x >= 0 && y >= 0 { map[(y * 300 + x) as usize] } else { 0 };
    get(x + sz - 1, y + sz - 1) -
    get(x + sz - 1, y - 1     ) -
    get(x - 1     , y + sz - 1) +
    get(x - 1     , y - 1     )
}

fn largest_total_square(serial: i32) -> (i32, i32, i32, i32) {
    let power_map = cumulative_powers(serial);

    let search_space = (0..300).flat_map(|y| {
        (0..300).flat_map(move |x| {
            (1..=((300 - y).min(300 - x))).map(move |sz| {
                (x, y, sz)
            })
        })
    });

    search_space.map(|(x, y, sz)|
        (subsquare(&power_map, x, y, sz), 1 + x, 1 + y, sz)
    ).max().unwrap()
}

fn main() {
    assert_eq!(fuel_cell_power_level(3, 5, 8), 4);
    assert_eq!(fuel_cell_power_level(122, 79, 57), -5);
    assert_eq!(fuel_cell_power_level(217, 196, 39), 0);
    assert_eq!(fuel_cell_power_level(101, 153, 71), 4);
    assert_eq!(square_power(33, 45, 18, 3), 29);
    assert_eq!(square_power(21, 61, 42, 3), 30);
    assert_eq!(powerest_square(18, 3), (29, 33, 45));
    assert_eq!(powerest_square(42, 3), (30, 21, 61));

    let puzzle_input = 9110;
    println!("{:?}", powerest_square(puzzle_input, 3));

    println!("{:?}", largest_total_square(puzzle_input));
    assert_eq!(largest_total_square(18), (113, 90, 269, 16));
    assert_eq!(largest_total_square(42), (119, 232, 251, 12));
}
