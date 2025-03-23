use super::utils;
use md5;
use str;

const DAY: i32 = 4;

pub fn solve(use_dummy_inputs: bool) {
    utils::solve(part01, part02, DAY, use_dummy_inputs)
}

fn match_zeros(base_chars: &str, n_zeros: usize) -> u64 {
    let match_str = "0".repeat(n_zeros);
    let mut numerical_suffix: u64 = 0;

    // Took me ages to figure out the syntax for this stuff and it turns out
    // to reduce to 4 lines 🥲
    loop {
        let data = format!("{}{}", base_chars, numerical_suffix);
        let hash = md5::compute(data);
        let hash_str = format!("{:02x}", hash);
        if hash_str.starts_with(&match_str) {
            break;
        }
        numerical_suffix += 1;
    }

    numerical_suffix
}
fn part01(_inputs: &str) {
    println!("Numerical suffix: {}", match_zeros(_inputs, 5));
}

fn part02(_inputs: &str) {
    println!("Numerical suffix: {}", match_zeros(_inputs, 6));
}
