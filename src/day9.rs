pub fn run() {
    let input_bytes = include_bytes!("day9_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let numbers: Vec<i64> = input_string
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    part1(&numbers);
    part2(&numbers);
}

fn find_weak_number(numbers: &[i64]) -> Result<i64, &str> {
    fn check_sum(number: i64, buffer: &[i64]) -> bool {
        // Using a slice with a param of type &[i64; 25] doesn't work unfortunately.
        assert_eq!(buffer.len(), 25);
        for x in buffer[0..24].iter() {
            for y in buffer[1..25].iter() {
                if x + y == number {
                    return true;
                }
            }
        }
        return false;
    }

    for i in 25..numbers.len() {
        if !check_sum(numbers[i], &numbers[(i - 25)..i]) {
            return Ok(numbers[i]);
        }
    }

    return Err("Couldn't find the weak number.");
}

fn part1(numbers: &[i64]) {
    println!("Day 9, part 1: {}", find_weak_number(numbers).unwrap());
}

fn part2(numbers: &[i64]) {
    fn contiguous_range(weak_number: i64, numbers: &[i64]) -> Result<&[i64], &str> {
        for i in 0..(numbers.len() - 1) {
            for j in (i + 1)..numbers.len() {
                let sum: i64 = numbers[i..=j].iter().sum();
                if sum > weak_number {
                    break;
                } else if sum == weak_number {
                    return Ok(&numbers[i..=j]);
                }
            }
        }
        return Err("Couldn't find the contiguous range.");
    }

    let weak_number = find_weak_number(numbers).unwrap();
    let range = contiguous_range(weak_number, numbers).unwrap();

    println!(
        "Day 9, part 2: {}",
        range.iter().min().unwrap() + range.iter().max().unwrap()
    );
}
