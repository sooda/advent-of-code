use std::io::{self, Read};

#[derive(Clone, Copy, PartialEq)]
enum SchemKind { Key, Lock }
use SchemKind::*;
type Schematic = (SchemKind, [i8; 5]);

fn pairs_fit(schematics: &[Schematic]) -> usize {
    let keys = schematics.iter().filter(|s| s.0 == Key).copied().collect::<Vec<_>>();
    let locks = schematics.iter().filter(|s| s.0 == Lock).copied().collect::<Vec<_>>();
    let mut fit = 0;
    for k in keys {
        for l in &locks {
            if k.1.iter().zip(l.1.iter()).all(|(kk, ll)| kk + ll <= 7) {
                fit += 1;
            }
        }
    }
    fit
}

fn parse(s: &str) -> Schematic {
    // this makes both the locks and keys have one column more than in the puzzle spec, but it's
    // consistent and thus trivial to compensate for (>7 overflows, not >5).
    let columns = s.split('\n').fold([0; 5], |mut acc, line| {
            line.chars().enumerate().for_each(|(i, ch)| if ch == '#' {
                acc[i] += 1;
            } else {
                assert_eq!(ch, '.');
            });
            acc
        });
    if s.starts_with('#') {
        (Lock, columns)
    } else {
        (Key, columns)
    }
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let schematics = file.split("\n\n")
        .map(|s| parse(s))
        .collect::<Vec<_>>();
    println!("{}", pairs_fit(&schematics));
}
