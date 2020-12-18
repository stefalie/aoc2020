use regex::Regex;
use std::collections::HashMap;

enum Inst<'a> {
    Mask(&'a str),
    Mem { addr: u64, val: u64 },
}

pub fn run() {
    let input_bytes = include_bytes!("day14_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let re_mask = Regex::new(r"mask = ([01X]{36})").unwrap();
    let re_mem = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

    let instructions: Vec<Inst> = input_string
        .lines()
        .map(|line| {
            if let Some(cap) = re_mask.captures(line) {
                Inst::Mask(cap.get(1).unwrap().as_str())
            } else if let Some(cap) = re_mem.captures(line) {
                Inst::Mem {
                    addr: cap.get(1).unwrap().as_str().parse::<u64>().unwrap(),
                    val: cap.get(2).unwrap().as_str().parse::<u64>().unwrap(),
                }
            } else {
                panic!("Unknown instruction: {}", line);
            }
        })
        .collect();

    part1(&instructions);
    part2(&instructions);
}

fn part1(instructions: &[Inst]) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask_or: u64 = 0;
    let mut mask_and: u64 = 0;

    for inst in instructions {
        match inst {
            Inst::Mask(mask) => {
                mask_or = 0;
                mask_and = 0;

                for c in mask.chars() {
                    mask_or <<= 1;
                    mask_and <<= 1;

                    match c {
                        '1' => mask_or |= 1,
                        'X' => mask_and |= 1,
                        '0' => (), // mask_and |= 0, no-op
                        _ => panic!("Unknown character: {}", c),
                    }
                }
            }
            Inst::Mem { addr, val } => {
                let _ = memory.insert(*addr, (val & mask_and) | mask_or);
            }
        }
    }

    let result = memory.iter().fold(0, |acc, (_, val)| acc + val);
    println!("Day 14, part 1: {}", result);
}

// TODO: Very similar to part1, could do semantic compression.
fn part2(instructions: &[Inst]) {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask_or: u64 = 0;
    let mut mask_num_addr: u64 = 0;
    // Good ole https://fgiesen.wordpress.com/2011/01/17/texture-tiling-and-swizzling/
    let mut mask_x: u64 = 0; // Identical to mask_and above.

    for inst in instructions {
        match inst {
            Inst::Mask(mask) => {
                mask_or = 0;
                mask_num_addr = 1;

                for c in mask.chars() {
                    mask_or <<= 1;
                    mask_x <<= 1;

                    match c {
                        '1' => mask_or |= 1,
                        'X' => {
                            mask_x |= 1;
                            mask_num_addr <<= 1;
                        }
                        '0' => (),
                        _ => panic!("Unknown character: {}", c),
                    }
                }
            }
            Inst::Mem { addr, val } => {
                // Apply the 1s (in mask_or) and clear all the Xs to 0.
                let addr_base = (addr | mask_or) & !mask_x;
                let mut offset: u64 = 0;
                for _i in 0..mask_num_addr {
                    let _ = memory.insert(addr_base + offset, *val);
                    offset = offset.wrapping_sub(mask_x) & mask_x;
                }
            }
        }
    }

    let result = memory.iter().fold(0, |acc, (_, val)| acc + val);
    println!("Day 14, part 2: {}", result);
}
