use std::io::{self, Read};

fn check_four_digits(x: &str, min: u32, max: u32) -> bool {
    match x.parse::<u32>() {
        Ok(n) => n >= min && n <= max,
        Err(..) => false,
    }
}

fn check_byr(x: &str) -> bool {
    // (Birth Year) - four digits; at least 1920 and at most 2002.
    check_four_digits(x, 1920, 2002)
}

fn check_iyr(x: &str) -> bool {
    // (Issue Year) - four digits; at least 2010 and at most 2020.
    check_four_digits(x, 2010, 2020)
}

fn check_eyr(x: &str) -> bool {
    // (Expiration Year) - four digits; at least 2020 and at most 2030.
    check_four_digits(x, 2020, 2030)
}

fn check_hgt(x: &str) -> bool {
    // (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    if !x.ends_with("cm") && !x.ends_with("in") {
        false
    } else {
        let end = &x[x.len() - 2..];
        let (min, max) = match end {
            "cm" => (150, 193),
            "in" => (59, 76),
            _ => unreachable!()
        };
        match x[..x.len() - 2].parse::<u32>() {
            Ok(n) => n >= min && n <= max,
            Err(..) => false
        }
    }
}

fn check_hcl(x: &str) -> bool {
    // (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let x = x.as_bytes();
    if x[0] != b'#' {
        false
    } else {
        x[1..].iter().all(|&c| (c >= b'0' && c <= b'9') || (c >= b'a' && c <= b'f'))
    }
}
fn check_ecl(x: &str) -> bool {
    // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    let valid_colors = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    valid_colors.iter().any(|&vc| x == vc)
}
fn check_pid(x: &str) -> bool {
    // (Passport ID) - a nine-digit number, including leading zeroes.
    match x.parse::<u32>() {
        Ok(_) => x.len() == 9,
        Err(..) => false
    }
}

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

    let field_checks: &[(&str, fn(&str) -> bool)] = &[
        ("byr", check_byr),
        ("iyr", check_iyr),
        ("eyr", check_eyr),
        ("hgt", check_hgt),
        ("hcl", check_hcl),
        ("ecl", check_ecl),
        ("pid", check_pid),
    ];
    let pp_has_valid_field = |pp: &[(&str, &str)], field: &str, chk: fn(&str) -> bool| pp.iter().any(|&(ppfield, ppval)| {
        ppfield == field && chk(ppval)
    });
    let pp_is_extra_valid = |pp: &[(&str, &str)]| field_checks.iter().all(|(field, chk)| pp_has_valid_field(pp, field, *chk));
    let extra_valid_passports = passports.iter().filter(|&pp| pp_is_extra_valid(pp)).count();
    println!("{}", extra_valid_passports);
}
