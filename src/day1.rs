use std::collections::HashSet;

fn main() {
    let data = include_str!("../data/day1.data");
    let res: isize = data
        .lines()
        .map(|line| line.trim_start_matches('+'))
        .map(str::parse::<isize>)
        .map(Result::unwrap)
        .sum();
    println!("Part 1: {}", res);

    let sequence: Vec<_> = data
        .lines()
        .map(|line| line.trim_start_matches('+'))
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut last: isize = 0;
    let mut set = HashSet::new();
    for chg in sequence.iter().cycle() {
        last += chg;
        if set.contains(&last) {
            println!("Part 2: {}", last);
            break;
        }
        set.insert(last);
    }
}
