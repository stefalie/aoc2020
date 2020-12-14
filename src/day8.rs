pub fn run() {
    let input_bytes = include_bytes!("day8_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let mut instructions: Vec<(&str, i32)> = input_string
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            let inst = split.next().unwrap();
            let arg = split.next().unwrap().parse::<i32>().unwrap();
            return (inst, arg);
        })
        .collect();

    part1(&instructions);
    part2(&mut instructions);
}

fn interpret(instructions: &[(&str, i32)]) -> (bool, i32) {
    let mut acc: i32 = 0;
    let mut ip: i32 = 0;

    let mut executed_insts = vec![false; instructions.len()];

    loop {
        if ip == instructions.len() as i32 {
            return (true, acc);
        }
        if ip > instructions.len() as i32 || executed_insts[ip as usize] {
            return (false, acc);
        }
        executed_insts[ip as usize] = true;

        let inst = instructions[ip as usize];
        match inst {
            ("acc", arg) => {
                acc += arg;
                ip += 1;
            }
            ("jmp", arg) => {
                ip += arg;
            }
            ("nop", _) => {
                ip += 1;
            }
            _ => panic!("Unknown instruction {}", inst.0),
        }
    }
}

fn part1(instructions: &[(&str, i32)]) {
    println!("Day 8, part 1: {}", interpret(instructions).1);
}

fn part2(instructions: &mut [(&str, i32)]) {
    fn modify(index: usize, instructions: &mut [(&str, i32)]) -> bool {
        let inst = &mut instructions[index];
        match inst {
            ("acc", _) => return false,
            ("nop", arg) => *inst = ("jmp", *arg),
            ("jmp", arg) => *inst = ("nop", *arg),
            _ => panic!("Unknown instruction {}", inst.0),
        }
        return true;
    }

    let mut result: Result<i32, &str> = Err("No valid result found.");
    for i in 0..instructions.len() {
        if modify(i, instructions) {
            let (success, acc) = interpret(instructions);
            modify(i, instructions);  // revert change
            if success {
                result = Ok(acc);
                break;
            }
        }
    }

    println!("Day 8, part 2: {}", result.unwrap());
}
