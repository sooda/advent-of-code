fn bake(recipe_count: usize) -> usize {
    let mut recipes = vec![3, 7];
    let mut alice = 0;
    let mut bob = 1;

    while recipes.len() < recipe_count + 10 {
        let sum = recipes[alice] + recipes[bob];
        if sum >= 10 {
            recipes.push(sum / 10);
        }
        recipes.push(sum % 10);
        alice = (alice + 1 + recipes[alice]) % recipes.len();
        bob = (bob + 1 + recipes[bob]) % recipes.len();
    }
    // need to take separately; the last iteration could have inserted one extra
    recipes.iter().skip(recipe_count).take(10).fold(0, |acc, x| 10 * acc + x)
}

fn main() {
    assert!(bake(9) == 5158916779);
    assert!(bake(5) == 0124515891);
    assert!(bake(18) == 9251071085);
    assert!(bake(2018) == 5941429882);
    let puzzle_input = 165061;
    println!("{:10}", bake(puzzle_input));
}
