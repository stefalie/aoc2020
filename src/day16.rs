#[allow(dead_code)]
struct Field<'a> {
    name: &'a str,
    range1: std::ops::Range<u32>,
    range2: std::ops::Range<u32>,
}

pub fn run() {
    let input_bytes = include_bytes!("day16_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let mut split1 = input_string.splitn(2, "your ticket:");
    let fields: Vec<Field> = split1
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|line| {
            let mut split1 = line.split(": ");
            let name = split1.next().unwrap();
            let mut split2 = split1.next().unwrap().split(" or ");
            let mut range1 = split2
                .next()
                .unwrap()
                .split('-')
                .map(|n| n.parse::<u32>().unwrap());
            let mut range2 = split2
                .next()
                .unwrap()
                .split('-')
                .map(|n| n.parse::<u32>().unwrap());
            Field {
                name,
                range1: range1.next().unwrap()..range1.next().unwrap() + 1,
                range2: range2.next().unwrap()..range2.next().unwrap() + 1,
            }
        })
        .collect();

    let mut split2 = split1.next().unwrap().splitn(2, "nearby tickets:");
    let ticket: Vec<u32> = split2
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    let nearby_tickets: Vec<Vec<u32>> = split2
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.split(',').map(|n| n.parse::<u32>().unwrap()).collect())
        .collect();

    part1(&fields, &nearby_tickets);
    part2(&fields, &ticket, &nearby_tickets);
}

fn is_valid(n: u32, field: &Field) -> bool {
    field.range1.contains(&n) || field.range2.contains(&n)
}

fn is_valid_any(n: u32, fields: &[Field]) -> bool {
    fields.iter().any(|f| is_valid(n, f))
}

fn part1(fields: &[Field], nearby_tickets: &[Vec<u32>]) {
    let result: u32 = nearby_tickets
        .iter()
        .map(|t| {
            t.iter()
                .map(|n| if is_valid_any(*n, fields) { 0 } else { *n })
                .sum::<u32>()
        })
        .sum();

    println!("Day 16, part 1: {}", result);
}

fn part2(fields: &[Field], ticket: &[u32], nearby_tickets: &[Vec<u32>]) {
    let valid_tickets: Vec<Vec<u32>> = nearby_tickets
        .iter()
        .filter(|t| t.iter().all(|n| is_valid_any(*n, fields)))
        .cloned()
        .collect();

    let mut matrix = [[true; 20]; 20];
    for t in &valid_tickets {
        fields.iter().enumerate().for_each(|(f_idx, f)| {
            t.iter().enumerate().for_each(|(col, n)| {
                if !is_valid(*n, f) {
                    matrix[f_idx][col] = false;
                }
            });
        });
    }

    //for x in 0..20 {
    //    for y in 0..20 {
    //        if matrix[x][y] {
    //            print!("1");
    //        } else {
    //            print!("0");
    //        }
    //    }
    //    println!();
    //}

    // TODO: This should be easier to do, no? Some fancy matrix ops?
    let mut map_field_to_col = [0; 20];
    for _ in 0..20 {
        for f_idx in 0..20 {
            if matrix[f_idx].iter().filter(|b| **b).count() == 1 {
                let col = matrix[f_idx]
                    .iter()
                    .enumerate()
                    .find(|(_, b)| **b)
                    .unwrap()
                    .0;
                map_field_to_col[f_idx] = col;

                // Clear this column everywhere and restart.
                for f in 0..20 {
                    matrix[f][col] = false;
                }
                break;
            }
        }
    }

    //for (i, m) in map_field_to_col.iter().enumerate() {
    //    println!("{} -> {}", i, m);
    //}

    let result = map_field_to_col[..6]
        .iter()
        .map(|col| ticket[*col] as usize)
        .fold(1, |acc, x| acc * x);
    println!("Day 16, part 2: {}", result);
}
