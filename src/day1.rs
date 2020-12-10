pub fn run() {
    let input_bytes = include_bytes!("day1_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let numbers: Vec<i32> = input_string
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    part1(&numbers);
    part2(&numbers);
}

fn part1(numbers: &[i32]) {
    for (i1, n1) in numbers[..(numbers.len() - 1)].iter().enumerate() {
        for n2 in numbers[(i1 + 1)..].iter() {
            if (n1 + n2) == 2020 {
                println!("Day 1, part 1: {}; n1: {}; n2 {}", n1 * n2, n1, n2);
                return;
            }
        }
    }
}

fn part2(numbers: &[i32]) {
    for (i1, n1) in numbers[..(numbers.len() - 2)].iter().enumerate() {
        for (i2, n2) in numbers[(i1 + 1)..(numbers.len() - 1)].iter().enumerate() {
            for n3 in numbers[(i2 + 1)..].iter() {
                if (n1 + n2 + n3) == 2020 {
                    println!("Day 1, part 2: {}; n1: {}; n2 {}; n3 {}", n1 * n2 * n3, n1, n2, n3);
                    return;
                }
            }
        }
    }
}
