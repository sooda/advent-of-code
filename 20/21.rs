use std::io::{self, BufRead};
use std::collections::hash_map::{HashMap, Entry};
use std::collections::HashSet;

struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

// ugh, I want a HashMap::remove that takes a closure and doesn't need to hash because we already
// have the entry from the iteration
fn remove_by<K, V, P>(hashmap: &mut HashMap<K, V>, predicate: P) -> Option<(K, V)>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
    P: FnMut(&(&K, &V)) -> bool
{
    let entry: Option<_> = hashmap.iter().find(predicate);
    // throw away the refs because hashmap will get modified
    let cloned_key = entry.map(|(k, _v)| k.clone());
    // must clone k even though remove_entry takes just a ref because map will get modified :( most
    // likely it'll be just hashed, maybe compared and discarded before the actual removal, so this
    // shouldn't be necessary
    cloned_key.map(|k| hashmap.remove_entry(&k).expect("just got this from entry??"))
}

// which allergen is which ingredient
fn final_mapping(mut plausible_ingredients: HashMap<String, HashSet<String>>) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let unique = |(_k, v): &(&String, &HashSet<String>)| v.len() == 1;

    while let Some((allergen, mut ingrs)) = remove_by(&mut plausible_ingredients, unique) {
        let ingredient = ingrs.drain().next().unwrap();
        for (_alle, ings) in plausible_ingredients.iter_mut() {
            ings.remove(&ingredient);
        }
        out.push((allergen, ingredient));
    }

    // no ambiguous ingredients should remain
    assert!(plausible_ingredients.is_empty());
    out
}

fn parse_food(line: &str) -> Food {
    let mut sp = line.strip_suffix(")").unwrap().split(" (contains ");
    let ingredients = sp.next().unwrap().split(' ').map(|s| s.to_owned()).collect();
    let allergens = sp.next().unwrap().split(", ").map(|s| s.to_owned()).collect();
    Food { ingredients, allergens }
}

fn main() {
    let foods: Vec<_> = io::stdin().lock().lines()
        .map(|line| parse_food(&line.unwrap()))
        .collect();

    let all_allergens = foods.iter().fold(HashSet::new(), |mut alls, food| {
        alls.extend(food.allergens.iter());
        alls
    });

    // from allergen to all possible ingredient names
    let mut plausible_ingred: HashMap<String, HashSet<String>> = HashMap::new();

    // intersect: each allergen can only be one of the ingredients that exist in all foods where
    // the allergen is mentioned
    for &allergen in &all_allergens {
        for food in &foods {
            if food.allergens.contains(allergen) {
                match plausible_ingred.entry(allergen.clone()) {
                    Entry::Vacant(e) => {
                        e.insert(food.ingredients.clone());
                    },
                    Entry::Occupied(mut e) => {
                        let filtered: HashSet<String> = e.get()
                            .intersection(&food.ingredients)
                            .cloned().collect();
                        *e.get_mut() = filtered;
                    },
                }
            }
        }
    }

    let dangerous_ingredients: HashSet<String> = plausible_ingred.values()
        .fold(HashSet::new(), |mut dangerous, next| {
            dangerous.extend(next.iter().cloned());
            dangerous
        });
    println!("dangerous: {:?}", dangerous_ingredients);

    let all_ingredients: HashSet<String> = foods.iter()
        .fold(HashSet::new(), |mut ingreds, food| {
            ingreds.extend(food.ingredients.iter().cloned());
            ingreds
        });

    let safe_ingredients: HashSet<String> = all_ingredients.difference(&dangerous_ingredients)
        .cloned().collect();

    let safe_occurrences = foods.iter().map(|food| {
        food.ingredients.intersection(&safe_ingredients).count()
    }).sum::<usize>();
    println!("safe times: {}", safe_occurrences);

    println!("initially plausible:");
    for a in &plausible_ingred {
        println!("{:?}", a);
    }

    let mut final_mapping: Vec<(String, String)> = final_mapping(plausible_ingred);
    final_mapping.sort_unstable();
    let ingredient_list: Vec<String> = final_mapping.into_iter().map(|(_k, v)| v).collect();
    let canonical_dangerous_ingredient_list = ingredient_list.join(",");
    println!("{}", canonical_dangerous_ingredient_list);

    // dtb,zgk,pxr,cqnl,xkclg,xtzh,jpnv,lsvlx
}
