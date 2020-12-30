#[derive(Clone, Copy, PartialEq)]
enum State {
    Hash,
    Point,
    Empty,
}

#[derive(Clone, Copy, PartialEq)]
enum Orient {
    Rot0,
    Rot90,
    Rot180,
    Rot270,
    MirRot0,
    MirRot90,
    MirRot180,
    MirRot270,
}

// Rust doesn't seem to have an easy way to iterate through an enum.
const ALL_ORIENTS: [Orient; 8] = [
    Orient::Rot0,
    Orient::Rot90,
    Orient::Rot180,
    Orient::Rot270,
    Orient::MirRot0,
    Orient::MirRot90,
    Orient::MirRot180,
    Orient::MirRot270,
];

type Tile = [State; 10 * 10];
type Adjacencies = Vec<Vec<[[bool; 8]; 8]>>;

// Gets strides and bases to loop over the right edge of a tile, from top to bottom.
fn base_stride_right_edge(orient: Orient) -> (isize, isize) {
    match orient {
        Orient::Rot0 => (9, 10),
        Orient::Rot90 => (99, -1),
        Orient::Rot180 => (90, -10),
        Orient::Rot270 => (0, 1),
        Orient::MirRot0 => (0, 10),
        Orient::MirRot90 => (90, 1),
        Orient::MirRot180 => (99, -10),
        Orient::MirRot270 => (9, -1),
    }
}

fn base_stride_left_edge(orient: Orient) -> (isize, isize) {
    let new_orient = match orient {
        Orient::Rot0 => Orient::MirRot0,
        Orient::Rot90 => Orient::MirRot270,
        Orient::Rot180 => Orient::MirRot180,
        Orient::Rot270 => Orient::MirRot90,
        Orient::MirRot0 => Orient::Rot0,
        Orient::MirRot90 => Orient::Rot270,
        Orient::MirRot180 => Orient::Rot180,
        Orient::MirRot270 => Orient::Rot90,
    };
    base_stride_right_edge(new_orient)
}

fn base_stride_top_edge(orient: Orient) -> (isize, isize) {
    let new_orient = match orient {
        Orient::Rot0 => Orient::Rot270,
        Orient::Rot90 => Orient::Rot0,
        Orient::Rot180 => Orient::Rot90,
        Orient::Rot270 => Orient::Rot180,
        Orient::MirRot0 => Orient::MirRot270,
        Orient::MirRot90 => Orient::MirRot0,
        Orient::MirRot180 => Orient::MirRot90,
        Orient::MirRot270 => Orient::MirRot180,
    };
    base_stride_right_edge(new_orient)
}

fn base_stride_bottom_edge(orient: Orient) -> (isize, isize) {
    let new_orient = match orient {
        Orient::Rot0 => Orient::MirRot90,
        Orient::Rot90 => Orient::MirRot0,
        Orient::Rot180 => Orient::MirRot270,
        Orient::Rot270 => Orient::MirRot180,
        Orient::MirRot0 => Orient::Rot90,
        Orient::MirRot90 => Orient::Rot0,
        Orient::MirRot180 => Orient::Rot270,
        Orient::MirRot270 => Orient::Rot180,
    };
    base_stride_right_edge(new_orient)
}

fn cmp_horizontal(left: &Tile, right: &Tile, orient_left: Orient, orient_right: Orient) -> bool {
    let (base_left, stride_left) = base_stride_right_edge(orient_left);
    let (base_right, stride_right) = base_stride_left_edge(orient_right);
    for i in 0..10 {
        if left[(base_left + i * stride_left) as usize]
            != right[(base_right + i * stride_right) as usize]
        {
            return false;
        }
    }
    return true;
}
fn cmp_vertical(bottom: &Tile, top: &Tile, orient_bottom: Orient, orient_top: Orient) -> bool {
    let (base_bottom, stride_bottom) = base_stride_top_edge(orient_bottom);
    let (base_top, stride_top) = base_stride_bottom_edge(orient_top);
    for i in 0..10 {
        if bottom[(base_bottom + i * stride_bottom) as usize]
            != top[(base_top + i * stride_top) as usize]
        {
            return false;
        }
    }
    return true;
}

