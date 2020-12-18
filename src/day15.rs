fn play(numbers: &[i32], n: usize) -> i32 {
    let mut last_turn = vec![None; n - 1];  // Faster than HashMap

    for (i, n) in numbers[..(numbers.len() - 1)].iter().enumerate() {
        last_turn[*n as usize] = Some(i as i32  + 1);
    }

    let mut last: i32 = *numbers.iter().last().unwrap();
    for i in (numbers.len() + 1)..=n {
        let curr = match last_turn[last as usize] {
            Some(t) => i as i32 - 1 - t,
            None => 0,
        };
        last_turn[last as usize] = Some(i as i32 - 1);
        last = curr;
    }

    return last;
}

pub fn run() {
    let input_bytes = include_bytes!("day15_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let numbers: Vec<i32> = input_string
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    part1(&numbers);
    part2(&numbers);
}

fn part1(numbers: &[i32]) {
    let result = play(numbers, 2020);
    println!("Day 15, part 1: {}", result);
}

fn part2(numbers: &[i32]) {
    let result = play(numbers, 30000000);
    println!("Day 15, part 2: {}", result);
}
