use super::utils;
// use std::{collections::HashMap, ops::Index};

const DAY: i32 = 5;
const DEBUG_LEVEL: i8 = 0;

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
            // println!("string \"{}\" is valid", test_string);
            valid_count += 1;
        }
    }

    println!("There are {} valid strings", valid_count);
}

fn part02(_inputs: &str) {
    const N_CHARS: usize = 2;
    let mut valid_count: i32 = 0;
    let mut str_len: usize;

    let mut has_duplicate_pairs: bool;
    let mut has_palindrome_match: bool;

    for test_string in _inputs.split_whitespace() {
        if DEBUG_LEVEL > 1 {
            println!("Checking string - {}", test_string);
        }
        let str_buf: Vec<char> = test_string.chars().collect();

        str_len = str_buf.len();
        has_duplicate_pairs = false;
        has_palindrome_match = false;

        // Want to find a pair of characters that appears at least twice and does not overlap
        // the matching pair indices
        // Use double index iteration to left -> right 1. select a pair for consideration
        // then 2. slide over the rest of the string to check for matches
        'pair_matching: for i in 1..str_len {
            for j in i + 1..(str_len - 1) {
                // Both chars from the first pair match the second pair
                if str_buf[i - 1] == str_buf[j] && str_buf[i] == str_buf[j + 1] {
                    has_duplicate_pairs = true;
                    if DEBUG_LEVEL > 0 {
                        println!(
                            "Duplicate pairs found in {}, {}{} - {}{}",
                            test_string,
                            str_buf[i - 1],
                            str_buf[i],
                            str_buf[j],
                            str_buf[j + 1]
                        );
                    }
                    break 'pair_matching;
                }
            }

            // don't use the last N characters as the first pair
            // otherwise there won't be anything else to iterate over
            if i >= str_len - (N_CHARS + 1) {
                // Again took ages to figure out but this ^ caused an off by 1 error as I had a string
                // pair "...aczcz" at the end of the input that did not get checked.
                //     - the last pairs considered were ac & zc
                // Moving this bounds check here to the end of the loop fixed it 🥲.
                break;
            }
        }

        // Check if there is a 3 character palindrome
        'palindrome_match: for i in 2..str_len {
            if str_buf[i - 2] == str_buf[i] {
                has_palindrome_match = true;
                if DEBUG_LEVEL > 0 {
                    println!(
                        "Palindrome match found in {}, {:?}",
                        test_string,
                        &str_buf[i - 2..i + 1].iter().collect::<String>(),
                    );
                }
                break 'palindrome_match;
            }
        }

        if has_duplicate_pairs && has_palindrome_match {
            valid_count += 1;
        }
    }

    println!("There are {} valid strings", valid_count);
}
