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

#[allow(dead_code)]
fn largest_total_square_naive(serial: i32) -> ((i32, i32, i32), i32) {
    (0..=300).map(|sz| (powerest_square(serial, sz), sz)).max().unwrap()
}

fn largest_total_square_faster(serial: i32) -> ((i32, i32, i32), i32) {
    let mut max = ((0, 0, 0), 0);

    for y0 in 0..=300 {
        let maxsz_y = 300 - y0;
        for x0 in 0..=300 {
            let maxsz_x = 300 - x0;
            let mut sum = fuel_cell_power_level(x0, y0, serial);
            max = max.max(((sum, x0, y0), 1));

            let sz = maxsz_y.min(maxsz_x);
            for off in 1..sz {
                let bot_x = x0 + off;
                let bot_y = y0 + off;
                // bottom right corner is included in the column part
                let mut bottom_row = (x0..bot_x)
                    .map(|x| fuel_cell_power_level(x, bot_y, serial)).sum::<i32>();
                let mut right_col = (y0..=bot_y)
                    .map(|y| fuel_cell_power_level(bot_x, y, serial)).sum::<i32>();
                sum += bottom_row + right_col;
                max = max.max(((sum, x0, y0), 1 + off));
            }
        }
    }

    max
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

    println!("{:?}", largest_total_square_faster(puzzle_input));
    assert!(largest_total_square_faster(18) == ((113, 90, 269), 16));
    assert!(largest_total_square_faster(42) == ((119, 232, 251), 12));
}
