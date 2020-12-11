pub fn run() {
    let input_bytes = include_bytes!("day5_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let lines: Vec<&str> = input_string.lines().collect();

    part1(&lines);
    part2(&lines);
}

fn seat_number(line: &str) -> i32 {
    assert_eq!(line.len(), 10);

    // The seat number are just in a weird binary format:
    // 'B' & 'R' are 1s
    // 'F' & 'L' are 0s
    let mut seat = 0;
    for (i, c) in line.chars().enumerate() {
        match c {
            'B' | 'R' => seat += 1 << (9 - i),
            'F' | 'L' => (),
            _ => panic!("Unknown character {}", c),
        }
    }

    return seat;
}

fn part1(lines: &[&str]) {
    let highest_seat_number = lines.iter().map(|l| seat_number(l)).max().unwrap();
    println!("Day 5, part 1: {}", highest_seat_number);
}

fn part2(lines: &[&str]) {
    let mut taken_seats = [false; 1024];
    lines.iter().for_each(|l| taken_seats[seat_number(l) as usize] = true);

    let first_seat = taken_seats.iter().position(|s| *s == true).unwrap();
    let my_seat = first_seat + taken_seats[first_seat..]
        .iter()
        .position(|s| *s == false)
        .unwrap();

    println!("Day 5, part 2: {}", my_seat);
}
