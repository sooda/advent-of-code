use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Lines;

use std::cell::Cell;

extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct Group {
    // Number of units is modified from group refs during fighting; can't hold mutable attack
    // target refs and shared attack source refs simultaneously to the same lists, so this is an ok
    // workaround.
    //
    // Alternatively a team of groups could stay immutable during one fight and the wounded groups
    // could be collected to a new mutable list (perhaps keeping the other group attributes in
    // another "character class" type that would be referenced to avoid unnecessary copies). Might
    // work as well but then the new copy of groups should be looked at to determine whether the
    // about-to-attack group ended up in fact dead in a previous attack of the same fight.
    units: Cell<i32>,

    // it's useful to sort by some of these in inverse order, so the sign bit is a nice feature
    unit_hitpts: i32,
    attack_pts: i32,
    attack_type: String,
    initiative: i32,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
}

#[derive(Debug)]
struct Game {
    // could also contain all units in a single list, with types for immune system groups and
    // infenction groups separately; in that case, would iterate just that single list but would
    // filter attacking/defending groups by type each time
    immune_system: Vec<Group>,
    infection: Vec<Group>,
}

impl Group {
    fn effective_power(&self) -> i32 {
        self.units.get() * self.attack_pts
    }

    fn alive(&self) -> bool {
        self.units.get() > 0
    }

    fn unit_count(&self) -> i32 {
        self.units.get()
    }

    fn attack_damage(&self, target: &Group) -> i32 {
        let weak = target.weaknesses.contains(&self.attack_type);
        let immune = target.immunities.contains(&self.attack_type);
        if weak {
            assert!(!immune);
            self.effective_power() * 2
        } else if immune {
            assert!(!weak);
            0
        } else {
            self.effective_power()
        }
    }

    // would be real nice if target could be &mut Group to document that it's modified, ugh
    fn affect_damage(&self, target: &Group) {
        let damage = self.attack_damage(target);
        let killed_whole_units = damage / target.unit_hitpts;
        // Cell::update() is nightly :(
        let current_units = target.units.get();
        let remaining_units = if current_units > killed_whole_units {
            current_units - killed_whole_units
        } else {
            0
        };
        target.units.set(remaining_units);
    }
}

// 389 units each with 13983 hit points (immune to bludgeoning) with an attack that does 256 cold damage at initiative 13
// 1827 units each with 5107 hit points with an attack that does 24 slashing damage at initiative 18
// 7019 units each with 2261 hit points (immune to radiation, slashing, cold) with an attack that does 3 fire damage at initiative 16
// 491 units each with 3518 hit points (weak to cold; immune to fire, bludgeoning) with an attack that does 65 radiation damage at initiative 1
// 411 units each with 6375 hit points (immune to slashing; weak to cold, fire) with an attack that does 151 bludgeoning damage at initiative 14
fn parse_group(line: &str) -> Group {
    // lol, takes ages to compile this again and again apparently but it's fine
    let re = Regex::new(r"(\d+) units each with (\d+) hit points (\((weak|immune) to ([a-z, ]+)(; (weak|immune) to ([a-z, ]+))?\) )?with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)").unwrap();
    let cap = re.captures(line).unwrap();

    let units = cap.get(1).unwrap().as_str().parse().unwrap();
    let hitpts = cap.get(2).unwrap().as_str().parse().unwrap();

    let (weaknesses, immunities) = if cap.get(3).is_some() { // group 3 describes these
        let first_kind = cap.get(4).unwrap().as_str(); // "weak" or "immune"
        let first_types = cap.get(5).unwrap().as_str(); // if 3 exists, 4 and 5 do too

        // cap.get(7) is "immune" or "weak" or nonexistent
        let (second_kind, second_types) = if let Some(second_kind) = cap.get(7) {
            let second_kind = second_kind.as_str();
            let second_types = cap.get(8).unwrap().as_str(); // if 7 exists, 8 exists too
            (second_kind, second_types)
        } else {
            (if first_kind == "weak" { "immune" } else { "weak" }, "")
        };

        match first_kind {
            "weak" => {
                assert!(second_kind == "immune");
                (first_types, second_types)
            },
            "immune" => {
                assert!(second_kind == "weak");
                (second_types, first_types)
            }
            _ => panic!()
        }
    } else {
        ("", "")
    };
    let attack_pts = cap.get(9).unwrap().as_str().parse().unwrap();
    let attack_type = cap.get(10).unwrap().as_str().to_owned();
    let initiative = cap.get(11).unwrap().as_str().parse().unwrap();

    let cloned = |s: &str| s.to_owned();
    Group {
        units: Cell::new(units),
        unit_hitpts: hitpts,
        attack_pts: attack_pts,
        attack_type: attack_type,
        initiative: initiative,
        weaknesses: weaknesses.split(", ").map(cloned).collect(),
        immunities: immunities.split(", ").map(cloned).collect(),
    }
}

