use std::env;
mod day_1;
mod day_2;
mod util;

const DAY_1_FILE_PATH: &str = "./resources/day_1";
const DAY_2_FILE_PATH: &str = "./resources/day_2";

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num = &args[1];
    let task_num = &args[2];

    match day_num.as_str() {
        "1" => match task_num.as_str() {
            "1" => {
                if let Ok(lines) = util::read_lines(DAY_1_FILE_PATH) {
                    println!("{}", day_1::task_one(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            "2" => {
                if let Ok(lines) = util::read_lines(DAY_1_FILE_PATH) {
                    println!("{}", day_1::task_two(lines));
                } else {
                    println!("Failed to read file");
                }
            }
            _ => println!("Task not implemented"),
        },
        "2" => match task_num.as_str() {
            "1" => {
                println!(
                    "{}",
                    day_2::task_one(util::read_lines_to_string_vec(DAY_2_FILE_PATH))
                )
            }
            _ => println!("Task not implemented"),
        },
        _ => println!("No match for exercise {}, task {}", day_num, task_num),
    }
}
