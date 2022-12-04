use std::collections::HashMap;

/*
 * https://adventofcode.com/2022/day/2
 *
 * Play rock-paper-scissors rounds per the file.
 * Calculate the total score for the file.
 */

pub fn task_one(lines: Vec<String>) -> String {
    let response_scores: HashMap<&str, i32> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    lines
        .iter()
        .map(|line| {
            let plays: Vec<&str> = line.split(" ").collect();
            let second = plays[1];
            let result = match line.as_str() {
                "A X" => 3,
                "A Y" => 6,
                "A Z" => 0,
                "B X" => 0,
                "B Y" => 3,
                "B Z" => 6,
                "C X" => 6,
                "C Y" => 0,
                "C Z" => 3,
                _ => panic!("invalid line!"),
            };

            if let Some(score) = response_scores.get(second) {
                result + *score
            } else {
                panic!("invalid play")
            }
        })
        .sum::<i32>()
        .to_string()
}

pub fn task_two(lines: Vec<String>) -> String {
    let outcome_scores: HashMap<&str, i32> = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);

    lines
        .iter()
        .map(|line| {
            let plays: Vec<&str> = line.split(" ").collect();
            let outcome = plays[1];
            let response_score = match line.as_str() {
                "A X" => 3,
                "A Y" => 1,
                "A Z" => 2,
                "B X" => 1,
                "B Y" => 2,
                "B Z" => 3,
                "C X" => 2,
                "C Y" => 3,
                "C Z" => 1,
                _ => panic!("invalid line!"),
            };

            if let Some(score) = outcome_scores.get(outcome) {
                response_score + *score
            } else {
                panic!("invalid play")
            }
        })
        .sum::<i32>()
        .to_string()
}