fn parse_team(input: &mut Lines<BufReader<File>>) -> Vec<Group> {
    let parse_line = |line: String| {
        if line != "" {
            Some(parse_group(&line))
        } else {
            // a group ends either with an empty line or eof
            None
        }
    };

    let mut army = vec![];

    while let Some(group) = input.next().and_then(|l| parse_line(l.unwrap())) {
        army.push(group);
    }

    army
}

// this map lives at least as long as the opposite teams
type TargetMap<'a> = Vec<(&'a Group, &'a Group)>;

fn select_targets<'a>(team: &'a [Group], enemies: &'a [Group]) -> TargetMap<'a> {
    let mut selection_order = team.iter().filter(|grp| grp.alive()).collect::<Vec<_>>();
    // negative for best first
    selection_order.sort_unstable_by_key(|grp| (-grp.effective_power(), -grp.initiative));
    let selection_order = selection_order;
    let mut target_order = TargetMap::new();

    // refs for mutating and for the target map
    let mut enemies_remaining = enemies.iter().collect::<Vec<_>>();

    for attacker in selection_order {
        let ordering_key = |(_, receiver): &(usize, &&Group)| (
            attacker.attack_damage(receiver),
            receiver.effective_power(),
            receiver.initiative
        );
        // no target chosen if cannot deal dmg to any group
        let alive_attackable = |(_, receiver): &(usize, &&Group)|
            receiver.alive() && attacker.attack_damage(receiver) > 0;

        // Vec::remove() takes an index, Vec::remove_item() is nightly :(
        if let Some((pos, &selected)) = enemies_remaining.iter().enumerate()
                .filter(alive_attackable).max_by_key(ordering_key) {
            target_order.push((attacker, selected));
            enemies_remaining.remove(pos);
        }
    }

    target_order
}

fn fight(mut attacks: TargetMap) {
    attacks.sort_unstable_by_key(|(attacker, _)| -attacker.initiative);

    for (attacker, defender) in attacks {
        if false {
            println!("he attak");
            println!("    {:?}", attacker);
            println!("he receiv");
            println!("    {:?}", defender);
        }
        // the attacker can die in a previous attack in this fight
        if attacker.alive() {
            attacker.affect_damage(defender);
        }
    }
}

fn round(game: &mut Game) {
    let mut immu_targets = select_targets(&game.immune_system, &game.infection);
    let mut infe_targets = select_targets(&game.infection, &game.immune_system);
    let mut attacks = Vec::new();
    attacks.append(&mut immu_targets);
    attacks.append(&mut infe_targets);
    fight(attacks);
}

fn play(game: &mut Game) -> i32 {
    while game.immune_system.iter().any(Group::alive) && game.infection.iter().any(Group::alive) {
        if false {
            println!("immu:");
            for g in &game.immune_system {
                println!("    {:?}", g);
            }
            println!("infe:");
            for g in &game.infection {
                println!("    {:?}", g);
            }
            println!("");
        }
        round(game);
    }
    // no need to find out which one won; the dead has no units left
    game.immune_system.iter().chain(game.infection.iter()).map(|grp| grp.unit_count()).sum()
}

fn parse_game(input: &mut Lines<BufReader<File>>) -> Game {
    let immu_title = input.next().unwrap().unwrap();
    assert!(immu_title == "Immune System:");
    let immu_army = parse_team(input);

    let infe_title = input.next().unwrap().unwrap();
    assert!(infe_title == "Infection:");
    let infe_army = parse_team(input);
    Game { immune_system: immu_army, infection: infe_army }
}

fn main() {
    let mut input = BufReader::new(File::open(&std::env::args().nth(1).unwrap()).unwrap()).lines();
    let mut game = parse_game(&mut input);
    println!("{}", play(&mut game));
}
