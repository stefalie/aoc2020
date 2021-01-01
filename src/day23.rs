use itertools::Itertools;

pub fn run() {
    let input_bytes = include_bytes!("day23_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let cups: Vec<usize> = input_string
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    part1(&cups);
    part2(&cups);
}

fn play(start: usize, num_iter: usize, next_cup: &mut [usize]) {
    let len = next_cup.len();

    let mut curr = start; // -1 because input_bytes is 1-based.
    for _ in 0..num_iter {
        let next1 = next_cup[curr];
        let next2 = next_cup[next1];
        let next3 = next_cup[next2];

        // Find destination.
        let dec_wrap = |n: usize| if n == 0 { len - 1 } else { n - 1 };

        let mut dest = dec_wrap(curr);
        while dest == next1 || dest == next2 || dest == next3 {
            dest = dec_wrap(dest);
        }

        next_cup[curr] = next_cup[next3]; // Close gap
        next_cup[next3] = next_cup[dest]; // Insert at dest
        next_cup[dest] = next1; // Insert at dest

        curr = next_cup[curr];
    }
}

fn part1(cups: &[usize]) {
    // Instead of doing some crazy % magic, let's build something like a linked list inside
    // vector.
    let len = cups.len();
    let mut next_cup = vec![0; len];
    for i in 0..len {
        next_cup[cups[i] - 1] = cups[(i + 1) % len] - 1; // -1 because input is 1-based.
    }

    let start = cups[0] - 1; // -1 because input_bytes is 1-based.
    play(start, 100, &mut next_cup);

    let mut after_one = vec![0; len - 1];
    let mut curr = 0; // 0 because input_bytes is 0-based.
    for i in 0..after_one.len() {
        curr = next_cup[curr];
        after_one[i] = curr + 1; // +1 because input_bytes is 0-based.
    }

    let result = after_one.iter().join("");
    println!("Day 23, part 1: {}", result);
}

// TODO: Next time just add an extra (wasted) element to the vector and use
// 1-based indices if the examples also use 1-based indexing.
fn part2(cups: &[usize]) {
    let num_total = 1000000;
    let num_initial = cups.len();
    let mut next_cup = vec![0; num_total];
    for i in 0..(num_initial - 1) {
        next_cup[cups[i] - 1] = cups[i + 1] - 1; // -1 because input is 1-based.
    }
    next_cup[cups[num_initial - 1] - 1] = num_initial;
    for i in num_initial..(num_total - 1) {
        next_cup[i] = i + 1;
    }
    let start = cups[0] - 1; // -1 because input_bytes is 1-based.
    next_cup[num_total - 1] = start;

    play(start, 10000000, &mut next_cup);

    // Note again the switch back from 0-based to 1-based.
    let one_after_one = next_cup[0];
    let two_after_one = next_cup[one_after_one];
    let result = (one_after_one + 1) * (two_after_one + 1);
    println!("Day 23, part 2: {}", result);
}
