use std::collections::HashMap;

struct Worker {
    map: HashMap<char, Vec<char>>,
    done: Vec<char>,
}

impl Worker {
    fn new(map: &HashMap<char, Vec<char>>) -> Self {
        Self {
            map: map.clone(),
            done: Vec::new(),
        }
    }

    fn candidates(&self) -> Vec<char> {
        let mut values = self
            .map
            .iter()
            .filter(|(chr, _)| !self.done.contains(*chr))
            .filter(|(_, v)| v.iter().all(|item| self.done.contains(item)))
            .map(|(chr, _)| chr)
            .copied()
            .collect::<Vec<_>>();
        values.sort_unstable();
        values
    }
}

impl Iterator for Worker {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.candidates().first().copied();
        if let Some(chr) = val {
            self.done.push(chr);
        }
        val
    }
}

fn main() {
    let input = include_str!("../data/day7.data");
    let hmap = parse_input(input);
    // println!("{:#?}", hmap);
    //println!("{:#?}", steps.len());

    let worker = Worker::new(&hmap);
    let res: String = worker.collect();
    println!("Part 1: {}", res);
}

fn parse_input(data: &str) -> HashMap<char, Vec<char>> {
    data.lines().fold(HashMap::new(), |mut acc, line| {
        let splits: Vec<_> = line.split_ascii_whitespace().collect();

        let chr1 = splits[1].chars().next().unwrap();
        let chr2 = splits[7].chars().next().unwrap();

        acc.entry(chr2).or_insert_with(Vec::new).push(chr1);
        acc.entry(chr1).or_insert_with(Vec::new);
        acc
    })
}

// fn runnable_task(world: HashMap, )