pub fn run() {
    let input_bytes = include_bytes!("day20_input");
    let input_string = String::from_utf8(input_bytes.to_vec()).unwrap();
    let lines: Vec<&str> = input_string.lines().collect();

    // TODO: Use a bitset instead?
    let mut tiles: Vec<Tile> = Vec::new();
    let mut tile_ids: Vec<usize> = Vec::new();

    for chunk in lines.chunks(12) {
        let tile_id = chunk[0]
            .trim_start_matches("Tile ")
            .trim_end_matches(':')
            .parse::<usize>()
            .unwrap();

        let mut tile = [State::Point; 100];
        for (y, line) in chunk.iter().skip(1).take(10).enumerate() {
            for (x, c) in line.chars().enumerate() {
                tile[y * 10 + x] = match c {
                    '#' => State::Hash,
                    '.' => State::Point,
                    _ => panic!("Unknown character {}", c),
                }
            }
        }

        tile_ids.push(tile_id);
        tiles.push(tile);
    }

    let num_tiles = tiles.len();
    let width = (num_tiles as f64).sqrt() as usize;

    // Compute which tiles can be on the right side of and above which ohter tiles.
    // TODO: Use only half the memory and computation time as everything is symmetric and a
    // triangular matrix is sufficient.
    // TODO: Could probably be even more compact because we should only have to compare every
    // side of every tile with every side of every other tile (4x4 for the innermost part).
    //
    // TODO: How else to make mutable Vec<Vec<...>>?
    let adjacent_right: Adjacencies = tiles
        .iter()
        .map(|tile1| {
            tiles
                .iter()
                .map(|tile2| {
                    let mut compatibility = [[false; 8]; 8];
                    for (i, orient1) in ALL_ORIENTS.iter().enumerate() {
                        for (j, orient2) in ALL_ORIENTS.iter().enumerate() {
                            compatibility[i][j] = cmp_horizontal(tile1, tile2, *orient1, *orient2);
                        }
                    }
                    return compatibility;
                })
                .collect()
        })
        .collect();
    let adjacent_top: Adjacencies = tiles
        .iter()
        .map(|tile1| {
            tiles
                .iter()
                .map(|tile2| {
                    let mut compatibility = [[false; 8]; 8];
                    for (i, orient1) in ALL_ORIENTS.iter().enumerate() {
                        for (j, orient2) in ALL_ORIENTS.iter().enumerate() {
                            compatibility[i][j] = cmp_vertical(tile1, tile2, *orient1, *orient2);
                        }
                    }
                    return compatibility;
                })
                .collect()
        })
        .collect();

    let mut grid: Vec<Vec<(isize, Orient)>> = (0..width)
        .map(|_col| (0..width).map(|_row| (-1, Orient::Rot0)).collect())
        .collect();

    part1(
        &tiles,
        &tile_ids,
        width,
        &adjacent_right,
        &adjacent_top,
        &mut grid,
    );
    part2(&tiles, &tile_ids, width, &grid);
}

// Backtrack
fn solve(
    width: usize,
    x: usize,
    y: usize,
    grid: &mut Vec<Vec<(isize, Orient)>>,
    used_tiles: &mut [bool],
    adjacent_right: &Adjacencies,
    adjacent_top: &Adjacencies,
) -> bool {
    if x == 0 && y == width {
        return true;
    }
    let x_next = if x + 1 == width { 0 } else { x + 1 };
    let y_next = if x + 1 == width { y + 1 } else { y };

    for tile_id in 0..(width * width) {
        if !used_tiles[tile_id] {
            used_tiles[tile_id] = true; // Mark tile as used.

            for orient in ALL_ORIENTS.iter() {
                // Constraint to left of new tile.
                if x > 0 {
                    let (left_id, left_orient) = grid[y][x - 1];
                    if !adjacent_right[left_id as usize][tile_id][left_orient as usize]
                        [*orient as usize]
                    {
                        continue;
                    }
                }
                // Constraint to bottom of new tile.
                if y > 0 {
                    let (bottom_id, bottom_orient) = grid[y - 1][x];
                    if !adjacent_top[bottom_id as usize][tile_id][bottom_orient as usize]
                        [*orient as usize]
                    {
                        continue;
                    }
                }

                grid[y][x] = (tile_id as isize, *orient);

                if solve(
                    width,
                    x_next,
                    y_next,
                    grid,
                    used_tiles,
                    adjacent_right,
                    adjacent_top,
                ) {
                    return true;
                }
            }

            // Tile cannot be placed here, make it availabe again.
            used_tiles[tile_id] = false;
            grid[y][x] = (-1, Orient::Rot0); // Strictly speaking not necessary.
        }
    }

    return false;
}

fn part1(
    tiles: &[Tile],
    tile_ids: &[usize],
    width: usize,
    adjacent_right: &Adjacencies,
    adjacent_top: &Adjacencies,
    grid: &mut Vec<Vec<(isize, Orient)>>,
) {
    let num_tiles = tiles.len();
    let mut used_tiles = vec![false; num_tiles];

    let success = solve(
        width,
        0,
        0,
        grid,
        &mut used_tiles,
        adjacent_right,
        adjacent_top,
    );
    assert!(success);

    let result: usize = [
        grid[0][0],
        grid[width - 1][0],
        grid[0][width - 1],
        grid[width - 1][width - 1],
    ]
    .iter()
    .map(|grid_tile| tile_ids[grid_tile.0 as usize])
    .product();
    println!("Day 20, part 1: {}", result);
}

