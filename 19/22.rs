use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Technique {
    NewStack,
    Cut(i128),
    Increment(i128),
}
use Technique::*;

#[derive(Debug)]
struct ParseTechError {}

impl FromStr for Technique {
    type Err = ParseTechError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        Ok(match words.next().unwrap() {
            "deal" => match words.next().unwrap() {
                "into" => NewStack,
                "with" => {
                    assert!(words.next().unwrap() == "increment");
                    Increment(words.next().unwrap().parse().unwrap())
                },
                _ => panic!("bad deal"),
            },
            "cut" => Cut(words.next().unwrap().parse().unwrap()),
            _ => panic!("bad input"),
        })
    }
}

fn newstack(deck: &mut [i128]) {
    deck.reverse();
}

fn cut(deck: &mut Vec<i128>, n: i128) {
    let n = if n >= 0 { n } else { deck.len() as i128 + n } as usize;
    let top: Vec<_> = deck.drain(0..n).collect();
    deck.extend(top);
}

fn increment(deck: &mut Vec<i128>, n: i128) {
    let n = n as usize;
    let len = deck.len();
    let mut next = vec![0; len];
    let mut pos = 0;
    // bleh, iter_mut() doesn't cycle()
    for src in deck.into_iter() {
        next[pos] = *src;
        pos = (pos + n) % len;
    }
    *deck = next;
}

// now with a humongous deck, the position computation complexity cannot depend on the deck size

fn newstack_position(len: usize, card: i128) -> usize {
    len - 1 - (card as usize) // (% len not needed)
}

fn cut_position(len: usize, n: i128, card: i128) -> usize {
    assert!(n != 0);
    (len as i128 - n + card) as usize % len
}

fn increment_position(len: usize, n: i128, card: i128) -> usize {
    (card * n) as usize % len
}

const DEBUG_POSITIONS: bool = true;

fn shuffle_position(len: usize, card: i128, tech: Technique) -> usize {
    match tech {
        NewStack => newstack_position(len, card),
        Cut(n) => cut_position(len, n, card),
        Increment(n) => increment_position(len, n, card),
    }
}

fn shuffle_deck(deck: &mut Vec<i128>, tech: Technique) {
    let orig = if DEBUG_POSITIONS {
        deck.clone()
    } else {
        Vec::with_capacity(0)
    };
    match tech {
        NewStack => newstack(deck),
        Cut(n) => cut(deck, n),
        Increment(n) => increment(deck, n),
    };
    if DEBUG_POSITIONS {
        for (pos, &card) in orig.iter().enumerate() {
            let pos = pos as i128;
            let newpos = shuffle_position(deck.len(), pos, tech);
            if deck[newpos] != card {
                println!("orig {:?}", orig);
                println!("deck {:?}", deck);
                panic!("shuffle tech {:?} orig {} at {} new is {} at {}", tech, card, pos, deck[newpos], newpos);
            }
        }
    }
}

fn shuffle_all_position(len: usize, mut card: i128, steps: &[Technique]) -> usize {
    for &tech in steps {
        card = shuffle_position(len, card, tech) as i128;
    }
    card as usize
}

fn shuffle_all<'a>(deck: &'a mut Vec<i128>, steps: &[Technique]) -> &'a Vec<i128> {
    let orig = if DEBUG_POSITIONS {
        deck.clone()
    } else {
        Vec::with_capacity(0)
    };
    for &tech in steps {
        shuffle_deck(deck, tech);
    }
    if DEBUG_POSITIONS {
        for (pos, &card) in orig.iter().enumerate() {
            let pos = pos as i128;
            let newpos = shuffle_all_position(deck.len(), pos, steps);
            if deck[newpos] != card {
                println!("orig {:?}", orig);
                println!("deck {:?}", deck);
                panic!("shuffle orig {} at {} new is {} at {}", card, pos, deck[newpos], newpos);
            }
        }
    }
    deck
}

fn factory_order(n: usize) -> Vec<i128> {
    (0..(n as i128)).collect()
}

fn sampledeck(tech: Technique) -> Vec<i128> {
    let mut deck = factory_order(10);
    shuffle_deck(&mut deck, tech);
    deck
}

fn where_2019(steps: &[Technique]) -> usize {
    let mut deck = factory_order(10007);
    shuffle_all(&mut deck, steps);
    deck.into_iter().position(|card| card == 2019).unwrap()
}

fn inv(m: i128, n: i128) -> i128 {
    // assert!(is_prime(n));
    if true {
        modpow(m, n - 2, n)
    } else {
        assert!(m > 0);
        if m == 1 {
            1
        } else {
            (1 + n * (m - inv(n % m, m))) / m
        }
    }
}

fn inv_general_shuffle(a: i128, b: i128, len: i128) -> (i128, i128) {
    // c = a * x + b mod len
    // c - b = a * x mod len
    // (c - b) * inv(a) = x mod len
    // inv(a) * c + inv(a) * -b
    let i = inv(a + len, len);
    (i, i * -b)
}

