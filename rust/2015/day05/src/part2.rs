use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Eq)]
struct Pair(char, char, usize, usize);

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Hash for Pair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

pub fn solve(input: &String) {
    let mut count = 0;

    for str in input.lines() {
        if is_nice(str) {
            count += 1;
        }
    }

    println!("Part 2: {count}");
}

fn is_nice(str: &str) -> bool {
    let mut last_last_char = '\0';
    let mut last_char = '\0';

    let mut repeated_in_between_letter_found = false;
    let mut double_pair_found = false;

    let mut set = HashSet::new();

    for (i, c) in str.chars().enumerate() {
        let pair = Pair(last_char, c, i, i + 1);

        if set.contains(&pair) {
            let Pair(_, _, _, idx) = set.get(&pair).unwrap();
            if *idx != pair.2 {
                double_pair_found = true;
            }
        } else {
            set.insert(pair);
        }

        if last_last_char == c {
            repeated_in_between_letter_found = true;
        }

        last_last_char = last_char;
        last_char = c;
    }

    repeated_in_between_letter_found && double_pair_found
}
