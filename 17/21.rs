use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Block2([[u8; 2]; 2]);
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Block3([[u8; 3]; 3]);
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Block4([[u8; 4]; 4]);

//#[derive(Debug)]
//struct Rule2to3(Block2, Block3);
//#[derive(Debug)]
//struct Rule3to4(Block3, Block4);

#[derive(Debug)]
enum Rule {
    Rule2to3(Block2, Block3),
    Rule3to4(Block3, Block4)
}
use Rule::*;

fn parse_two(dst: &mut Block2, src: &[u8]) {
    let off = "../".len();
    dst.0[0].copy_from_slice(&src[0..2]);
    dst.0[1].copy_from_slice(&src[off..off + 2]);
}

fn parse_three(dst: &mut Block3, src: &[u8]) {
    //println!("{:?}", src);
    let off = ".../".len();
    dst.0[0].copy_from_slice(&src[0..3]);
    dst.0[1].copy_from_slice(&src[off..off + 3]);
    dst.0[2].copy_from_slice(&src[2*off..2*off + 3]);
}

fn parse_four(dst: &mut Block4, src: &[u8]) {
    //println!("{:?}", src);
    let off = "..../".len();
    dst.0[0].copy_from_slice(&src[0..4]);
    dst.0[1].copy_from_slice(&src[off..off + 4]);
    dst.0[2].copy_from_slice(&src[2*off..2*off + 4]);
    dst.0[3].copy_from_slice(&src[3*off..3*off + 4]);
}

fn parse_line(line: &str) -> Rule {
    let bytes = line.as_bytes();
    let two_pattern_len = "../..".len();
    let three_pattern_len = ".../.../...".len();
    let four_pattern_len = "..../..../..../....".len();
    let arrow_len = " => ".len();
    if line.len() == "../.. => ###/.##/#..".len() {
        let mut from = Block2([[0; 2]; 2]);
        let mut to = Block3([[0; 3]; 3]);
        parse_two(&mut from, &bytes[0..two_pattern_len]);
        //println!("{:?} {:?}", line, from);
        parse_three(&mut to, &bytes[two_pattern_len + arrow_len..
                                    two_pattern_len + arrow_len + three_pattern_len]);
        //println!("{:?} {:?}", line, to);
        Rule2to3(from, to)
    } else if line.len() == ".../.../... => #.#./#..#/#.##/#.#.".len() {
        let mut from = Block3([[0; 3]; 3]);
        let mut to = Block4([[0; 4]; 4]);
        parse_three(&mut from, &bytes[0..three_pattern_len]);
        //println!("{:?} {:?}", line, from);
        parse_four(&mut to, &bytes[three_pattern_len + arrow_len..
                                    three_pattern_len + arrow_len + four_pattern_len]);
        //println!("{:?} {:?}", line, to);
        Rule3to4(from, to)
    } else {
        unreachable!()
    }
}

/*
struct Block2([[u8; 2]; 2]);

impl Block2 {
    fn from(src: &[u8], x: usize, y: usize) -> Self {
        let mut b = Block2([[0; 2]; 2]);
        let w = (src.len() as f64).sqrt() as usize;
        let off = y * w + x;
        for i in 0..2 {
            b.0[i].copy_from_slice(&src[off+i*w..off+i*w + 2]);
        }
        b
    }
}

*/
//struct Block3([[u8; 3]; 3]);

impl Block2 {
    fn from(src: &[u8], x: usize, y: usize) -> Self {
        let mut b = Block2([[0; 2]; 2]);
        let w = (src.len() as f64).sqrt() as usize;
        let off = y * w + x;
        for i in 0..2 {
            b.0[i].copy_from_slice(&src[off+i*w..off+i*w + 2]);
        }
        b
    }
    fn flip(&self) -> Self {
        let mut b = Block2([[0; 2]; 2]);
        b.0[0][0] = self.0[1][0];
        b.0[0][1] = self.0[1][1];
        b.0[1][1] = self.0[0][1];
        b.0[1][0] = self.0[0][0];
        b
    }
    fn rotleft(&self) -> Self {
        let mut b = Block2([[0; 2]; 2]);
        b.0[0][0] = self.0[0][1];
        b.0[0][1] = self.0[1][1];
        b.0[1][1] = self.0[1][0];
        b.0[1][0] = self.0[0][0];
        b
    }
    fn rotright(&self) -> Self {
        let mut b = Block2([[0; 2]; 2]);
        b.0[0][1] = self.0[0][0];
        b.0[1][1] = self.0[0][1];
        b.0[1][0] = self.0[1][1];
        b.0[0][0] = self.0[1][0];
        b
    }
    fn rot180(&self) -> Self {
        let mut b = Block2([[0; 2]; 2]);
        b.0[0][0] = self.0[1][1];
        b.0[0][1] = self.0[1][0];
        b.0[1][1] = self.0[0][0];
        b.0[1][0] = self.0[0][1];
        b
    }
}

