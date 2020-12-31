use itertools::Itertools;
use std::collections::{BTreeMap, HashMap, HashSet};

pub fn run() {
    let input_bytes = include_bytes!("day21_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let food: Vec<(Vec<&str>, Vec<&str>)> = input_string
        .lines()
        .map(|line| {
            let (ingredients, allergens) = line.splitn(2, " (contains ").next_tuple().unwrap();
            let ingredients = ingredients.split_whitespace().collect();
            let allergens = allergens.strip_suffix(')').unwrap();
            let allergens = allergens.split(", ").collect();
            (ingredients, allergens)
        })
        .collect();

    let mut allergy_to_ingredient: HashMap<&str, HashSet<&str>> = HashMap::new();
    for f in &food {
        for allergy in &f.1 {
            let set_new: HashSet<&str> = f.0.iter().cloned().collect();

            if let Some(set) = allergy_to_ingredient.get_mut(allergy) {
                *set = set.intersection(&set_new).copied().collect();
            } else {
                allergy_to_ingredient.insert(allergy, set_new);
            }
        }
    }

    part1(&food, &allergy_to_ingredient);
    part2(&mut allergy_to_ingredient);
}

fn part1(food: &[(Vec<&str>, Vec<&str>)], allergy_to_ingredient: &HashMap<&str, HashSet<&str>>) {
    let bad_ingredients: HashSet<&str> = allergy_to_ingredient
        .values()
        .flat_map(|ings| ings.iter())
        .copied()
        .collect();

    let result: usize = food
        .iter()
        .map(|f| {
            f.0.iter()
                .filter(|ing| !bad_ingredients.contains(*ing))
                .count()
        })
        .sum();

    println!("Day 21, part 1: {}", result);
}

fn part2(allergy_to_ingredient: &mut HashMap<&str, HashSet<&str>>) {
    // See also day16.rs for a faster solution.
    // TODO: This is not efficient.
    let mut used_ingredients: HashSet<&str> = HashSet::new();
    let mut mapping: BTreeMap<&str, &str> = BTreeMap::new();
    while let Some((k, v)) = allergy_to_ingredient.iter().find(|(_k, v)| {
        v.iter()
            .filter(|ing| !used_ingredients.contains(*ing))
            .count()
            == 1
    }) {
        let ingredient = v
            .iter()
            .filter(|ing| !used_ingredients.contains(*ing))
            .next()
            .unwrap();

        mapping.insert(*k, *ingredient);
        used_ingredients.insert(ingredient);
    }

    let result = mapping.values().join(",");
    println!("Day 21, part 2: {}", result);
}
