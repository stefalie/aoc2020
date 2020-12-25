use itertools::Itertools;

#[derive(Clone)]
enum Rule {
    SeqChoice(Vec<u32>, Vec<u32>),
    Seq(Vec<u32>),
    Char(char),
}

pub fn run() {
    let input_bytes = include_bytes!("day19_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let mut rules_w_idx: Vec<(u32, Rule)> = Vec::new();

    fn parse_seq(seq: &str) -> Vec<u32> {
        seq.split(' ').map(|s| s.parse::<u32>().unwrap()).collect()
    }

    for line in input_string.lines().take_while(|line| !line.is_empty()) {
        let (idx_str, rule) = line.splitn(2, ": ").next_tuple().unwrap();
        let idx = idx_str.parse::<u32>().unwrap();

        if rule.starts_with('"') {
            rules_w_idx.push((idx, Rule::Char(rule.chars().nth(1).unwrap())));
        } else if let Some((lhs, rhs)) = rule.splitn(2, " | ").next_tuple() {
            rules_w_idx.push((idx, Rule::SeqChoice(parse_seq(lhs), parse_seq(rhs))));
        } else {
            rules_w_idx.push((idx, Rule::Seq(parse_seq(rule))));
        }
    }

    rules_w_idx.sort_by(|lhs, rhs| lhs.0.cmp(&rhs.0));
    let rules: Vec<Rule> = rules_w_idx.iter().map(|(_, r)| r.clone()).collect();

    let strings: Vec<&str> = input_string
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .collect();

    part1(&rules, &strings);
    part2(&rules, &strings);
}

fn check_seq(
    rules: &[Rule],
    rule_stack: &mut Vec<u32>,
    string: &str,
    seq: &[u32],
) -> Result<(), ()> {
    for s in seq.iter().rev() {
        rule_stack.push(*s);
    }
    let res = check(rules, rule_stack, string);
    for _ in seq.iter() {
        rule_stack.pop(); // Unwind stack.
    }
    res
}

fn check(rules: &[Rule], rule_stack: &mut Vec<u32>, string: &str) -> Result<(), ()> {
    if rule_stack.is_empty() && string.is_empty() {
        return Ok(());
    }
    if rule_stack.is_empty() || string.is_empty() {
        return Err(());
    }

    let mut res = Err(());
    let rule_idx = rule_stack.pop().unwrap();

    match &rules[rule_idx as usize] {
        Rule::Seq(seq) => {
            res = check_seq(rules, rule_stack, string, seq);
        }
        Rule::Char(c) => {
            if *c == string.chars().next().unwrap() {
                res = check(rules, rule_stack, &string[1..]);
            }
        }
        Rule::SeqChoice(seq1, seq2) => {
            res = check_seq(rules, rule_stack, string, seq1);

            // Short circuit
            if res.is_ok() {
                return res;
            }

            res = check_seq(rules, rule_stack, string, seq2);
        }
    }

    rule_stack.push(rule_idx);
    return res;
}

fn part1(rules: &[Rule], strings: &[&str]) {
    let mut stack: Vec<u32> = Vec::new();
    let result = strings
        .iter()
        .filter(|s| {
            stack.clear();
            stack.push(0);
            check(rules, &mut stack, s).is_ok()
        })
        .count();
    println!("Day 19, part 1: {}", result);
}

fn part2(rules: &[Rule], strings: &[&str]) {
    let mut rules2 = rules.to_vec();
    rules2[8] = Rule::SeqChoice(vec![42], vec![42, 8]);
    rules2[11] = Rule::SeqChoice(vec![42, 31], vec![42, 11, 31]);

    let mut stack: Vec<u32> = Vec::new();
    let result = strings
        .iter()
        .filter(|s| {
            stack.clear();
            stack.push(0);
            check(&rules2, &mut stack, s).is_ok()
        })
        .count();

    println!("Day 19, part 2: {}", result);
}
