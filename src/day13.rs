pub fn run() {
    let input_bytes = include_bytes!("day13_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let mut lines = input_string.lines();
    let timestamp = lines.next().unwrap().parse::<u64>().unwrap();
    let frequencies: Vec<(u64, u64)> = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, f)| *f != "x") // filter out the 'x's
        .map(|(i, f)| (i as u64, f.parse::<u64>().unwrap()))
        .collect();

    part1(&frequencies, timestamp);
    part2(&frequencies);
}

fn part1(frequencies: &[(u64, u64)], timestamp: u64) {
    let (min_freq, min_delay) = frequencies
        .iter()
        .map(|(_, f)| {
            let n = (timestamp + f - 1) / f;
            (f, n * f - timestamp)
        })
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    println!("Day 13, part 1: {}", min_freq * min_delay);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn part2(frequencies: &[(u64, u64)]) {
    let mut timestamp = 0;
    let mut step = frequencies[0].1;

    for f in frequencies {
        while (timestamp + f.0) % f.1 != 0 {
            timestamp += step;
        }
        step = lcm(step, f.1);
    }

    println!("Day 13, part 2: {}", timestamp);
}