/*
 * Given the shuffle process as N different aN*src+bN, come up with a combined xN*src+yN so running
 * the process once gets each source to one dest with ax+b (all are computed mod len)
 *
 * .. except the steps are inverted as well because the puzzle goes that way
 *
 *       a0 * x + b0
 * a1 * (a0 * x + b0) + b1
 *  a0 * a1 * x + a1 * b0 + b1
 * (a0, b0) combined with (a1, b1) becomes (a0 * a1, a1 * b0 + b1)
*/
fn generalize_process_backwards(steps: &[Technique], deck_size: i128) -> (i128, i128) {
    // next_idx = mult * card_idx + add
    let mut combo_mult = 1i128;
    let mut combo_add = 0i128;
    // (could also combine first and invert then)
    for &tech in steps.iter().rev() {
        let (mult, add) = match tech {
            NewStack => (-1, -1),
            Cut(n) => (1, -n),
            Increment(n) => (n, 0),
        };
        let (mult, add) = inv_general_shuffle(mult, add, deck_size);
        combo_mult = (mult * combo_mult) % deck_size;
        combo_add = (mult * combo_add + add) % deck_size;
    }

    // debug double check
    if false {
        for card in 0..deck_size {
            let next = shuffle_all_position(deck_size as usize, card as i128, steps);
            let next = shuffle_all_position(deck_size as usize, next as i128, steps);
            let card2 = (combo_mult * (next as i128) + combo_add + (deck_size*deck_size) as i128) % (deck_size as i128);
            let card2 = (combo_mult * (card2 as i128) + combo_add + (deck_size*deck_size) as i128) % (deck_size as i128);
            let magic = manyrounds(combo_mult, combo_add, next as i128, deck_size, 2);
            println!("before {} then {} = {} ~ {}", card, next, card2, magic);
            assert_eq!(card, card2);
            assert_eq!(card, magic);
        }
    }

    (combo_mult, combo_add)
}

fn modpow(mut base: i128, mut exp: i128, modu: i128) -> i128 {
    if modu == 1 {
        0
    } else {
        let mut result = 1;
        base %= modu;
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % modu;
            }
            exp >>= 1;
            base = (base * base) % modu;
        }
        result
    }
}

fn manyrounds(a: i128, b: i128, card: i128, size: i128, n: i128) -> i128 {
    /*
     * ax+b
     * a(ax+b)+b = a^2x + ab + b
     * a(a(ax+b)+b)+b = a^3x + a^2b + ab + b
     * a(a(a(ax+b)+b)+b)+b = a^4x + a^3b + a^2b + ab + b = a^4x + b(1 + a + a^2 + a^3)
     * ax+b recursively n times = a^n x + b (1 + a + a^2 + a^3 + .. + a^(n-1))
     * a^n * card + b * (1-a^n) / (1 - a)
     * a^n * card + b * (1-a^n) * inv(1 - a)
     */
    let an = modpow(a, n, size);
    println!("rounds a {} b {} card {} size {} n {} inv {}", a, b, card, size, n, inv(1 - a, size));
    let series = if an != 1 {
        (1 - an) * inv(1 - a, size)
    } else {
        1 + (n - 1) * a
    };
    println!("({} * {} + {} * {}) % {}", an, card, b, series, size);

    let mut x = (an * card) % size + (b * series) % size;
    while x < 0 {
        x += size;
    }
    x
}

fn giant_deck_2020(steps: &[Technique]) -> usize {
    let deck_size = 119_315_717_514_047i128;
    let process_repetition = 101_741_582_076_661i128;
    let card = 2020;
    let (a, b) = generalize_process_backwards(steps, deck_size);
    let magic = manyrounds(a, b, card, deck_size, process_repetition);
    println!("magic {}", magic);
    0
}

fn main() {
    assert_eq!(sampledeck(NewStack), &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    assert_eq!(sampledeck(Cut(3)), &[3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    assert_eq!(sampledeck(Cut(-4)), &[6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    assert_eq!(sampledeck(Increment(3)), &[0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    assert_eq!(shuffle_all(&mut factory_order(10),
            &[Increment(7), NewStack, NewStack]),
        &[0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    assert_eq!(shuffle_all(&mut factory_order(10),
            &[Cut(6), Increment(7), NewStack]),
        &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    assert_eq!(shuffle_all(&mut factory_order(10),
            &[Increment(7), Increment(9), Cut(-2)]),
        &[6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    assert_eq!(shuffle_all(&mut factory_order(10),
            &[NewStack, Cut(-2), Increment(7), Cut(8), Cut(-4), Increment(7),
            Cut(3), Increment(9), Increment(3), Cut(-1)]),
        &[9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);

    let steps: Vec<Technique> = io::stdin().lock().lines()
        .map(|l| l.unwrap().parse().unwrap()).collect();

    println!("{}", where_2019(&steps));
    println!("{}", giant_deck_2020(&steps));
}
