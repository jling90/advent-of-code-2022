fn pairs_from(line: &str) -> (u32, u32) {
    let mut split = line.split("-");
    (
        split.next().unwrap().parse::<u32>().unwrap(),
        split.next().unwrap().parse::<u32>().unwrap(),
    )
}

pub fn task_one(lines: Vec<String>) -> String {
    lines
        .iter()
        .map(|line| {
            let mut split = line.split(",");
            let (a1, a2) = pairs_from(split.next().unwrap());
            let (b1, b2) = pairs_from(split.next().unwrap());
            let ra = a1..=a2;
            let rb = b1..=b2;

            if ra.contains(&b1) && ra.contains(&b2) || rb.contains(&a1) && rb.contains(&a2) {
                1
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}
