use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn path_letters(map: &[String]) -> (String, usize) {
    let mut path = String::new();
    let mut x = map[0].as_bytes().iter().position(|&x| x == b'|').unwrap() as i32;
    let mut y = 0i32;
    let mut dx = 0i32;
    let mut dy = 1i32;
    let w = map[0].as_bytes().len() as i32;
    let h = map.len() as i32;
    let mut steps = 0;

    let at = |x: i32, y: i32| map[y as usize].as_bytes()[x as usize];

    loop {
        //println!("{}+{} {}+{} {}", x, dx, y, dy, at(x, y) as char);
        match at(x, y)  {
            b'|' | b'-' => {
                // go on
            }
            b'+' => {
                if dx != 1 && x > 0 && at(x - 1, y) != b' ' {
                    dx = -1;
                    dy = 0;
                    //println!("left");
                } else if dx != -1 && x < w - 1 && at(x + 1, y) != b' ' {
                    dx = 1;
                    dy = 0;
                    //println!("right");
                } else if dy != 1 && y > 0 && at(x, y - 1) != b' ' {
                    dx = 0;
                    dy = -1;
                    //println!("up");
                } else if dy != -1 && y < h - 1 && at(x, y + 1) != b' ' {
                    dx = 0;
                    dy = 1;
                    //println!("down");
                } else {
                    unreachable!()
                }
            }
            b'A' ... b'Z' => {
                //println!("{}", at(x, y) as char);
                path.push(at(x, y) as char);
            },
            b' ' => break,
            _ => unreachable!()
        }
        x += dx;
        y += dy;
        steps += 1;
    }

    (path, steps)
}

fn main() {
    let map = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    println!("{:?}", path_letters(&map));
}
