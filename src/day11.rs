#[derive(Clone, Copy, PartialEq)]
enum GridType {
    Floor,
    EmptySeat,
    OccupiedSeat,
    Boundary,
}

// TODO: Figure out how to deal with structs.
//struct Grid {
//    content: Vec<GridType>,
//    width: usize,
//}

pub fn run() {
    let input_bytes = include_bytes!("day11_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();

    let width = input_string.lines().next().unwrap().len() + 2;

    // Map every line to its enum equivalents, and also put a boundary of '.' around everything.
    let mut grid = vec![GridType::Boundary; width];
    input_string.lines().for_each(|line| {
        grid.push(GridType::Boundary);
        grid.append(
            &mut line
                .chars()
                .map(|c| match c {
                    '.' => GridType::Floor,
                    'L' => GridType::EmptySeat,
                    '#' => GridType::OccupiedSeat,
                    _ => panic!("Unknown character {}.", c),
                })
                .collect(),
        );
        grid.push(GridType::Boundary);
    });
    grid.append(&mut vec![GridType::Boundary; width]);

    part1(&grid, width);
    part2(&grid, width);
}

// TODO: Use 'dyn Fn'? What does it do?
type CountNeighbors = fn(&[GridType], usize, usize) -> usize;
fn count_adjacent_occupied(grid: &[GridType], width: usize, index: usize) -> usize {
    let fields = [
        index - 1 - width,
        index + 0 - width,
        index + 1 - width,
        index - 1 - 0,
        index + 1 - 0,
        index - 1 + width,
        index + 0 + width,
        index + 1 + width,
    ];

    fields
        .iter()
        .filter(|i| grid[**i] == GridType::OccupiedSeat)
        .count()
}

fn evolve(
    grid: &[GridType],
    width: usize,
    leave_thres: usize,
    counter: CountNeighbors,
) -> (Vec<GridType>, bool) {
    let mut changed = false;

    let new_grid = grid
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let new_f = match f {
                GridType::EmptySeat if counter(grid, width, i) == 0 => GridType::OccupiedSeat,
                GridType::OccupiedSeat if counter(grid, width, i) >= leave_thres => {
                    GridType::EmptySeat
                }
                _ => *f, // everything else stays unchanged
            };

            // TODO: Not sure if having a map with side effects is elegant.
            if new_f != *f {
                changed = true;
            }

            new_f
        })
        .collect();

    (new_grid, changed)
}

fn count_iterations(
    grid: &[GridType],
    width: usize,
    leave_thres: usize,
    count_fn: CountNeighbors,
) -> usize {
    // TODO: I got no clue why this works.
    let mut g = grid.to_vec();
    loop {
        let (new_g, changed) = evolve(&g, width, leave_thres, count_fn);
        g = new_g;

        if !changed {
            break;
        }
    }

    g.iter().filter(|c| **c == GridType::OccupiedSeat).count()
}

fn part1(grid: &[GridType], width: usize) {
    println!(
        "Day 11, part 1: {}",
        count_iterations(grid, width, 4, count_adjacent_occupied)
    );
}

fn count_visible_occupied(grid: &[GridType], width: usize, index: usize) -> usize {
    let dirs = [
        -1 - width as isize,
        0 - width as isize,
        1 - width as isize,
        -1 - 0,
        1 - 0,
        -1 + width as isize,
        0 + width as isize,
        1 + width as isize,
    ];

    dirs.iter()
        .filter(|dir| {
            let mut curr = index as isize + **dir;
            while grid[curr as usize] == GridType::Floor {
                curr += **dir;
            }
            return grid[curr as usize] == GridType::OccupiedSeat;
        })
        .count()
}

fn part2(grid: &[GridType], width: usize) {
    println!(
        "Day 11, part 2: {}",
        count_iterations(grid, width, 5, count_visible_occupied)
    );
}
