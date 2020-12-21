#[derive(Clone, Copy, PartialEq)]
enum State {
    Inactive,
    Active,
}

const OFFSET: usize = 7;
const WIDTH: usize = 8 + 2 * OFFSET; // Initial size + 2 times possible spread.

pub fn run() {
    let input_bytes = include_bytes!("day17_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let mut grid_3d = [State::Inactive; WIDTH * WIDTH * WIDTH];
    let mut grid_4d = [State::Inactive; WIDTH * WIDTH * WIDTH * WIDTH];

    let start_offset_3d: usize = OFFSET * (1 + WIDTH * (1 + WIDTH));
    let start_offset_4d: usize = OFFSET * (1 + WIDTH * (1 + WIDTH * (1 + WIDTH)));

    input_string.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            grid_3d[start_offset_3d + j + i * WIDTH] = match c {
                '.' => State::Inactive,
                '#' => State::Active,
                _ => panic!("Unknown character {}.", c),
            };

            grid_4d[start_offset_4d + j + i * WIDTH] = match c {
                '.' => State::Inactive,
                '#' => State::Active,
                _ => panic!("Unknown character {}.", c),
            };
        })
    });

    part1(&grid_3d);
    part2(&grid_4d);
}

fn count_active_neighbors_3d(grid: &[State], index: usize) -> usize {
    let mut counter = 0;
    let range: [isize; 3] = [-1, 0, 1];
    for z in range.iter() {
        for y in range.iter() {
            for x in range.iter() {
                if *x == 0 && *y == 0 && *z == 0 {
                    continue;
                }

                let width = WIDTH as isize;
                if grid[(index as isize + x + width * (y + width * z)) as usize] == State::Active {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn evolve_3d(grid: &[State; WIDTH * WIDTH * WIDTH]) -> usize {
    let mut ping = *grid;
    let mut pong = [State::Inactive; WIDTH * WIDTH * WIDTH];

    //for x in 0..WIDTH {
    //    for y in 0..WIDTH {
    //        print!(
    //            "{}",
    //            if ping[WIDTH * WIDTH * OFFSET + x + y * WIDTH] == State::Active {
    //                '#'
    //            } else {
    //                '.'
    //            }
    //        );
    //    }
    //    println!();
    //}

    for it in 1..=6 {
        let start = OFFSET - it;
        for z in start..(WIDTH - start) {
            for y in start..(WIDTH - start) {
                for x in start..(WIDTH - start) {
                    let index = x + WIDTH * (y + WIDTH * z);

                    let num_neighbors = count_active_neighbors_3d(&ping, index);
                    if (ping[index] == State::Active && (2..=3).contains(&num_neighbors))
                        || (ping[index] == State::Inactive && num_neighbors == 3)
                    {
                        pong[index] = State::Active;
                    } else {
                        pong[index] = State::Inactive;
                    }
                }
            }
        }
        std::mem::swap(&mut ping, &mut pong);
    }

    ping.iter().filter(|&s| *s == State::Active).count()
}

fn part1(grid: &[State; WIDTH * WIDTH * WIDTH]) {
    let result = evolve_3d(grid);
    println!("Day 17, part 1: {}", result);
}

fn count_active_neighbors_4d(grid: &[State], index: usize) -> usize {
    let mut counter = 0;
    let range: [isize; 3] = [-1, 0, 1];
    for w in range.iter() {
        for z in range.iter() {
            for y in range.iter() {
                for x in range.iter() {
                    if *x == 0 && *y == 0 && *z == 0 && *w == 0 {
                        continue;
                    }

                    let width = WIDTH as isize;
                    if grid[(index as isize + x + width * (y + width * (z + width * w))) as usize]
                        == State::Active
                    {
                        counter += 1;
                    }
                }
            }
        }
    }
    counter
}

fn evolve_4d(grid: &[State; WIDTH * WIDTH * WIDTH * WIDTH]) -> usize {
    // Need vectors here to prevent stack overflow.
    let mut ping = grid.to_vec();
    let mut pong = vec![State::Inactive; WIDTH * WIDTH * WIDTH * WIDTH];

    for it in 1..=6 {
        let start = OFFSET - it;
        for w in start..(WIDTH - start) {
            for z in start..(WIDTH - start) {
                for y in start..(WIDTH - start) {
                    for x in start..(WIDTH - start) {
                        let index = x + WIDTH * (y + WIDTH * (z + WIDTH * w));

                        let num_neighbors = count_active_neighbors_4d(&ping, index);
                        if (ping[index] == State::Active && (2..=3).contains(&num_neighbors))
                            || (ping[index] == State::Inactive && num_neighbors == 3)
                        {
                            pong[index] = State::Active;
                        } else {
                            pong[index] = State::Inactive;
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut ping, &mut pong);
    }

    ping.iter().filter(|&s| *s == State::Active).count()
}

fn part2(grid: &[State; WIDTH * WIDTH * WIDTH * WIDTH]) {
    let result = evolve_4d(grid);
    println!("Day 17, part 2: {}", result);
}
