struct Polymer {
    data: Vec<char>,
}

impl Polymer {
    fn polymerize(&mut self) -> bool {
        let mut reduced = false;
        let mut last: Option<char> = None;
        let mut v = Vec::new();
        for chr in &self.data {
            if let Some(old) = last {
                if &old != chr && old.to_ascii_lowercase() == chr.to_ascii_lowercase() {
                    reduced = true;
                    last = None;
                } else {
                    v.push(old);
                    last = Some(*chr);
                }
            } else {
                last = Some(*chr);
            }
        }
        if let Some(val) = last {
            v.push(val);
        }
        self.data = v;
        reduced
    }
}

fn evaluate_removed_letter(polymer: &Polymer, letter: char) -> usize {
    let mut new_poly = Polymer {
        data: polymer
            .data
            .iter()
            .filter(|&&chr| {
                chr != letter.to_ascii_lowercase() && chr != letter.to_ascii_uppercase()
            })
            .copied()
            .collect(),
    };
    loop {
        if !new_poly.polymerize() {
            break;
        }
    }
    new_poly.data.len()
}

fn main() {
    let data = include_str!("../data/day5.data");
    let mut polymer = Polymer {
        data: data.trim().chars().collect(),
    };

    loop {
        if !polymer.polymerize() {
            break;
        }
    }
    println!("Part 1: {}", polymer.data.iter().collect::<String>().len());

    let res = ('a'..='z')
        .map(|letter| evaluate_removed_letter(&polymer, letter))
        .min();
    println!("Part 2: {}", res.unwrap());
}
