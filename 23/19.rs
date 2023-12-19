use std::io::{self, Read};
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

#[derive(Debug)]
enum Cmp {
    Lt,
    Gt,
    Nop,
}

#[derive(Debug)]
struct Rule {
    part_prop: usize, // 0123 for xmas
    cmp_type: Cmp,
    cmp: i32,
    dest: String,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

type Part = [i32; 4];

fn do_test(part: &Part, workflows: &HashMap<&str, &Workflow>, current: &str) -> bool {
    if current == "A" {
        // accept
        true
    } else if current == "R" {
        // reject
        false
    } else {
        for r in &workflows.get(current).unwrap().rules {
            if match r.cmp_type {
                Cmp::Lt => part[r.part_prop] < r.cmp,
                Cmp::Gt => part[r.part_prop] > r.cmp,
                Cmp::Nop => true,
            } {
                return do_test(part, workflows, &r.dest);
            }
        }
        panic!()
    }
}

fn test_part(part: &Part, workflows: &HashMap<&str, &Workflow>) -> bool {
    do_test(part, workflows, "in")
}

fn accepted_part_numbers(workflows: &[Workflow], parts: &[Part]) -> i32 {
    let workflows = workflows.iter()
        .map(|wf| (&wf.name as &str, wf))
        .collect::<HashMap<_, _>>();
    parts.iter()
        .filter(|p| test_part(p, &workflows))
        .map(|p| p.iter().sum::<i32>())
        .sum()
}

fn parse_workflows(inp: &str) -> Vec<Workflow> {
    // px{a<2006:qkq,m>2090:A,rfg}
    let re = Regex::new(r"([[:alpha:]]+)\{(.*),([[:alpha:]]+)\}").unwrap();
    let rule_re = Regex::new(r"([xmas])(.)([0-9]+):([[:alpha:]]+)").unwrap();

    let xmas = "xmas";
    inp.lines().map(|l| {
        let cap = re.captures(l).unwrap();
        let name = cap.get(1).unwrap().as_str().to_string();
        let mut rules = cap.get(2).unwrap()
            .as_str()
            .split(',')
            .map(|r| {
                let cap = rule_re.captures(r).unwrap();
                let part_ch = cap.get(1).unwrap().as_str().chars().nth(0).unwrap();
                let part_prop = xmas.chars().position(|c| c == part_ch).unwrap();
                let cmp_type = match cap.get(2).unwrap().as_str() {
                    "<" => Cmp::Lt,
                    ">" => Cmp::Gt,
                    _ => panic!()
                };
                let cmp = cap.get(3).unwrap().as_str().parse().unwrap();
                let dest = cap.get(4).unwrap().as_str().to_string();
                Rule { part_prop, cmp_type, cmp, dest }
            })
            .collect::<Vec<_>>();
        let fallback = cap.get(3).unwrap().as_str().to_string();
        rules.push(Rule { part_prop: 0, cmp_type: Cmp::Nop, cmp: 0, dest: fallback });
        Workflow { name, rules }
    }).collect()
}

fn parse_parts(inp: &str) -> Vec<Part> {
    // {x=787,m=2655,a=1222,s=2876}
    let re = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}").unwrap();
    inp.lines().map(|l| {
        let cap = re.captures(l).unwrap();
        let get = |i| cap.get(i).unwrap().as_str().parse().unwrap();
        [get(1), get(2), get(3), get(4)]
    }).collect()
}

fn parse(file: &str) -> (Vec<Workflow>, Vec<Part>) {
    let mut sp = file.split("\n\n");
    let workflows = parse_workflows(sp.next().unwrap());
    let parts = parse_parts(sp.next().unwrap());
    (workflows, parts)
}

fn main() {
    let mut file = String::new();
    io::stdin().read_to_string(&mut file).unwrap();
    let (workflows, parts) = parse(&file);
    println!("{}", accepted_part_numbers(&workflows, &parts));
}
