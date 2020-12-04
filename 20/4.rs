use std::io::{self, Read};

fn main() {
    let mut batch_file = String::new();
    io::stdin().read_to_string(&mut batch_file).unwrap();
    batch_file.truncate(batch_file.len() - 1); // strip off last newline
    let passports: Vec<Vec<(&str, &str)>> = batch_file.split("\n\n").map(|passport_entry| {
        passport_entry.split(&[' ', '\n'][..]).map(|field| {
            let mut sp = field.split(":");
            (sp.next().unwrap(), sp.next().unwrap())
        }).collect()
    }).collect();

    let required_fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; // also cid is ok but not required
    let pp_has_field = |pp: &[(&str, &str)], field: &str| pp.iter().any(|&(ppfield, _ppval)| ppfield == field);
    let pp_is_valid = |pp: &[(&str, &str)]| required_fields.iter().all(|&field| pp_has_field(pp, field));
    let valid_passports = passports.iter().filter(|&pp| pp_is_valid(pp)).count();
    println!("{}", valid_passports);
}
