use super::utils;
// use std::{collections::HashMap, ops::Index};

const DAY: i32 = 5;

pub fn solve(use_dummy_inputs: bool) {
    utils::solve(part01, part02, DAY, use_dummy_inputs)
}

fn part01(_inputs: &str) {
    const MIN_VOWELS: usize = 3;
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const BLOCKED_STRINGS: [&str; 4] = ["ab", "cd", "pq", "xy"];

    let mut valid_count: i32 = 0;
    let mut vowel_count: usize;
    let mut duplicated_counts: i32;
    // let mut char_count: HashMap<char, i32> = HashMap::new();

    let mut has_enough_vowels: bool;
    let mut has_duplicate_chars: bool;
    let mut has_blocked_strings: bool;

    for test_string in _inputs.split_whitespace() {
        // char_count.clear();

        // vVv this is actually unique count
        // vowel_count = VOWELS.iter().map(|v| test_string.contains(*v) as i32).sum();
        vowel_count = test_string.chars().filter(|c| VOWELS.contains(c)).count();

        duplicated_counts = 0;
        // Feels a bit hacky to compare sequential elements like this + introduce a side-effect
        // variable access into a supposedly functional iterator 😂
        test_string.chars().reduce(|acc, e| {
            if acc == e {
                duplicated_counts += 1;
            }
            e
        });
        has_duplicate_chars = duplicated_counts > 0;

        // vVv this was doing counts irrespective of position, we actually only
        //     really care about sequential duplicates
        // for (c) in test_string.chars() {
        // char_count
        //     .entry(c)
        //     .and_modify(|counter| *counter += 1)
        //     .or_insert(1);
        // }
        // has_duplicate_chars = *char_count.values().max().unwrap_or(&0) > 1;

        has_enough_vowels = vowel_count >= MIN_VOWELS;
        has_blocked_strings = BLOCKED_STRINGS
            .iter()
            .filter(|substr| test_string.contains(*substr))
            .count()
            > 0;

        if has_enough_vowels && has_duplicate_chars && !has_blocked_strings {
            println!("string \"{}\" is valid", test_string);
            valid_count += 1;
        }
    }

    println!("There are {} valid strings", valid_count);
}

fn part02(_inputs: &str) {
    println!("unimplemented");
}
