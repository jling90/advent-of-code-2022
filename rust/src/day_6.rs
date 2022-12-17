fn has_dupes<T: std::cmp::PartialEq>(slice: &[T]) -> bool {
    (1..slice.len()).any(|i| slice[i..].contains(&slice[i - 1]))
}

fn solve(line: &str, window_size: usize) -> String {
    let chars: Vec<char> = line.chars().collect();
    let windows = chars.windows(window_size);
    let mut last_idx: Option<usize> = None;

    for (idx, window) in windows.enumerate() {
        last_idx = Some(idx + window_size);
        if !has_dupes(window) {
            break;
        }
    }

    last_idx.unwrap().to_string()
}

pub fn task_one(line: &str) -> String {
    solve(line, 4)
}

pub fn task_two(line: &str) -> String {
    solve(line, 14)
}
