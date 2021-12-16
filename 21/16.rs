use std::io::{self, BufRead};

const BITS_PER_ELEM: usize = 4;
const FULL_ELEM_MASK: u8 = (1 << BITS_PER_ELEM) - 1;
struct Bitstream<'a> {
    chs: &'a [u8],
    epos: usize,
    bpos: usize,
    maxlen: usize,
}

impl<'a> Bitstream<'a> {
    fn new(chs: &'a [u8]) -> Self {
        Self { chs, epos: 0, bpos: 0, maxlen: chs.len() * BITS_PER_ELEM }
    }

    fn extract_bits(&mut self, n: usize) -> u64 {
        let mut remaining = n;
        let mut out = 0u64;
        while remaining > 0 {
            let in_this_elem = BITS_PER_ELEM - self.bpos;
            let from_this_elem = remaining.min(in_this_elem);
            let mask = FULL_ELEM_MASK >> self.bpos;
            let shift = BITS_PER_ELEM - from_this_elem - self.bpos;

            out <<= from_this_elem;
            out |= ((self.chs[self.epos] & mask) >> shift) as u64;
            remaining -= from_this_elem;
            self.bpos += from_this_elem;
            if self.bpos == BITS_PER_ELEM {
                self.bpos = 0;
                self.epos += 1;
            }
        }
        out
    }

    fn substream(&mut self, len: usize) -> Bitstream {
        let out = Bitstream {
            chs: self.chs,
            epos: self.epos,
            bpos: self.bpos,
            maxlen: self.epos * BITS_PER_ELEM + self.bpos + len
        };
        self.bpos += len;
        self.epos += self.bpos / BITS_PER_ELEM;
        self.bpos = self.bpos % BITS_PER_ELEM;
        out
    }

    fn available(&self) -> bool {
        self.epos * BITS_PER_ELEM + self.bpos < self.maxlen
    }
}

fn version_sum_operator_bitnum(stream: &mut Bitstream) -> u64 {
    let bitnum = stream.extract_bits(15);
    let mut sum = 0;
    {
        let mut substream = stream.substream(bitnum as usize);
        while substream.available() {
            sum += version_sum_recursive(&mut substream);
        }
    }
    sum
}

fn version_sum_operator_pktcount(stream: &mut Bitstream) -> u64 {
    let pktcount = stream.extract_bits(11);
    let mut sum = 0;
    for _ in 0..pktcount {
        sum += version_sum_recursive(stream);
    }
    sum
}

fn version_sum_operator(stream: &mut Bitstream) -> u64 {
    let length_type_id = stream.extract_bits(1);
    match length_type_id {
        0 => version_sum_operator_bitnum(stream),
        1 => version_sum_operator_pktcount(stream),
        _ => unreachable!()
    }
}

fn version_sum_literal(stream: &mut Bitstream) {
    loop {
        let group = stream.extract_bits(5);
        if (group & 0b10000) == 0 {
            break;
        }
    }
}

fn version_sum_recursive(stream: &mut Bitstream) -> u64 {
    let ver = stream.extract_bits(3);
    let type_id = stream.extract_bits(3);
    match type_id {
        4 => { version_sum_literal(stream); ver},
        _operator_type => ver + version_sum_operator(stream),
    }
}

fn version_sum(bits: &[u8]) -> u64 {
    let mut stream = Bitstream::new(bits);
    version_sum_recursive(&mut stream)
}

fn main() {
    let bits: Vec<u8> = io::stdin().lock().lines()
        .next().unwrap().unwrap()
        .chars().map(|b| "0123456789ABCDEF".chars().position(|c| c == b).unwrap() as u8) // or from_str_radix
        .collect();
    println!("{}", version_sum(&bits));
}
