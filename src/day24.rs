use std::collections::HashSet;

enum HexDir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

pub fn run() {
    let input_bytes = include_bytes!("day24_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let flip_dirs: Vec<Vec<HexDir>> = input_string
        .lines()
        .map(|line| {
            let mut path = Vec::new();
            let mut has_s = false;
            let mut has_n = false;
            line.chars().for_each(|c| match c {
                'e' => {
                    if has_n {
                        path.push(HexDir::NE);
                        has_n = false;
                    } else if has_s {
                        path.push(HexDir::SE);
                        has_s = false;
                    } else {
                        path.push(HexDir::E);
                    }
                }
                'w' => {
                    if has_n {
                        path.push(HexDir::NW);
                        has_n = false;
                    } else if has_s {
                        path.push(HexDir::SW);
                        has_s = false;
                    } else {
                        path.push(HexDir::W);
                    }
                }
                'n' => has_n = true,
                's' => has_s = true,
                _ => panic!("Unknown character {}.", c),
            });
            path
        })
        .collect();

    part1(&flip_dirs);
    part2(&flip_dirs);
}

fn eval_path(path: &[HexDir]) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    for step in path.iter() {
        match step {
            HexDir::E => x += 1,
            HexDir::SE => y -= 1,
            HexDir::SW => {
                y -= 1;
                x -= 1;
            }
            HexDir::W => x -= 1,
            HexDir::NW => y += 1,
            HexDir::NE => {
                y += 1;
                x += 1;
            }
        }
    }
    (x, y)
}

fn part1(flip_dirs: &[Vec<HexDir>]) {
    let mut flipped_once = HashSet::new();

    for tile in flip_dirs.iter() {
        let coord = eval_path(tile);
        if flipped_once.contains(&coord) {
            flipped_once.remove(&coord);
        } else {
            flipped_once.insert(coord);
        }
    }
    let result = flipped_once.len();
    println!("Day 24, part 1: {}", result);
}

fn part2(flip_dirs: &[Vec<HexDir>]) {
    // Find size of grid.
    let mut min = (std::i32::MAX, std::i32::MAX);
    let mut max = (std::i32::MIN, std::i32::MIN);
    for path in flip_dirs.iter() {
        let coord = eval_path(path);
        min.0 = std::cmp::min(coord.0, min.0);
        min.1 = std::cmp::min(coord.1, min.1);
        max.0 = std::cmp::max(coord.0, max.0);
        max.1 = std::cmp::max(coord.1, max.1);
    }
    // +1 for origin, +2 for margin, +2 * num_iter because it could potentially spread
    let num_iter = 100;
    let margin = 1 + num_iter;
    let extra = 1 + 2 * margin;
    // This could probably be much smaller, but was the simplest estimate that I could think
    // off that definitely is large enough to contain any black tiles generated over 100
    // iterations.
    let dim = (max.0 - min.0 + extra, max.1 - min.1 + extra);
    let mut grid = vec![false; (dim.0 * dim.1) as usize];

    let lin_coord = |c: (i32, i32)| ((c.0 - min.0 + margin) + (c.1 - min.1 + margin) * dim.0) as usize;

    for path in flip_dirs.iter() {
        let coord = lin_coord(eval_path(path));
        grid[coord] = !grid[coord];
    }

    let mut changes = Vec::new();
    let nghbrs = [(-1, 0), (0, 1), (1, 1), (1, 0), (0, -1), (-1, -1)];

    for it in 1..=num_iter {
        for ty in (min.1 - it)..=(max.1 + it)  {
            for tx in (min.0 - it)..=(max.0 + it) {
                // Count neighbors.
                let num_black = nghbrs
                    .iter()
                    .filter(|n| grid[lin_coord((tx + n.0, ty + n.1))])
                    .count();

                let coord = lin_coord((tx, ty));
                if (grid[coord] && (num_black == 0 || num_black > 2))
                    || (!grid[coord] && num_black == 2)
                {
                    changes.push(coord);
                }
            }
        }

        for c in changes.iter() {
            grid[*c] = !grid[*c];
        }
        changes.clear();
    }

    let result = grid.iter().filter(|t| **t).count();
    println!("Day 24, part 2: {}", result);
}
