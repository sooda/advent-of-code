use std::io::{self, BufRead};

fn num_digits(layer: &[u8]) -> (usize, usize, usize) {
    layer.iter().fold((0, 0, 0), |sum, digit| {
        match digit {
            b'0' => (sum.0 + 1, sum.1, sum.2),
            b'1' => (sum.0, sum.1 + 1, sum.2),
            b'2' => (sum.0, sum.1, sum.2 + 1),
            _ => panic!()
        }
    })
}

fn decode(digits: &[u8]) -> [u8; 25 * 6] {
    let mut image = [b'2'; 25 * 6];
    for layer in digits.chunks(25 * 6) {
        for (&input, output) in layer.iter().zip(image.iter_mut()) {
            if *output == b'2' {
                *output = input;
            }
        }
    }
    image
}

fn print_layer(layer: &[u8]) {
    for row in layer.chunks(25) {
        for &ch in row {
            print!("{}", if ch == b'1' { '#' } else { ' ' });
        }
        println!();
    }
}

fn main() {
    let digits: Vec<u8> = io::stdin().lock().lines().next().unwrap().unwrap()
        .bytes().collect();
    let fewest_0 = digits.chunks(25 * 6).map(num_digits).min_by_key(|&d| d.0).unwrap();
    println!("{}", fewest_0.1 * fewest_0.2);
    print_layer(&decode(&digits));
}
