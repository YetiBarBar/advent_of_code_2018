use std::collections::HashMap;

struct WorkStep1 {
    map: HashMap<char, Vec<char>>,
    done: Vec<char>,
}

impl WorkStep1 {
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

    fn complete_job(&mut self, job: char) {
        self.done.push(job);
    }
}

impl Iterator for WorkStep1 {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.candidates().first().copied();
        if let Some(chr) = val {
            self.complete_job(chr);
        }
        val
    }
}

/* struct WorkStep2 {
    map: HashMap<char, Vec<char>>,
    done: HashMap<char, usize>,
    current_time: usize,
    worker_count: usize,
}

impl WorkStep2 {
    fn new(map: &HashMap<char, Vec<char>>) -> Self {
        Self {
            map: map.clone(),
            done: HashMap::new(),
            current_time: 0,
            worker_count: 0,
        }
    }

    fn candidates(&self) -> Vec<char> {
        let mut values = self
            .map
            .iter()
            .filter(|(chr, _)| self.done.get(*chr).unwrap_or(&usize::MAX) > &self.current_time)
            .filter(|(_, v)| {
                v.iter()
                    .all(|item| self.done.get(item).unwrap_or(&usize::MAX) > &self.current_time)
            })
            .map(|(chr, _)| chr)
            .copied()
            .collect::<Vec<_>>();
        values.sort_unstable();
        values
    }

    fn next_second(&mut self) {
        let candidates = self.candidates();
        let to_process_count = candidates.len().min(5 - self.worker_count);
        println!(
            "Second: {} => {} items",
            self.current_time,
            candidates.len()
        );
        for &val in candidates.iter().take(to_process_count) {
            println!(
                "Second: {} => {} items",
                self.current_time, to_process_count
            );
            self.done
                .insert(val, self.current_time + (val as usize - 'A' as usize) + 1);
        }
        self.current_time += 1;
    }

    fn has_finished(&self) -> bool {
        self.done.len() == self.map.len()
            && self.done.values().all(|time| time > &self.current_time)
    }
} */

fn main() {
    let input = include_str!("../data/day7.data");
    let hmap = parse_input(input);

    let worker = WorkStep1::new(&hmap);
    let res: String = worker.collect();
    println!("Part 1: {}", res);

    /*     let mut worker2 = WorkStep2::new(&hmap);
    for second in 0.. {
        if worker2.has_finished() {
            println!("Part 2: {}", second);
            break;
        }
        worker2.next_second();
    } */
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
