use super::utils;

const DAY: i32 = 1;

pub fn solve(use_dummy_inputs: bool) {
    utils::solve(part01, part02, DAY, use_dummy_inputs)
}

fn part01(_inputs: &str) {
    let mut increments: i32 = 0;
    let mut decrements: i32 = 0;

    for c in _inputs.chars() {
        if c == '(' {
            increments += 1;
        } else if c == ')' {
            decrements += 1;
        }
    }

    println!("Solution to part 1, {}", increments - decrements);
}

fn part02(_inputs: &str) {
    let mut pos: i32 = 0;
    let mut found_idx: i32 = -1;

    for (i, c) in _inputs.char_indices() {
        pos += if c == '(' { 1 } else { -1 };
        if pos == -1 {
            found_idx = (i as i32) + 1; // floor indexing starts at 1
            break;
        }
    }

    println!("Solution to part 2, {}", found_idx);
}
