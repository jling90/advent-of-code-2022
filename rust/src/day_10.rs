pub fn task_one(lines: Vec<String>) -> String {
    let mut cycles: Vec<i32> = vec![1];

    for line in lines {
        let last = *cycles.last().unwrap_or(&1);
        match line.as_str() {
            "noop" => cycles.push(last),
            instr => {
                let mut splits = instr.split(" ");

                // Discard "addx"
                splits.next();

                let modifier: i32 = splits.next().unwrap().parse().unwrap();
                cycles.push(last);
                cycles.push(last + modifier);
            }
        };
    }

    let mut sum = 0;

    for idx in [20, 60, 100, 140, 180, 220] {
        sum = sum + cycles[idx - 1] * (idx as i32);
    }

    sum.to_string()
}
