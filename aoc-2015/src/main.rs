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

    match args[1].as_str() {
        "day01" => day01::solve(use_dummy_inputs),
        "day02" => day02::solve(use_dummy_inputs),
        _ => eprintln!("Solutions not yet implemented for \"{}\"", args[1]),
    }
    
}