impl Block3 {
    fn from(src: &[u8], x: usize, y: usize) -> Self {
        let mut b = Block3([[0; 3]; 3]);
        let w = (src.len() as f64).sqrt() as usize;
        let off = y * w + x;
        for i in 0..3 {
            b.0[i].copy_from_slice(&src[off+i*w..off+i*w + 3]);
        }
        b
    }

    fn to(&self, dst: &mut [u8], x: usize, y: usize) {
        let w = (dst.len() as f64).sqrt().ceil() as usize;
        //println!("aaa {:?} {:?}", *self, w);
        let off = y * w + x;
        for i in 0..3 {
            //println!("q {:?}", &dst[off+i*w..off+i*w + 3]);
            dst[off+i*w..off+i*w + 3].copy_from_slice(&self.0[i]);
            //println!("w {:?}", &dst[off+i*w..off+i*w + 3]);
        }
    }
    fn transform<F>(&self, f: F) -> Self
    where F: Fn(usize, usize) -> (usize, usize) {
        let mut b = Block3([[0; 3]; 3]);

        for y in 0..3 {
            for x in 0..3 {
                let src = f(x, y);
                b.0[y][x] = self.0[src.1][src.0];
            }
        }

        b
    }
    fn flip(&self) -> Self {
        /*
         * 012
         * 345
         * 678
         *
         * 210
         * 543
         * 876
         */
        self.transform(|x, y| (2 - x, y))
    }
    fn rotleft(&self) -> Self {
        /*
         * 012
         * 345
         * 678
         *
         * 258 0,0: 2,0   2,0: 2,2
         * 147 2,1: 1,2
         * 036 0,2: 0,0
         */
        self.transform(|x, y| (2 - y, x))
    }
    fn rotright(&self) -> Self {
        self.rotleft().rotleft().rotleft()
    }
    fn rot180(&self) -> Self {
        self.rotleft().rotleft()
    }
}

//struct Block4([[u8; 4]; 4]);

impl Block4 {
    /*
    fn from(src: &[u8], x: usize, y: usize) -> Self {
        let mut b = Block4([[0; 4]; 4]);
        let w = (src.len() as f64).sqrt().ceil() as usize;
        let off = y * w + x;
        for i in 0..4 {
            b.0[i].copy_from_slice(&src[off+i*w..off+i*w + 4]);
        }
        b
    }
    */

    fn to(&self, dst: &mut [u8], x: usize, y: usize) {
        let w = (dst.len() as f64).sqrt().ceil() as usize;
        //println!("aaa {:?} {:?}", *self, w);
        let off = y * w + x;
        for i in 0..4 {
            //println!("q {:?}", &dst[off+i*w..off+i*w + 4]);
            dst[off+i*w..off+i*w + 4].copy_from_slice(&self.0[i]);
            //println!("w {:?}", &dst[off+i*w..off+i*w + 4]);
        }
    }
}

    /*
fn expand_2to3(dest: &mut Block3, src: &mut Block2, rules: &[Rule]) {
        let mut b = Block2([[0; 2]; 2]);
        let w = (src.len() as f64).sqrt() as usize;
        let off = y * w + x;
        for i in 0..2 {
            b.0[i].copy_from_slice(&src[off+i*w..off+i*w + 2]);
        }
        b
}
*/

