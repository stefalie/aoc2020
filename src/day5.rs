pub fn run() {
    let input_bytes = include_bytes!("day5_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let lines: Vec<&str> = input_string.lines().collect();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &[&str]) {
    for l in lines.iter() {
        println!("{}", l);
    }

    println!("Day 5, part 1: {}", lines.len());
}

fn part2(lines: &[&str]) {
    println!("Day 5, part 2: {}", lines.len());
}
