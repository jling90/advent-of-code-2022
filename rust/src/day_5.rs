use std::{fs::File, io};

// https://stackoverflow.com/a/64499219
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

// TODO: add an implementation which stores stacks as a 1D vec.
fn process_inputs(
    lines: io::Lines<io::BufReader<File>>,
) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut lines_iter = lines.into_iter();
    let mut stacks: Vec<Vec<char>> = vec![];

    while let Some(res_row) = lines_iter.next() {
        let row = res_row.unwrap();
        if row.is_empty() {
            break;
        };

        stacks.push(
            row.chars()
                .enumerate()
                .filter(|(idx, _ch)| *idx >= 1 && (*idx - 1) % 4 == 0)
                .map(|(_idx, ch)| ch)
                .collect(),
        )
    }

    // Drop numbers row
    stacks.pop();

    let instructions: Vec<(usize, usize, usize)> = lines_iter
        .map(|l| {
            let mut ins = l
                .as_ref()
                .unwrap()
                .split(" ")
                .enumerate()
                .filter_map(|(idx, word)| {
                    if idx % 2 == 1 {
                        Some(word.parse::<usize>().unwrap())
                    } else {
                        None
                    }
                });

            (
                ins.next().unwrap(),
                ins.next().unwrap(),
                ins.next().unwrap(),
            )
        })
        .collect();

    // Transpose rows of letters into columns and remove whitespaces
    let trimmed_stacks: Vec<Vec<char>> = transpose(stacks)
        .iter_mut()
        .map(|stack| {
            stack
                .into_iter()
                .filter_map(|c| if c.is_whitespace() { None } else { Some(*c) })
                .rev()
                .collect()
        })
        .collect();

    (trimmed_stacks, instructions)
}

pub fn task_one(lines: io::Lines<io::BufReader<File>>) -> String {
    let (mut stacks, instructions) = process_inputs(lines);

    for (count, from, to) in instructions {
        for _ in 0..count {
            if let Some(c) = stacks[from - 1].pop() {
                stacks[to - 1].push(c);
            }
        }
    }

    stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect()
}

pub fn task_two(lines: io::Lines<io::BufReader<File>>) -> String {
    let (mut stacks, instructions) = process_inputs(lines);

    for (count, from, to) in instructions {
        let cursor = stacks[from - 1].len() - count;
        let mut drained = stacks[from - 1].drain(cursor..).collect();

        stacks[to - 1].append(&mut drained);
    }

    stacks
        .iter_mut()
        .map(|stack| stack.pop().unwrap())
        .collect()
}
