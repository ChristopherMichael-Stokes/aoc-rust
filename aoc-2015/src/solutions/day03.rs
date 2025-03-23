use super::utils;

const DAY: i32 = 3;

pub fn solve(use_dummy_inputs: bool) {
    utils::solve(part01, part02, DAY, use_dummy_inputs)
}

fn point_to_scalar_pos(x: usize, y: usize, _n_rows: usize, n_cols: usize) -> usize {
    x * n_cols + y
}

fn scalar_pos_to_point(pos: usize, _n_rows: usize, n_cols: usize) -> (usize, usize) {
    (pos / n_cols, pos % n_cols)
}

fn walk_movement(move_char: char, mut x: usize, mut y: usize) -> (usize, usize) {
    match move_char {
        '^' => y -= 1,
        'v' => y += 1,
        '<' => x += 1,
        '>' => x -= 1,
        _ => panic!("Invalid input: {}", move_char),
    }

    (x, y)
}

fn part01(_inputs: &str) {
    let n_rows: usize = _inputs.len() * 2 + 1;
    let n_cols: usize = n_rows;
    let grid_len = n_rows * n_cols; // Guaranteed to be odd by +1
    let mut santa_grid: Vec<i32> = vec![0; grid_len];
    // Each character can only move one position and at worst case we move entirely in one direction
    // so setting each side of the center to that total seems safe enough
    dbg!(santa_grid.len());
    dbg!(n_rows);
    dbg!(_inputs.len());

    let mut santa_pos: usize = (santa_grid.len() - 1) / 2;
    let (mut x, mut y) = scalar_pos_to_point(santa_pos, n_rows, n_cols);
    santa_grid[santa_pos] = 1;

    dbg!(santa_pos);
    dbg!((x, y));

    /*
    If we have an input of size 2, we get a ((2 * 2) + 1) * 2 === 25 sized grid
    like below:

    // 00000   5
    // 00000  10
    // 00000  15
    // 00000  20
    // 00000  25

    the center point is (25 - 1) // 2 === 12.

    Now we need a formula to convert any 2d movement into a 1d int as our internal
    representation is a contiguous array ...
    i.e. center or [2,2] === 12, middle top [0,2] === 2,
         bottom middle [4,2] === 22, etc.

    Think in C with pointer array it's something like arr_width * i + j??

    so
        z = W * i + j
    and in reverse
        i,j == ??
            j = z - W *i
            i = (z - j) / W

        logically i = z / W and j = z % W, but not sure on how to prove this?
     */
    for move_char in _inputs.chars() {
        (x, y) = walk_movement(move_char, x, y);
        santa_pos = point_to_scalar_pos(x, y, n_rows, n_cols);
        santa_grid[santa_pos] += 1;
    }

    let unique_visits = santa_grid
        .iter()
        .fold(0, |acc, e| acc + if *e != 0 { 1 } else { 0 });

    println!("Santa visits {}, unique addresses.", unique_visits);
}

fn part02(_inputs: &str) {
    let n_rows: usize = _inputs.len() * 2 + 1;
    let n_cols: usize = n_rows;
    let grid_len = n_rows * n_cols; // Guaranteed to be odd by +1
    let mut santa_grid: Vec<i32> = vec![0; grid_len];
    dbg!(santa_grid.len());
    dbg!(n_rows);
    dbg!(_inputs.len());

    let santa_pos: usize = (santa_grid.len() - 1) / 2;
    let (mut x, mut y) = scalar_pos_to_point(santa_pos, n_rows, n_cols);
    let (mut x2, mut y2) = (x, y);
    santa_grid[santa_pos] = 1;

    for (idx, move_char) in _inputs.char_indices() {
        if idx % 2 == 0 {
            (x, y) = walk_movement(move_char, x, y);
            santa_grid[point_to_scalar_pos(x, y, n_rows, n_cols)] += 1;
        } else {
            (x2, y2) = walk_movement(move_char, x2, y2);
            santa_grid[point_to_scalar_pos(x2, y2, n_rows, n_cols)] += 1;
        }
    }

    let unique_visits = santa_grid
        .iter()
        .fold(0, |acc, e| acc + if *e != 0 { 1 } else { 0 });

    println!("Santa visits {}, unique addresses.", unique_visits);
}
