#[derive(Clone, Copy, PartialEq)]
enum Inst {
    Add,
    Mul,
    ParensOpen,
    ParensClose,
    Int(i64),
}

pub fn run() {
    let input_bytes = include_bytes!("day18_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let exprs: Vec<Vec<Inst>> = input_string
        .lines()
        .map(|line| {
            let mut expr = Vec::new();
            line.chars().for_each(|c| match c {
                '+' => expr.push(Inst::Add),
                '*' => expr.push(Inst::Mul),
                '(' => expr.push(Inst::ParensOpen),
                ')' => expr.push(Inst::ParensClose),
                '1'..='9' => expr.push(Inst::Int(c.to_digit(36).unwrap() as i64)),
                ' ' => (),
                _ => panic!("Unknown character {}", c),
            });
            expr
        })
        .collect();

    part1(&exprs);
    part2(&exprs);
}

type Precedence = fn(op: Inst) -> usize;

// https://www.geeksforgeeks.org/expression-evaluation/
// Shunting Yard Algorithm by Edgar Dijkstra
fn interpret(expr: &[Inst], precedence: Precedence) -> i64 {
    let mut values: Vec<i64> = Vec::new();
    let mut ops = Vec::new();

    fn apply_op(ops: &mut Vec<Inst>, values: &mut Vec<i64>) {
        let lhs = values.pop().unwrap();
        let rhs = values.pop().unwrap();
        match ops.pop().unwrap() {
            Inst::Add => values.push(lhs + rhs),
            Inst::Mul => values.push(lhs * rhs),
            _ => panic!("Unexpected instruction on operator stack."),
        }
    };

    for i in expr.iter() {
        match i {
            Inst::Add | Inst::Mul => {
                while !ops.is_empty() && precedence(*(ops.last().unwrap())) >= precedence(*i) {
                    apply_op(&mut ops, &mut values);
                }
                ops.push(*i);
            }
            Inst::ParensOpen => ops.push(Inst::ParensOpen),
            Inst::ParensClose => {
                while *ops.last().unwrap() != Inst::ParensOpen {
                    // Potentially also stop on empty, but that would be an error.
                    apply_op(&mut ops, &mut values);
                }
                ops.pop(); // Pop '('.
            }
            Inst::Int(num) => values.push(*num),
        }
    }

    while !ops.is_empty() {
        apply_op(&mut ops, &mut values);
    }
    assert_eq!(values.len(), 1);

    return values[0];
}

fn precedence1(op: Inst) -> usize {
    match op {
        Inst::Add | Inst::Mul => 1,
        _ => 0,
    }
}

fn part1(exprs: &[Vec<Inst>]) {
    let result: i64 = exprs.iter().map(|e| interpret(e, precedence1)).sum();
    println!("Day 18, part 1: {}", result);
}

fn precedence2(op: Inst) -> usize {
    match op {
        Inst::Add => 2,
        Inst::Mul => 1,
        _ => 0,
    }
}

fn part2(exprs: &[Vec<Inst>]) {
    let result: i64 = exprs.iter().map(|e| interpret(e, precedence2)).sum();
    println!("Day 18, part 2: {}", result);
}
