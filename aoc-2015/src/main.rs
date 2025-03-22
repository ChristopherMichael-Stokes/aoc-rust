use std::env;

mod solutions;

use crate::solutions::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter the day to run solutions for, e.g. \"day01\" to run the first day");
        return;
    }

    let use_dummy_inputs = args.len() > 2
        && ["true", "True", "y", "Y", "1"]
            .iter()
            .any(|&item| args[2].contains(item));

    let problem: &String = &args[1];
    if problem == "day01" {
        day01::solve(use_dummy_inputs);
    } else if problem == "day02" {
        day02::solve(use_dummy_inputs);
    } else {
        println!("Solutions not yet implemented for \"{}\"", problem);
    }
}
