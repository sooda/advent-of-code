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

fn bake_more(input: usize, sz: usize) -> usize {
    let mut recipes = vec![3u8, 7u8];
    let mut alice = 0;
    let mut bob = 1;

    let endscore = |recipes: &[u8]| {
        recipes.iter().skip(recipes.len() - sz).fold(0, |acc, &x| 10 * acc + (x as usize))
    };

    loop {
        let sum = recipes[alice] + recipes[bob];
        if sum >= 10 {
            recipes.push(sum / 10);
            if endscore(&recipes) == input {
                break;
            }
        }
        recipes.push(sum % 10);
        if endscore(&recipes) == input {
            break;
        }
        alice = (alice + 1 + recipes[alice] as usize) % recipes.len();
        bob = (bob + 1 + recipes[bob] as usize) % recipes.len();
        if recipes.len() % 100 == 0 {
        }
    }

    recipes.len() - sz
}

fn main() {
    assert!(bake(9) == 5158916779);
    assert!(bake(5) == 0124515891);
    assert!(bake(18) == 9251071085);
    assert!(bake(2018) == 5941429882);
    let puzzle_input = 165061;
    let puzzle_digits = 6;
    println!("{:10}", bake(puzzle_input));
    assert!(bake_more(51589, 5) == 9);
    assert!(bake_more( 1245, 5) == 5);
    assert!(bake_more(92510, 5) == 18);
    assert!(bake_more(59414, 5) == 2018);
    println!("{}", bake_more(puzzle_input, puzzle_digits));
}
