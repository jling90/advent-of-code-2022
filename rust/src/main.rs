use std::env;
mod day_1;
mod day_2;
mod util;

fn main() {
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
        _ => println!("No match for exercise {}, task {}", day_num, task_num),
    }
}
