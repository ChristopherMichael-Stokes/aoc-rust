use regex::Regex;
use std::ops::Index;
use std::ops::IndexMut;

use super::utils;

const DAY: i32 = 6;

pub fn solve(use_dummy_inputs: bool) {
    utils::solve(part01, part02, DAY, use_dummy_inputs)
}

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

// Feels like this struct adds bloat for no real value, would ideally not include this,
// but keeping for reference on using structs
struct Grid<T> {
    shape: Vec<usize>,
    buf: Vec<T>,
}

impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.buf[index]
    }
}

fn make_grid(n_rows: usize, n_cols: usize) -> Grid<i32> {
    Grid {
        shape: vec![n_rows, n_cols],
        buf: vec![0; n_rows * n_cols],
    }
}

fn part01(_inputs: &str) {
    const GRID_SIZE: usize = 1000;
    let mut grid = make_grid(GRID_SIZE, GRID_SIZE);

    let re = Regex::new(
        r"(?<action>turn on|turn off|toggle) (?<x1>\d+),(?<y1>\d+) through (?<x2>\d+),(?<y2>\d+)",
    )
    .unwrap();
    // let re = Regex::new(r"Hello (?<name>\w+)!").unwrap();
    for instruction in _inputs.lines() {
        let Some(caps) = re.captures(instruction) else {
            panic!("Invalid instruction in string {}", instruction);
        };

        let x1 = caps["x1"].parse::<usize>().unwrap();
        let y1 = caps["y1"].parse::<usize>().unwrap();
        let x2 = caps["x2"].parse::<usize>().unwrap();
        let y2 = caps["y2"].parse::<usize>().unwrap();

        let action_str = &caps["action"];
        let action = match &caps["action"] {
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            "toggle" => Action::Toggle,
            _ => panic!(
                "Unexpected action found in input {}, action -> {}",
                instruction, action_str
            ),
        };

        for i in x1..x2 + 1 {
            for j in y1..y2 + 1 {
                let idx = i * grid.shape[1] + j;
                grid[idx] = match action {
                    Action::Toggle => {
                        if grid[idx] == 1 {
                            0
                        } else {
                            1
                        }
                    }
                    Action::TurnOff => 0,
                    Action::TurnOn => 1,
                }
            }
        }
    }
    let on: i32 = grid.buf.iter().sum();
    println!("Number of lights on: {}", on);
}

fn part02(_inputs: &str) {
    const GRID_SIZE: usize = 1000;
    let mut grid = make_grid(GRID_SIZE, GRID_SIZE);

    let re = Regex::new(
        r"(?<action>turn on|turn off|toggle) (?<x1>\d+),(?<y1>\d+) through (?<x2>\d+),(?<y2>\d+)",
    )
    .unwrap();

    for instruction in _inputs.lines() {
        let Some(caps) = re.captures(instruction) else {
            panic!("Invalid instruction in string {}", instruction);
        };

        let x1 = caps["x1"].parse::<usize>().unwrap();
        let y1 = caps["y1"].parse::<usize>().unwrap();
        let x2 = caps["x2"].parse::<usize>().unwrap();
        let y2 = caps["y2"].parse::<usize>().unwrap();

        let action_str = &caps["action"];
        let action = match &caps["action"] {
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            "toggle" => Action::Toggle,
            _ => panic!(
                "Unexpected action found in input {}, action -> {}",
                instruction, action_str
            ),
        };

        for i in x1..x2 + 1 {
            for j in y1..y2 + 1 {
                let idx = i * grid.shape[1] + j;
                grid[idx] += match action {
                    Action::Toggle => 2,
                    Action::TurnOff => {
                        if grid[idx] > 0 {
                            -1
                        } else {
                            0
                        }
                    }
                    Action::TurnOn => 1,
                }
            }
        }
    }

    let brightness: i32 = grid.buf.iter().sum();
    println!("Total brightness of all lights: {}", brightness);
}
