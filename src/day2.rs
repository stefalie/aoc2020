pub fn run() {
    let input_bytes = include_bytes!("day2_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let mut tuples = Vec::new();

    for l in input_string.lines() {
        let mut chunks = l.split_whitespace();
        let range = chunks.next().unwrap();
        let mut range_split = range.split('-');
        let lo = range_split.next().unwrap().parse::<i32>().unwrap();
        let hi = range_split.next().unwrap().parse::<i32>().unwrap();
        let letter = chunks.next().unwrap().strip_suffix(":").unwrap();
        let pw = chunks.next().unwrap();

        tuples.push((lo, hi, letter, pw))
    }

    part1(&tuples);
    part2(&tuples);
}

fn part1(tuples: &[(i32, i32, &str, &str)]) {
    let mut num_correct_pws = 0;

    for (lo, hi, letter, pw) in tuples.iter() {
        let count = pw.matches(letter).count() as i32;
        if (count >= *lo) && (count <= *hi) {
            num_correct_pws += 1;
        }
    }

    println!("Day 2, part 1: {}", num_correct_pws);
}

fn part2(tuples: &[(i32, i32, &str, &str)]) {
    let mut num_correct_pws = 0;

    for (lo, hi, letter, pw) in tuples.iter() {
        let i1 = (lo - 1) as usize;
        let i2 = (hi - 1) as usize;
        if (pw[i1..(i1 + 1)] == **letter) ^ (pw[i2..(i2 + 1)] == **letter) {
            num_correct_pws += 1;
        }
    }

    println!("Day 2, part 2: {}", num_correct_pws);
}