/*
for (d, t) in dest.iter_mut().zip(to.iter()) {
    d.copy_from_slice(t);
}
*/
fn apply_2to3(src: &Block2, from: &Block2, to: &Block3) -> Option<Block3> {
    //println!("eh2? {:?} {:?} {:?}", src, from, to);
    if *src == *from {
        Some(to.clone())
    } else if src.rotleft() == *from {
        Some(to.clone())
    } else if src.rotright() == *from {
        Some(to.clone())
    } else if src.rot180() == *from {
        Some(to.clone())
    } else if src.flip() == *from {
        Some(to.clone())
    } else if src.flip().rotleft() == *from {
        Some(to.clone())
    } else if src.flip().rotright() == *from {
        Some(to.clone())
    } else if src.flip().rot180() == *from {
        Some(to.clone())
    } else {
        None
    }
}

fn expand_2to3(src: &Block2, rules: &[Rule]) -> Block3 {
    for r in rules {
        if let &Rule2to3(from, to) = r {
            if let Some(ret) = apply_2to3(src, &from, &to) {
                return ret;
            }
        }
    }
    unreachable!()
}

fn apply_3to4(src: &Block3, from: &Block3, to: &Block4) -> Option<Block4> {
    //println!("eh3? {:?} {:?} {:?}", src, from, to);
    if *src == *from {
        Some(to.clone())
    } else if src.rotleft() == *from {
        Some(to.clone())
    } else if src.rotright() == *from {
        Some(to.clone())
    } else if src.rot180() == *from {
        Some(to.clone())
    } else if src.flip() == *from {
        Some(to.clone())
    } else if src.flip().rotleft() == *from {
        Some(to.clone())
    } else if src.flip().rotright() == *from {
        Some(to.clone())
    } else if src.flip().rot180() == *from {
        Some(to.clone())
    } else {
        None
    }
}

fn expand_3to4(src: &Block3, rules: &[Rule]) -> Block4 {
    for r in rules {
        if let &Rule3to4(from, to) = r {
            if let Some(ret) = apply_3to4(src, &from, &to) {
                return ret;
            }
        }
    }
    unreachable!()
}

fn enhance(canvas: &Vec<u8>, rules: &[Rule]) -> Vec<u8> {
    let w = (canvas.len() as f64).sqrt().ceil() as usize;
    if w % 2 == 0 {
        let newsz = w / 2 * 3;
        let mut ret = vec![b'.'; newsz * newsz];
        for gridy in 0..(w / 2) {
            for gridx in 0..(w / 2) {
                //println!("aaaaaaaaa {} {} {}", w, gridx, gridy);
                let original = Block2::from(canvas, 2 * gridx, 2 * gridy);
                let enhanced = expand_2to3(&original, rules);
                enhanced.to(&mut ret, 3 * gridx, 3 * gridy);
            }
        }
        ret
    } else if w % 3 == 0 {
        let newsz = w / 3 * 4;
        let mut ret = vec![b'?'; newsz * newsz];
        for gridy in 0..(w / 3) {
            for gridx in 0..(w / 3) {
                //println!("bbbbbbb {} {} {}", w, gridx, gridy);
                let original = Block3::from(canvas, 3 * gridx, 3 * gridy);
                let enhanced = expand_3to4(&original, rules);
                enhanced.to(&mut ret, 4 * gridx, 4 * gridy);
            }
        }
        ret
    } else {
        unreachable!()
    }
}

fn on_pixels_after(rules: &[Rule], iterations: usize) -> usize {
    let mut canvas = vec![
        b'.', b'#', b'.',
        b'.', b'.', b'#',
        b'#', b'#', b'#',
        ];

    for _i in 0..iterations {
        //println!("render loop: {} {} {}", _i, canvas.len(), (canvas.len() as f64).sqrt().ceil() as usize);
        let w = (canvas.len() as f64).sqrt().ceil() as usize;
        for row in canvas.chunks(w) {
            println!("{:?}", row.iter().map(|&x| x as char).collect::<String>());
        }
        println!("{}", canvas.iter().filter(|&&pixel| pixel == b'#').count());
        canvas = enhance(&canvas, rules);
    }

    canvas.iter().filter(|&&pixel| pixel == b'#').count()
}

fn main() {
    let rules = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    println!("{:?}", on_pixels_after(&rules, 5));
}
