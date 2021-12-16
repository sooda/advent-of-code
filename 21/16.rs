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
    let mut substream = stream.substream(bitnum as usize);
    while substream.available() {
        sum += version_sum_recursive(&mut substream);
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

fn values_operator_bitnum(stream: &mut Bitstream) -> Vec<u64> {
    let bitnum = stream.extract_bits(15);
    let mut values = Vec::new();
    let mut substream = stream.substream(bitnum as usize);
    while substream.available() {
        values.push(compute_recursive(&mut substream));
    }
    values
}

fn values_operator_pktcount(stream: &mut Bitstream) -> Vec<u64> {
    let pktcount = stream.extract_bits(11);
    let mut values = Vec::new();
    for _ in 0..pktcount {
        values.push(compute_recursive(stream));
    }
    values
}

fn values_operator(stream: &mut Bitstream) -> Vec<u64> {
    let length_type_id = stream.extract_bits(1);
    match length_type_id {
        0 => values_operator_bitnum(stream),
        1 => values_operator_pktcount(stream),
        _ => unreachable!()
    }
}

fn value_literal(stream: &mut Bitstream) -> u64 {
    let mut val = 0;
    loop {
        let group = stream.extract_bits(5);
        val <<= 4;
        val |= group & 0b1111;
        if (group & 0b10000) == 0 {
            break;
        }
    }
    val
}

fn compute_recursive(stream: &mut Bitstream) -> u64 {
    let _ver = stream.extract_bits(3);
    let type_id = stream.extract_bits(3);
    match type_id {
        4 => value_literal(stream),
        0 | 1 | 2 | 3 | 5 | 6 | 7 => {
            let values = values_operator(stream);
            match type_id {
                0 => values.iter().sum(),
                1 => values.iter().product(),
                2 => *values.iter().min().unwrap(),
                3 => *values.iter().max().unwrap(),
                5 => (values[0] > values[1]) as u64,
                6 => (values[0] < values[1]) as u64,
                7 => (values[0] == values[1]) as u64,
                _ => unreachable!()
            }
        },
        _ => unreachable!()
    }
}

fn compute(bits: &[u8]) -> u64 {
    let mut stream = Bitstream::new(bits);
    compute_recursive(&mut stream)
}

fn main() {
    let bits: Vec<u8> = io::stdin().lock().lines()
        .next().unwrap().unwrap()
        .chars().map(|b| "0123456789ABCDEF".chars().position(|c| c == b).unwrap() as u8) // or from_str_radix
        .collect();
    println!("{}", version_sum(&bits));
    println!("{}", compute(&bits));
}
