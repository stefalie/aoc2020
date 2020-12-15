pub fn run() {
    let input_bytes = include_bytes!("day10_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let mut numbers: Vec<u8> = input_string
        .split_whitespace()
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    numbers.push(*numbers.iter().max().unwrap() + 3);
    numbers.push(0);
    numbers.sort();

    part1(&numbers);
    part2(&numbers);
}

fn part1(numbers: &[u8]) {
    fn count_diffs(expected_diff: u8, numbers: &[u8]) -> usize {
        let values = numbers.iter();
        let next_values = numbers.iter().skip(1);
        let diffs = values.zip(next_values).map(|(curr, next)| next - curr);
        return diffs.filter(|d| *d == expected_diff as u8).count();
    };
    println!(
        "Day 10, part 1: {}",
        count_diffs(1, numbers) * count_diffs(3, numbers)
    );
}

fn part2(numbers: &[u8]) {
    let mut num_ways = vec![0 as u64; numbers.len()];
    num_ways[0] = 1;

    for i in 1..numbers.len() {
        // The current number can definitely be reached from the previous number
        // and maybe from the preceeding two numbers too. Therefore it's enough
        // to only consider the 3 previous numbers.
        num_ways[i] = num_ways[i - 1];
        for j in 2..=std::cmp::min(i, 3) {
            if (numbers[i] - numbers[i - j]) <= 3 {
                num_ways[i] += num_ways[i - j];
            }
        }
    }

    println!("Day 10, part 2: {}", num_ways.last().unwrap());
}
