use std::io::{self, BufRead};
use std::collections::HashMap;
use std::fmt;

#[derive(Eq, PartialEq, Hash)]
struct Mass {
    name: String,
    weight: i64,
}

impl fmt::Debug for Mass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.weight, self.name)
    }
}

impl std::ops::Mul<Mass> for i64 {
    type Output = Mass;
    fn mul(self, b: Mass) -> Mass {
        Mass { name: b.name, weight: self * b.weight }
    }
}

#[derive(Debug, Hash)]
struct Reaction {
    input: Vec<Mass>,
    output: Mass,
}

fn parse_amount(spec: &str) -> Mass {
    let mut parts = spec.split(' ');

    Mass {
        weight: parts.next().unwrap().parse().unwrap(),
        name: parts.next().unwrap().to_string()
    }
}

fn parse_reaction(line: String) -> Reaction {
    let mut sides = line.split(" => ");
    let leftspec = sides.next().unwrap();
    let rightspec = sides.next().unwrap();
    let input = leftspec.split(", ").map(parse_amount).collect();
    let output = parse_amount(rightspec);

    Reaction { input, output }
}

fn div_roundup(a: i64, b: i64) -> i64 {
    (a + b - 1) / b
}

fn produce_fuel<'a>(chain: &'a [Reaction], have: &mut HashMap<&'a str, i64>,
        need: &mut HashMap<&'a str, i64>) -> Option<i64> {
    let by_name = chain.iter().map(|r| (&r.output.name as &str, r)).collect::<HashMap<&str, _>>();
    let mut ore_consumed = 0;
    need.insert("FUEL", 1);
    while !need.is_empty() {
        // any cleaner way? could also keep needed keys in another list
        let (name, weight_required) = {
            let next_name = *need.iter().next().unwrap().0;
            need.remove_entry(next_name).unwrap()
        };

        if name == "ORE" {
            // a lot but not infinitely
            let weight_stored = have.get_mut(name).unwrap();
            if *weight_stored < weight_required {
                return None;
            }
            *weight_stored -= weight_required;
            ore_consumed += weight_required;
        } else {
            let weight_stored = have.entry(name).or_insert(0);
            let weight_missing = weight_required - *weight_stored;

            // boil more if necessary
            if weight_missing > 0 {
                let reaction = by_name[name];
                let per_cycle = reaction.output.weight; // only multiples of this much can be produced
                let cycles_needed = div_roundup(weight_missing, per_cycle);
                for in_mass in &reaction.input {
                    *need.entry(&in_mass.name).or_insert(0) += cycles_needed * in_mass.weight;
                }
                *weight_stored += cycles_needed * per_cycle;
            }

            *weight_stored -= weight_required;
        }
    }
    Some(ore_consumed)
}

fn ores_for_fuel(chain: &[Reaction]) -> i64 {
    let mut have: HashMap<&str, i64> = HashMap::new();
    let mut need: HashMap<&str, i64> = HashMap::new();
    have.insert("ORE", std::i64::MAX);

    produce_fuel(chain, &mut have, &mut need).unwrap()
}

fn max_fuel_produceable(chain: &[Reaction], ore_limit: i64) -> i64 {
    let mut have: HashMap<&str, i64> = HashMap::new();
    let mut need: HashMap<&str, i64> = HashMap::new();
    have.insert("ORE", ore_limit);
    let mut fuel_produced = 0;

    while let Some(_ored) = produce_fuel(chain, &mut have, &mut need) {
        fuel_produced += 1;
        if fuel_produced & 0xffff == 0 {
            // some statistics
            println!("fuel: {} ore: {} progress: {} %",
                fuel_produced, have["ORE"], 100 - 100 * have["ORE"] / ore_limit);
        }
    }
    fuel_produced
}

fn dump_graphviz(chain: &[Reaction]) {
    println!("digraph G {{");
    for rule in chain {
        for inp in &rule.input {
            println!("{} -> {} [label=\"{}/{}\"]",
                inp.name, rule.output.name, inp.weight, rule.output.weight);
        }
    }
    println!(r"}}");
}

fn main() {
    let reaction_chain: Vec<_> = io::stdin().lock().lines().map(
        |line| parse_reaction(line.unwrap())
    ).collect();

    if false {
        dump_graphviz(&reaction_chain);
    }

    println!("{}", ores_for_fuel(&reaction_chain));
    println!("{}", max_fuel_produceable(&reaction_chain, 1000000000000));
}
