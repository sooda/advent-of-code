// compile with either:
// rustc --cfg 'csimode="copypasta"'
// or:
// rustc --cfg 'csimode="fancy"'
// see further below for experiments.

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

// Square block of pixels, implemented as an array. A vector-based block with runtime-deduceable
// size might require less code, but I just wanted to see how this gets overengineered. And plain
// arrays might optimize better, and even a 4x4 array of u8 is just 128 bits so they might even fit
// in registers.
//
// And really u8 is unnecessary waste because just single bits could be used for these black-white
// pieces of art. Rotations and flips of bit patterns would be fun to write as just number lookups
// (4x4 has just 16 bits so it's doable) or even with arithmetic, but gotta stop somewhere... Now
// this would generalize to multi-color art.
pub trait Block: std::marker::Sized + PartialEq + Eq + Copy + Clone {
    const N: usize;
    fn new() -> Self;
    fn row_mut(&mut self, i: usize) -> &mut [u8];
    fn row(&self, i: usize) -> &[u8];
    fn from_canvas(src: &[u8], x: usize, y: usize) -> Self {
        let mut b = Self::new();
        let w = (src.len() as f64).sqrt().ceil() as usize;
        let off = y * w + x;
        for i in 0..Self::N {
            let pos = off + i * w;
            b.row_mut(i).copy_from_slice(&src[pos .. pos + Self::N]);
        }
        b
    }

    fn parse(src: &[u8]) -> Self {
        let mut b = Self::new();
        // ../.., .../.../...
        for i in 0..Self::N {
            let pos = i * (Self::N + 1);
            b.row_mut(i).copy_from_slice(&src[pos .. pos + Self::N]);
        }
        b
    }

    fn to_canvas(&self, dst: &mut [u8], x: usize, y: usize) {
        let w = (dst.len() as f64).sqrt().ceil() as usize;
        let off = y * w + x;
        for i in 0..Self::N {
            let pos = off + i * w;
            dst[pos .. pos + Self::N].copy_from_slice(self.row(i));
        }
    }

    fn transform<F>(&self, f: F) -> Self
        where F: Fn(usize, usize) -> (usize, usize) {
        let mut b = Self::new();

        for y in 0..Self::N {
            for x in 0..Self::N {
                let src = f(x, y);
                b.row_mut(y)[x] = self.row(src.1)[src.0];
            }
        }

        b
    }
    fn flip(&self) -> Self {
        /*
         * 012    210
         * 345 -> 543
         * 678    876
         */
        self.transform(|x, y| (Self::N - 1 - x, y))
    }
    fn rotleft(&self) -> Self {
        /*
         * 012    258
         * 345 -> 147
         * 678    036
         */
        self.transform(|x, y| (Self::N - 1 - y, x))
    }
    fn rotright(&self) -> Self {
        self.rotleft().rotleft().rotleft()
    }
    fn rot180(&self) -> Self {
        self.rotleft().rotleft()
    }
}

// to_canvas() isn't necessary for size 2 because they are only expanded, not written;
// from_canvas() and rotations aren't necessary for size 4 because they aren't expanded further.
// However, some are common, so they're all here.
macro_rules! implement_block {
    ($name:ident, $n: expr) => {
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub struct $name([[u8; $n]; $n]);
        impl Block for $name {
            const N: usize = $n;
            fn new() -> Self { $name([[0; $n]; $n]) }
            fn row_mut(&mut self, i: usize) -> &mut [u8] { &mut self.0[i] }
            fn row(&self, i: usize) -> &[u8] { &self.0[i] }
        }
    };
}

implement_block!(Block2, 2);
implement_block!(Block3, 3);
implement_block!(Block4, 4);

#[derive(Debug)]
pub enum Rule {
    Rule2to3(Block2, Block3),
    Rule3to4(Block3, Block4)
}

fn parse_line(line: &str) -> Rule {
    // This could be generalized more but let's keep even this part understandable
    let bytes = line.as_bytes();
    let two_pattern_len = "../..".len();
    let three_pattern_len = ".../.../...".len();
    let four_pattern_len = "..../..../..../....".len();
    let arrow_len = " => ".len();
    if line.len() == "../.. => ###/.##/#..".len() {
        let from = Block2::parse(&bytes[0..two_pattern_len]);
        let to = Block3::parse(&bytes[two_pattern_len + arrow_len..
                                    two_pattern_len + arrow_len + three_pattern_len]);
        Rule::Rule2to3(from, to)
    } else if line.len() == ".../.../... => #.#./#..#/#.##/#.#.".len() {
        let from = Block3::parse(&bytes[0..three_pattern_len]);
        let to = Block4::parse(&bytes[three_pattern_len + arrow_len..
                                    three_pattern_len + arrow_len + four_pattern_len]);
        Rule::Rule3to4(from, to)
    } else {
        unreachable!()
    }
}

// Could just combine flips and single rotlefts until we've found all combinations, but hard-coding
// all of the eight variations here is good enough.
pub fn apply_upscale<Src, Dst>(src: &Src, from: &Src, to: &Dst) -> Option<Dst>
where Src: Block, Dst: Block {
    if *src == *from
        || src.rotleft() == *from
        || src.rotright() == *from
        || src.rot180() == *from
        || src.flip() == *from
        || src.flip().rotleft() == *from
        || src.flip().rotright() == *from
        || src.flip().rot180() == *from {
        Some(to.clone())
    } else {
        None
    }
}

