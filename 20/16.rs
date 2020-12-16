use std::io::{self, BufRead};

#[derive(Debug)]
struct Rule {
    lo_range: (u32, u32),
    hi_range: (u32, u32),
}

type Ticket = Vec<u32>;

fn validate_field(field: u32, rules: &[Rule]) -> bool {
    rules.iter().any(|rule|
              (field >= rule.lo_range.0 && field <= rule.lo_range.1) ||
              (field >= rule.hi_range.0 && field <= rule.hi_range.1))
}

fn error_rate(ticket: &Ticket, rules: &[Rule]) -> u32 {
    ticket.iter().map(|&field| if !validate_field(field, rules) { field } else { 0 }).sum()
}

fn invalid_fields(tickets: &[Ticket], rules: &[Rule]) -> u32 {
    tickets.iter().map(|t| error_rate(t, rules)).sum()
}

fn parse_rule(line: &str) -> Rule {
    let range_spec = line.split(": ").nth(1).unwrap();
    let mut ranges = range_spec.split(" or ");
    let mut lo_ranges = ranges.next().unwrap().split('-').map(|x| x.parse().unwrap());
    let mut hi_ranges = ranges.next().unwrap().split('-').map(|x| x.parse().unwrap());
    let lo_range = (lo_ranges.next().unwrap(), lo_ranges.next().unwrap());
    let hi_range = (hi_ranges.next().unwrap(), hi_ranges.next().unwrap());
    Rule {
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
}
