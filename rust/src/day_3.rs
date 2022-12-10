/*
 * https://adventofcode.com/2022/day/3
 *
 * Split strings midway. Identify shared item.
 * Return sum of values for shared items.
 */

use std::collections::HashSet;

fn get_priority(letter: char) -> u32 {
    match letter {
        'a'..='z' => (letter as u32 - 'a' as u32) + 1,
        'A'..='Z' => (letter as u32 - 'A' as u32) + 27,
        _ => panic!("Invalid letter {} is not in range a..=z, A..=Z", letter),
    }
}

pub fn task_one(lines: Vec<String>) -> String {
    let sum: u32 = lines
        .iter()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);
            let first_chars: HashSet<char> = HashSet::from_iter(first.chars());
            let second_chars: HashSet<char> = HashSet::from_iter(second.chars());

            if let Some(matching) = first_chars.intersection(&second_chars).nth(0) {
                get_priority(*matching)
            } else {
                panic!("panik! no shared elements")
            }
        })
        .sum();

    sum.to_string()
}
