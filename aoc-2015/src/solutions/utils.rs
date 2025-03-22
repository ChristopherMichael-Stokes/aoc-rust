use std::fs;
use std::io;
use std::path::Path;

const INPUT_BASE: &str = "inputs";

fn format_path(day_number: i32, use_dummy_inputs: bool) -> String {
    format!(
        "day{:0>2}{}.txt",
        day_number,
        if use_dummy_inputs { "_dummy" } else { "" }
    )
}

fn get_input_contents(day_number: i32, use_dummy_inputs: bool) -> io::Result<String> {
    let file_path = Path::new(INPUT_BASE).join(format_path(day_number, use_dummy_inputs));
    fs::read_to_string(file_path)
}

pub(super) fn solve<F, G>(part01: F, part02: G, day_number: i32, use_dummy_inputs: bool)
where
    F: Fn(&str),
    G: Fn(&str),
{
    if let Ok(inputs) = get_input_contents(day_number, use_dummy_inputs) {
        println!("Inputs day {}:\n{}", day_number, inputs);
        let cleaned_inputs = inputs.trim();
        part01(cleaned_inputs);
        part02(cleaned_inputs);
    }
}
