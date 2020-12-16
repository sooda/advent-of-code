use std::io::{self, BufRead};

#[derive(Debug)]
struct Rule {
    name: String,
    lo_range: (u32, u32),
    hi_range: (u32, u32),
}

type Ticket = Vec<u32>;

fn validate_field_rule(field: u32, rule: &Rule) -> bool {
    (field >= rule.lo_range.0 && field <= rule.lo_range.1) ||
        (field >= rule.hi_range.0 && field <= rule.hi_range.1)
}

fn validate_field(field: u32, rules: &[Rule]) -> bool {
    rules.iter().any(|rule| validate_field_rule(field, rule))
}

fn error_rate(ticket: &Ticket, rules: &[Rule]) -> u32 {
    ticket.iter().map(|&field| if !validate_field(field, rules) { field } else { 0 }).sum()
}

fn invalid_fields(tickets: &[Ticket], rules: &[Rule]) -> u32 {
    tickets.iter().map(|t| error_rate(t, rules)).sum()
}

fn dump(valid_per_rule: &Vec<Vec<bool>>) {
    println!("field ->, rule v");
    for fi in 0..valid_per_rule[0].len() {
        print!("\t{}", fi);
    }
    println!();
    for (ri, fields) in valid_per_rule.iter().enumerate() {
        print!("{}", ri);
        for &f_valid_for_r in fields {
            if f_valid_for_r {
                print!("\tX");
            } else {
                print!("\t.");
            }
        }
        println!();
    }
}

// this would be nicer in some matlab-y proper matrix representation
fn unique_cell(valid_per_rule: &[Vec<bool>]) -> Option<(usize, usize)> {
    // unambiguous rule with just one possible field?
    for (ri, fields) in valid_per_rule.iter().enumerate() {
        if fields.iter().filter(|&&f| f).count() == 1 {
            let fi = fields.iter().position(|&f| f).unwrap();
            return Some((ri, fi));
        }
    }

    // unambiguous field with just one possible rule?
    let n_fields = valid_per_rule[0].len();
    for fi in 0..n_fields {
        if valid_per_rule.iter().filter(|fields| fields[fi]).count() == 1 {
            let ri = valid_per_rule.iter().position(|fields| fields[fi]).unwrap();
            return Some((ri, fi));
        }
    }

    // jazz music stops
    None
}

fn eliminate(valid_per_rule: &mut [Vec<bool>], field_per_rule: &mut [Option<usize>]) -> bool {
    if let Some((rule_id, field_id)) = unique_cell(valid_per_rule) {
        println!("field {} is rule {}", field_id, rule_id);
        field_per_rule[rule_id] = Some(field_id);
        // these too would be nicer in some matlab-y proper matrix representation
        for field in &mut valid_per_rule[rule_id] {
            *field = false;
        }
        for fields in valid_per_rule {
            fields[field_id] = false;
        }
        true
    } else {
        false
    }
}

// ret[rule] says which field for that rule
fn resolve_field_mapping(tickets: &[Ticket], rules: &[Rule]) -> Vec<Option<usize>> {
    let n_fields = tickets[0].len();
    let mut valid_per_rule: Vec<Vec<bool>> = rules.iter().map(|r| {
        (0..n_fields).map(|fi| {
            tickets.iter().all(|ticket| validate_field_rule(ticket[fi], r))
        }).collect()
    }).collect();

    dump(&valid_per_rule);

    let mut field_per_rule = vec![None; rules.len()];
    while eliminate(&mut valid_per_rule, &mut field_per_rule) {
        println!("--");
        dump(&valid_per_rule);
    }
    field_per_rule
}

fn departure_product(ticket: &Ticket, rules: &[Rule], mapping: &[Option<usize>]) -> u64 {
    // the first six are departure data in my input, but check for the name just to match the spec
    if false {
        mapping.iter()
            .take(6)
            .map(|&field_id| ticket[field_id.unwrap()] as u64)
            .fold(1, |prod, field| prod * field)
    } else {
        let field_ids = rules.iter().enumerate()
            .filter(|(_i, r)| r.name.starts_with("departure"))
            .map(|(i, _r)| mapping[i]);
        // if these ids weren't found for some reason, we'd crash here.
        field_ids
            .map(|field_id| ticket[field_id.expect("mapping is incomplete")] as u64)
            .fold(1, |prod, field| prod * field)
    }
}

fn parse_rule(line: &str) -> Rule {
    let mut sp = line.split(": ");
    let name = sp.next().unwrap().to_owned();
    let range_spec = sp.next().unwrap();
    let mut ranges = range_spec.split(" or ");
    let mut lo_ranges = ranges.next().unwrap().split('-').map(|x| x.parse().unwrap());
    let mut hi_ranges = ranges.next().unwrap().split('-').map(|x| x.parse().unwrap());
    let lo_range = (lo_ranges.next().unwrap(), lo_ranges.next().unwrap());
    let hi_range = (hi_ranges.next().unwrap(), hi_ranges.next().unwrap());
    Rule {
        name,
        lo_range,
        hi_range,
    }
}

fn main() {
    let notes: Vec<_> = io::stdin().lock().lines()
        .map(|line| line.unwrap())
        .collect();
    let mut sections = notes.split(|row| row == "");
    let rules: Vec<Rule> = sections.next().unwrap().iter().map(|r| parse_rule(r)).collect();
    let own_ticket: &str = sections.next().unwrap().iter().nth(1).unwrap(); // split off the header
    let own_ticket: Ticket = own_ticket.split(',').map(|x| x.parse().unwrap()).collect();
    // also "nearby tickets:\nstuff\nstuff\n
    let nearby_tickets: Vec<Ticket> = sections.next().unwrap().iter().skip(1).map(|x| {
        x.split(',').map(|x| x.parse().unwrap()).collect::<Vec<_>>()
    }).collect();
    println!("{}", invalid_fields(&nearby_tickets, &rules));
    let fixed_tickets: Vec<Ticket> = nearby_tickets.into_iter().filter(|ticket| {
        ticket.iter().all(|&field| validate_field(field, &rules))
    }).collect();
    let mapping = resolve_field_mapping(&fixed_tickets, &rules);
    println!("{}", departure_product(&own_ticket, &rules, &mapping));
}
