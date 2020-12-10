pub fn run() {
    let input_bytes = include_bytes!("day3_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();
    assert!(input_string.is_ascii());

    let lines = input_string.lines();
    let map: Vec<&[u8]> = lines.map(|l| l.as_bytes()).collect();

    part1(&map);
    part2(&map);
}

fn count_trees_on_slope(map: &[&[u8]], x_slope: usize, y_slope: usize) -> isize {
    let map_width = map[0].len();
    let mut num_trees = 0;
    let mut x = 0;

    for y in (0..map.len()).step_by(y_slope) {
        if map[y][x % map_width] == ('#' as u8) {
            num_trees += 1;
        }
        x += x_slope;
    }

    return num_trees;
}

fn part1(map: &[&[u8]]) {
    let num_trees = count_trees_on_slope(map, 3, 1);
    println!("Day 3, part 1: {}", num_trees);
}

fn part2(map: &[&[u8]]) {
    let result =
        count_trees_on_slope(map, 1, 1) *
        count_trees_on_slope(map, 3, 1) *
        count_trees_on_slope(map, 5, 1) *
        count_trees_on_slope(map, 7, 1) *
        count_trees_on_slope(map, 1, 2);
    println!("Day 3, part 2: {}", result);
}
