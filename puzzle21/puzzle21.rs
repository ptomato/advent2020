use std::collections::{HashMap, HashSet};
use std::env;

#[derive(Clone, Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Food {
    fn from_string(s: &str) -> Self {
        let mut split1 = s[0..s.len() - 1].split(" (contains ");
        let (ingredients_list, allergens_list) = (split1.next().unwrap(), split1.next().unwrap());
        let ingredients = ingredients_list.split(' ').map(String::from).collect();
        let allergens = allergens_list.split(", ").map(String::from).collect();
        Food {
            ingredients,
            allergens,
        }
    }
}

fn find_possible_allergens(foods: &[Food]) -> HashMap<String, HashSet<String>> {
    let all_allergens: HashSet<_> = foods
        .iter()
        .flat_map(|food| food.allergens.clone())
        .collect();
    let all_ingredients: Vec<_> = foods
        .iter()
        .flat_map(|food| food.ingredients.clone())
        .collect();
    let mut possible_allergens: HashMap<_, _> = all_ingredients
        .iter()
        .map(|ingredient| (ingredient.clone(), all_allergens.clone()))
        .collect();
    for food in foods {
        for ingredient in all_ingredients
            .iter()
            .filter(|&ingredient| !food.ingredients.contains(ingredient))
        {
            for allergen in &food.allergens {
                possible_allergens
                    .get_mut(ingredient)
                    .unwrap()
                    .remove(allergen);
            }
        }
    }
    possible_allergens
}

fn find_non_allergens(possible_allergens: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    possible_allergens
        .iter()
        .filter(|(_, allergens)| allergens.is_empty())
        .map(|(ingredient, _)| ingredient.clone())
        .collect()
}

fn determine_allergens(
    possible_allergens: &HashMap<String, HashSet<String>>,
) -> HashMap<String, String> {
    let mut to_be_determined: Vec<_> = possible_allergens
        .iter()
        .map(|(s, set)| (s.clone(), set.clone()))
        .collect();

    let mut dangerous_ingredient_list = HashMap::new();
    while !to_be_determined.is_empty() {
        to_be_determined.sort_by_key(|(_, set)| set.len());
        to_be_determined.reverse();

        let (ingredient, allergens) = to_be_determined.pop().unwrap();
        if allergens.is_empty() {
            continue;
        }
        assert!(allergens.len() == 1, "unable to determine allergens");
        let allergen = allergens.iter().next().unwrap();
        dangerous_ingredient_list.insert(allergen.clone(), ingredient.clone());
        for (_, remaining_allergens) in &mut to_be_determined {
            remaining_allergens.remove(allergen);
        }
    }
    dangerous_ingredient_list
}

fn main() {
    let input = include_str!("input");
    let foods: Vec<Food> = input.lines().map(|s| Food::from_string(s)).collect();
    let possible_allergens = find_possible_allergens(&foods);
    let non_allergens = find_non_allergens(&possible_allergens);
    if is_part2() {
        let mut dangerous_ingredient_list = determine_allergens(&possible_allergens);
        let mut dangerous_ingredients = dangerous_ingredient_list.drain().collect::<Vec<_>>();
        dangerous_ingredients.sort_by(|(allergen1, _), (allergen2, _)| allergen1.cmp(allergen2));
        let list = dangerous_ingredients
            .drain(..)
            .map(|(_, ingredient)| ingredient)
            .collect::<Vec<_>>()
            .join(",");
        println!("{}", list);
    } else {
        let count: usize = non_allergens
            .iter()
            .map(|ingredient| {
                foods
                    .iter()
                    .filter(|food| food.ingredients.contains(ingredient))
                    .count()
            })
            .sum();
        println!("{}", count);
    }
}

fn is_part2() -> bool {
    env::args().nth(1).map(|val| val == "2").unwrap_or(false)
}

#[test]
fn test_parse_food() {
    let food = Food::from_string("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)");
    assert_eq!(
        food.ingredients,
        ["mxmxvkd", "kfcds", "sqjhc", "nhms"]
            .iter()
            .cloned()
            .map(String::from)
            .collect()
    );
    assert_eq!(
        food.allergens,
        ["dairy", "fish"]
            .iter()
            .cloned()
            .map(String::from)
            .collect()
    );
}

#[test]
fn test_example() {
    let input = [
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
        "trh fvjkl sbzzf mxmxvkd (contains dairy)",
        "sqjhc fvjkl (contains soy)",
        "sqjhc mxmxvkd sbzzf (contains fish)",
    ];
    let foods: Vec<Food> = input.iter().map(|s| Food::from_string(s)).collect();
    let possible_allergens = find_possible_allergens(&foods);
    let non_allergens = find_non_allergens(&possible_allergens);
    assert_eq!(
        non_allergens,
        ["kfcds", "nhms", "sbzzf", "trh"]
            .iter()
            .cloned()
            .map(String::from)
            .collect()
    );
    let dangerous_ingredient_list = determine_allergens(&possible_allergens);
    assert_eq!(
        dangerous_ingredient_list,
        [("dairy", "mxmxvkd"), ("fish", "sqjhc"), ("soy", "fvjkl")]
            .iter()
            .cloned()
            .map(|(k, v)| (String::from(k), String::from(v)))
            .collect()
    );
}
