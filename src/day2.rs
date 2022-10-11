use std::collections::HashMap;

fn compute_box_step_1(line: &str) -> (isize, isize) {
    let mut hmap: HashMap<_, isize> = HashMap::new();
    for chr in line.chars() {
        *hmap.entry(chr).or_default() += 1_isize;
    }
    (
        hmap.values().any(|val| val == &2).into(),
        hmap.values().any(|val| val == &3).into(),
    )
}

fn distance(line1: &str, line2: &str) -> usize {
    line1
        .chars()
        .zip(line2.chars())
        .filter(|(chr1, chr2)| chr1 != chr2)
        .count()
}

fn common(line1: &str, line2: &str) -> String {
    line1
        .chars()
        .zip(line2.chars())
        .filter(|(chr1, chr2)| chr1 == chr2)
        .map(|(chr, _)| chr)
        .collect()
}

fn main() {
    let data = include_str!("../data/day2.data");
    let values = data
        .lines()
        .map(compute_box_step_1)
        .fold((0_isize, 0_isize), |(a, b), (c, d)| (a + c, b + d));
    println!("Step 1: {}", values.0 * values.1);

    for line1 in data.lines() {
        for line2 in data.lines() {
            if distance(line1, line2) == 1 {
                println!("Part 2 candidates: {} <=> {}", line1, line2);
                println!("Part 2: {}", common(line1, line2));
            }
        }
    }
}
