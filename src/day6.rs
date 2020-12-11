pub fn run() {
    let input_bytes = include_bytes!("day6_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let lines: Vec<&str> = input_string.lines().collect();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[&str]) {
    let mut num_total_yes_answers = 0;

    let mut yes_answers = [false; 26];
    for line in lines.iter() {
        if line.is_empty() {
            num_total_yes_answers += yes_answers.iter().filter(|&x| *x).count();
            yes_answers = [false; 26];
        }

        for c in line.chars() {
            yes_answers[(c as usize) - ('a' as usize)] = true;
            // This is probably more elegant:
            // yes_answers[c.to_digit(36).unwrap() as usize - 10] = true;
        }
    }
    num_total_yes_answers += yes_answers.iter().filter(|&x| *x).count();

    println!("Day 6, part 1: {}", num_total_yes_answers);
}

fn part2(lines: &[&str]) {
    let mut num_total_group_yes_answers = 0;

    let mut curr_group: i32 = -1; // Or 0x03ffffff, but it doesn't matter.
    for line in lines.iter() {
        if line.is_empty() {
            num_total_group_yes_answers += curr_group.count_ones();
            curr_group = -1;
        } else {
            curr_group &= line
                .chars()
                .fold(0, |acc, c| acc | (1 << (c.to_digit(36).unwrap() - 10)));
            // println!("current group: {:#028b}", curr_group);
        }
    }
    num_total_group_yes_answers += curr_group.count_ones();

    println!("Day 6, part 2: {}", num_total_group_yes_answers);
}
