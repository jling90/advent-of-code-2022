use std::{env, time::Instant};
mod day_1;
mod day_10;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod util;

fn main() {
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();
    let day_num = &args[1];
    let task_num = &args[2];

    let filename = format!("./resources/day_{}", day_num);

    match format!("{} {}", day_num, task_num).as_str() {
        "1 1" => {
            if let Ok(lines) = util::read_lines(filename) {
                println!("{}", day_1::task_one(lines));
            } else {
                println!("Failed to read file");
            }
        }
        "1 2" => {
            if let Ok(lines) = util::read_lines(filename) {
                println!("{}", day_1::task_two(lines));
            } else {
                println!("Failed to read file");
            }
        }
        "2 1" => {
            println!(
                "{}",
                day_2::task_one(util::read_lines_to_string_vec(filename))
            )
        }
        "2 2" => {
            println!(
                "{}",
                day_2::task_two(util::read_lines_to_string_vec(filename))
            )
        }
        "3 1" => {
            println!(
                "{}",
                day_3::task_one(util::read_lines_to_string_vec(filename))
            )
        }
        "3 2" => {
            if let Some(answer) = day_3::task_two(util::read_lines_to_string_vec(filename)) {
                println!("{}", answer)
            }
        }
        "4 1" => {
            println!(
                "{}",
                day_4::task_one(util::read_lines_to_string_vec(filename))
            )
        }
        "4 2" => {
            println!(
                "{}",
                day_4::task_two(util::read_lines_to_string_vec(filename))
            )
        }
        "5 1" => {
            if let Ok(lines) = util::read_lines(filename) {
                println!("{}", day_5::task_one(lines));
            } else {
                println!("Failed to read file");
            }
        }

        "5 2" => {
            if let Ok(lines) = util::read_lines(filename) {
                println!("{}", day_5::task_two(lines));
            } else {
                println!("Failed to read file");
            }
        }

        "6 1" => {
            println!(
                "{}",
                day_6::task_one(util::read_lines_to_string_vec(filename).get(0).unwrap())
            )
        }

        "6 2" => {
            println!(
                "{}",
                day_6::task_two(util::read_lines_to_string_vec(filename).get(0).unwrap())
            )
        }

        "7 1" => {
            println!(
                "{}",
                day_7::task_one(util::read_lines_to_string_vec(filename))
            )
        }

        "7 2" => {
            println!(
                "{}",
                day_7::task_two(util::read_lines_to_string_vec(filename))
            )
        }

        "10 1" => {
            println!(
                "{}",
                day_10::task_one(util::read_lines_to_string_vec(filename))
            )
        }

        _ => println!("No match for exercise {}, task {}", day_num, task_num),
    }

    println!("Elapsed: {:.2?}", now.elapsed());
}