fn part2(tiles: &[Tile], _tile_ids: &[usize], width: usize, grid: &Vec<Vec<(isize, Orient)>>) {
    let img_width = (10 - 2) * width;
    let mut image: Vec<State> = vec![State::Point; img_width * img_width];

    // Put together the full images minus the margins.
    //
    // Note that the tile grid's origin is bottom left, but the individual tiles as well as the image
    // we build here have their origin at the top left.

    // Base and stride for reading.
    fn base_stride(orient: Orient, w: usize, h: usize) -> (usize, isize, isize) {
        match orient {
            Orient::Rot0 => (0, 1, w as isize),
            Orient::Rot90 => (w - 1, w as isize, -1),
            Orient::Rot180 => (w * h - 1, -1, -(w as isize)),
            Orient::Rot270 => (w * (h - 1), -(w as isize), 1),
            Orient::MirRot0 => (w - 1, -1, w as isize),
            Orient::MirRot90 => (0, w as isize, 1),
            Orient::MirRot180 => (w * (h - 1), 1, -(w as isize)),
            Orient::MirRot270 => (w * h - 1, -(w as isize), -1),
        }
    }

    for ty in 0..width {
        for tx in 0..width {
            let tile_info = grid[width - ty - 1][tx];
            let tile_data = tiles[tile_info.0 as usize];
            let (base, stride_x, stride_y) = base_stride(tile_info.1, 10, 10);

            for y in 0..10 {
                for x in 0..10 {
                    // TODO: There would probably be a smart way to iterate only over the 8x8 grid.
                    if x > 0 && x < 9 && y > 0 && y < 9 {
                        let tile_idx: isize =
                            base as isize + (x as isize) * stride_x + (y as isize) * stride_y;
                        let img_idx = (ty * 8 + y - 1) * img_width + (tx * 8 + x - 1);
                        image[img_idx] = tile_data[tile_idx as usize];
                    }
                }
            }
        }
    }

    // Prints bottom left corner of image.
    //let show_size = 3;
    //for y in (img_width - (8 * show_size))..(img_width) {
    //    if y % 8 == 0 {
    //        for x in 0..show_size {
    //            let tile = grid[(img_width - y - 1) / 8][x];
    //            print!("{:6}@{} ", _tile_ids[tile.0 as usize], tile.1 as usize);
    //        }
    //        println!();
    //        for x in 0..show_size {
    //            let tile = grid[(img_width - y - 1) / 8][x];
    //            print!("{:6}@{} ", tile.0, tile.1 as usize);
    //        }
    //        println!();
    //    }
    //    for x in 0..(8 * show_size) {
    //        if x > 0 && x % 8 == 0 {
    //            print!(" ");
    //        }
    //        match image[y * img_width + x] {
    //            State::Hash => print!("#"),
    //            State::Point => print!("."),
    //            State::Empty => print!(" "),
    //        }
    //    }
    //    println!();
    //}

    // Find monsters.
    let monster: Vec<Vec<State>> = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ]
    .iter()
    .map(|line| {
        line.chars()
            .map(|c| match c {
                '#' => State::Hash,
                ' ' => State::Empty,
                _ => panic!("Unknown character {}", c),
            })
            .collect()
    })
    .collect();

    // Return number of '#' when a monster was found, or 0 otherwise.
    // 0 '#'s and no monster are indistinguishable, doesn't matter.
    fn scan_monster(
        image: &[State],
        monster: &[Vec<State>],
        base: usize,
        stride_x: isize,
        stride_y: isize,
    ) -> bool {
        let monster_width = monster[0].len();
        let monster_height = monster.len();

        for y in 0..monster_height {
            for x in 0..monster_width {
                let i_idx = base as isize + x as isize * stride_x + y as isize * stride_y;

                let m_state = monster[y][x];
                let i_state = image[i_idx as usize];
                if m_state == State::Hash && i_state != State::Hash {
                    return false;
                }
            }
        }
        return true;
    }

    let monster_width = monster[0].len();
    let monster_height = monster.len();
    let mut num_monsters = 0;
    for mo in ALL_ORIENTS.iter() {
        let (base, stride_x, stride_y) = base_stride(*mo, img_width, img_width);

        for y in 0..(img_width - monster_height + 1) {
            for x in 0..(img_width - monster_width + 1) {
                let offset =
                    (base as isize + x as isize * stride_x + y as isize * stride_y) as usize;
                if scan_monster(&image, &monster, offset, stride_x, stride_y) {
                    num_monsters += 1;
                }
            }
        }
    }

    let num_hashes = image.iter().filter(|f| **f == State::Hash).count();
    let num_hashes_monster = monster
        .iter()
        .flat_map(|l| l.iter())
        .filter(|f| **f == State::Hash)
        .count();

    let result = num_hashes - num_monsters * num_hashes_monster;
    println!("Day 20, part 2: {}", result);
}