pub mod csi_copypasta {
use crate::{Block, Block2, Block3, Block4, Rule, apply_upscale};

fn expand_2to3(src: &Block2, rules: &[Rule]) -> Block3 {
    for r in rules {
        if let &Rule::Rule2to3(from, to) = r {
            if let Some(ret) = apply_upscale(src, &from, &to) {
                return ret;
            }
        }
    }
    unreachable!()
}

fn expand_3to4(src: &Block3, rules: &[Rule]) -> Block4 {
    for r in rules {
        if let &Rule::Rule3to4(from, to) = r {
            if let Some(ret) = apply_upscale(src, &from, &to) {
                return ret;
            }
        }
    }
    unreachable!()
}

pub fn enhance(canvas: &Vec<u8>, rules: &[Rule]) -> Vec<u8> {
    let w = (canvas.len() as f64).sqrt().ceil() as usize;
    if w % 2 == 0 {
        let newsz = w / 2 * 3;
        let mut ret = vec![b'?'; newsz * newsz];
        for gridy in 0..(w / 2) {
            for gridx in 0..(w / 2) {
                let original = Block2::from_canvas(canvas, 2 * gridx, 2 * gridy);
                let enhanced = expand_2to3(&original, rules);
                enhanced.to_canvas(&mut ret, 3 * gridx, 3 * gridy);
            }
        }
        ret
    } else if w % 3 == 0 {
        let newsz = w / 3 * 4;
        let mut ret = vec![b'?'; newsz * newsz];
        for gridy in 0..(w / 3) {
            for gridx in 0..(w / 3) {
                let original = Block3::from_canvas(canvas, 3 * gridx, 3 * gridy);
                let enhanced = expand_3to4(&original, rules);
                enhanced.to_canvas(&mut ret, 4 * gridx, 4 * gridy);
            }
        }
        ret
    } else {
        unreachable!()
    }
}

} // csi_copypasta

mod csi_fancy {
use crate::{Block, Block2, Block3, Block4, Rule, apply_upscale};

// rule execution for enhancing (expanding) a single cell in a canvas grid
trait Expansion {
    type From: Block;
    type To: Block;
    fn destruct_rule(r: &Rule) -> Option<(Self::From, Self::To)>;
}

struct Exp2to3 {}

impl Expansion for Exp2to3 {
    type From = Block2;
    type To = Block3;
    fn destruct_rule(r: &Rule) -> Option<(Self::From, Self::To)> {
        if let &Rule::Rule2to3(from, to) = r {
            Some((from, to))
        } else {
            None
        }
    }
}

struct Exp3to4 {}

impl Expansion for Exp3to4 {
    type From = Block3;
    type To = Block4;
    fn destruct_rule(r: &Rule) -> Option<(Self::From, Self::To)> {
        if let &Rule::Rule3to4(from, to) = r {
            Some((from, to))
        } else {
            None
        }
    }
}

fn expand<E>(src: &E::From, rules: &[Rule]) -> E::To
where E: Expansion {
    for r in rules {
        // I wish I could "if let &E::RuleValue(from, to)" somehow to avoid destruct_rule
        if let Some((from, to)) = E::destruct_rule(&r) {
            if let Some(ret) = apply_upscale(src, &from, &to) {
                return ret;
            }
        }
    }
    unreachable!()
}

fn enhance_grid_cells<E>(canvas: &Vec<u8>, rules: &[Rule]) -> Vec<u8>
where E: Expansion {
    let w = (canvas.len() as f64).sqrt().ceil() as usize;
    let newsz = w / E::From::N * E::To::N;

    let mut ret = vec![b'?'; newsz * newsz];

    for gridy in 0..(w / E::From::N) {
        for gridx in 0..(w / E::From::N) {
            let original = E::From::from_canvas(canvas, E::From::N * gridx, E::From::N * gridy);
            // why no inference? expand(&original, rules) doesn't compile
            let enhanced = expand::<E>(&original, rules);
            enhanced.to_canvas(&mut ret, E::To::N * gridx, E::To::N * gridy);
        }
    }

    ret
}

pub fn enhance(canvas: &Vec<u8>, rules: &[Rule]) -> Vec<u8> {
    let w = (canvas.len() as f64).sqrt().ceil() as usize;

    if w % 2 == 0 {
        enhance_grid_cells::<Exp2to3>(canvas, rules)
    } else if w % 3 == 0 {
        enhance_grid_cells::<Exp3to4>(canvas, rules)
    } else {
        unreachable!()
    }
}

} // csi_fancy

#[cfg(csimode = "copypasta")]
use csi_copypasta::enhance;

#[cfg(csimode = "fancy")]
use csi_fancy::enhance;

fn on_pixels_after(rules: &[Rule], iterations: usize) -> usize {
    let mut canvas = vec![
        b'.', b'#', b'.',
        b'.', b'.', b'#',
        b'#', b'#', b'#',
        ];

    for _i in 0..iterations {
        //println!("render loop: {} {} {}", _i, canvas.len(), (canvas.len() as f64).sqrt().ceil() as usize);
        let w = (canvas.len() as f64).sqrt().ceil() as usize;
        //for row in canvas.chunks(w) {
        //    println!("{:?}", row.iter().map(|&x| x as char).collect::<String>());
        //}
        println!("{} {} {}", _i, w, canvas.iter().filter(|&&pixel| pixel == b'#').count());
        canvas = enhance(&canvas, rules);
    }

    canvas.iter().filter(|&&pixel| pixel == b'#').count()
}

fn main() {
    let rules = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap())
        .lines().map(|x| parse_line(&x.unwrap())).collect::<Vec<_>>();
    println!("{:?}", on_pixels_after(&rules, 5));
    println!("{:?}", on_pixels_after(&rules, 18));
}
