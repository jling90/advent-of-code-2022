use std::cmp;
use std::fs::File;
use std::io::{self};

/*
 * https://adventofcode.com/2022/day/1
 *
 * Read through a file, sum contiguous rows of ints
 * and return the max sum.
 */

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut sum = 0;
    let mut max_sum = 0;
    for line in lines {
        if let Ok(parsed) = line {
            if let Ok(n) = parsed.parse::<i32>() {
                sum = n + sum;
            } else {
                max_sum = cmp::max(max_sum, sum);
                sum = 0;
            }
        }
    }

    max_sum.to_string()
}
