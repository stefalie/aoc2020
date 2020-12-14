use std::collections::{HashMap, HashSet};

pub fn run() {
    let input_bytes = include_bytes!("day7_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let mut policies = HashMap::new();

    for line in input_string.lines() {
        let mut bag_content = line.strip_suffix('.').unwrap().split(" contain ");

        let bag = bag_content.next().unwrap().strip_suffix(" bags").unwrap();
        let content = bag_content.next().unwrap();

        if content == "no other bags" {
            policies.insert(bag, Vec::new());
        } else {
            let content_vec = content
                .split(", ")
                .map(|x| {
                    let mut count_bags = x
                        .trim_end_matches(" bag")
                        .trim_end_matches(" bags")
                        .splitn(2, ' ');
                    let count = count_bags.next().unwrap().parse::<i32>().unwrap();
                    let bags = count_bags.next().unwrap();
                    return (count, bags);
                })
                .collect();
            policies.insert(bag, content_vec);
        }
    }

    // TODO: Use regex instead.
    // TODO: Use int IDs instead of string in HashMap<&str, Vec<(i32, &str)>>.

    part1(&policies);
    part2(&policies);
}

fn part1(policies: &HashMap<&str, Vec<(i32, &str)>>) {
    // recursion + memoization
    let mut can_reach = HashSet::new();
    can_reach.insert("shiny gold");

    // TODO: Use recursive closure instead?
    // https://stackoverflow.com/questions/16946888/is-it-possible-to-make-a-recursive-closure-in-rust
    fn recurse<'a>(
        bag: &'a str,
        policies: &HashMap<&str, Vec<(i32, &'a str)>>,
        can_reach: &mut HashSet<&'a str>,
    ) -> bool {
        if can_reach.contains(bag) {
            return true;
        } else {
            let contents = policies.get(bag).unwrap();
            if contents.iter().any(|c| recurse(c.1, policies, can_reach)) {
                can_reach.insert(bag);
                return true;
            } else {
                return false;
            }
        }
    }
    policies.keys().for_each(|b| {
        let _ = recurse(b, &policies, &mut can_reach);
    });

    println!("Day 7, part 1: {}", can_reach.len() - 1);
}

fn part2(policies: &HashMap<&str, Vec<(i32, &str)>>) {
    fn recurse<'a>(bag: &'a str, policies: &HashMap<&str, Vec<(i32, &'a str)>>) -> i32 {
        let contents = policies.get(bag).unwrap();
        return contents.iter().map(|c| c.0 * recurse(c.1, policies)).sum::<i32>() + 1;
    }
    let num_nested_bags = recurse("shiny gold", &policies) - 1;

    println!("Day 7, part 2: {}", num_nested_bags);
}
