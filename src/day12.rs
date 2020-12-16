pub fn run() {
    let input_bytes = include_bytes!("day12_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let instructions: Vec<(char, i32)> = input_string
        .lines()
        .map(|line| {
            (
                line.chars().nth(0).unwrap(),
                line[1..].parse::<i32>().unwrap(),
            )
        })
        .collect();

    part1(&instructions);
    part2(&instructions);
}

fn part1(instructions: &[(char, i32)]) {
    let mut n: i32 = 0;
    let mut e: i32 = 0;
    let mut h: i32 = 0;

    for inst in instructions {
        match inst {
            ('N', arg) => n += arg,
            ('S', arg) => n -= arg,
            ('E', arg) => e += arg,
            ('W', arg) => e -= arg,
            ('F', arg) => match h {
                270 => n += arg,
                90 => n -= arg,
                0 => e += arg,
                180 => e -= arg,
                _ => panic!("Unknown orientation {}", h),
            },
            ('R', arg) => h = (h + arg) % 360,
            ('L', arg) => h = (h - arg + 360) % 360,
            _ => panic!("Unknown instruction {}{}", inst.0, inst.1),
        }
    }

    println!(
        "Day 12, part 1: {} = manhatten({}, {})",
        n.abs() + e.abs(),
        n,
        e
    );
}

fn part2(instructions: &[(char, i32)]) {
    let mut ship_n: i32 = 0;
    let mut ship_e: i32 = 0;
    let mut waypoint_n: i32 = 1;
    let mut waypoint_e: i32 = 10;

    for inst in instructions {
        match inst {
            ('N', arg) => waypoint_n += arg,
            ('S', arg) => waypoint_n -= arg,
            ('E', arg) => waypoint_e += arg,
            ('W', arg) => waypoint_e -= arg,
            ('F', arg) => {
                ship_n += arg * waypoint_n;
                ship_e += arg * waypoint_e;
            }
            ('R', _arg @ 180) | ('L', _arg @ 180) => {
                waypoint_n *= -1;
                waypoint_e *= -1;
            }
            ('R', _arg @ 270) | ('L', _arg @ 90) => {
                std::mem::swap(&mut waypoint_n, &mut waypoint_e);
                waypoint_e *= -1;
            }
            ('R', _arg @ 90) | ('L', _arg @ 270) => {
                std::mem::swap(&mut waypoint_n, &mut waypoint_e);
                waypoint_n *= -1;
            }
            _ => panic!("Unknown instruction {}{}", inst.0, inst.1),
        }
    }

    println!(
        "Day 12, part 2: {} = manhatten({}, {})",
        ship_n.abs() + ship_e.abs(),
        ship_n,
        ship_e
    );
}
