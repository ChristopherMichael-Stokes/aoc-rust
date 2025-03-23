use super::utils;

const DAY: i32 = 2;

pub fn solve(use_dummy_inputs: bool) {
    utils::solve(part01, part02, DAY, use_dummy_inputs)
}

fn part01(_inputs: &str) {
    let mut sizes: [i32; 3] = [0; 3];
    let mut idx: usize;
    let mut res: i32;
    let mut sum: i32 = 0;

    for line in _inputs.split_whitespace() {
        idx = 0;
        // Would be nice to figure out a functional + iterator based way to do this rather than for loop
        for dim in line.split("x") {
            sizes[idx] = dim.parse().unwrap();
            idx += 1;
        }
        sizes.sort(); // Get smallest two dimensions first
        res = 2 * sizes[0] * sizes[1] + 2 * sizes[0] * sizes[2] + 2 * sizes[1] * sizes[2];
        res += sizes[0] * sizes[1];
        sum += res;
    }

    println!("Total square ft: {}", sum);
}

fn part02(_inputs: &str) {
    let mut sizes: [i32; 3] = [0; 3];
    let mut idx: usize;
    let mut res: i32;
    let mut sum: i32 = 0;

    for line in _inputs.split_whitespace() {
        idx = 0;
        for dim in line.split("x") {
            sizes[idx] = dim.parse().unwrap();
            idx += 1;
        }
        sizes.sort(); // Get smallest two dimensions first
        res = 2 * sizes[0] + 2 * sizes[1];
        res += sizes[0] * sizes[1] * sizes[2];
        sum += res;
    }

    println!("Total square ft: {}", sum);
}
