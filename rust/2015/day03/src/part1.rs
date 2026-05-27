use std::collections::HashMap;

pub fn solve(input: &String) {
    let mut map = HashMap::new();

    let mut pos = (0, 0);

    map.insert(pos, 1);

    for c in input.chars() {
        match c {
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            '<' => pos.0 -= 1,
            '>' => pos.0 += 1,
            _ => {}
        }

        let value = if map.contains_key(&pos) {
            map.get(&pos).unwrap() + 1
        } else {
            1
        };

        map.insert(pos, value);
    }

    println!("Part 1: {}", map.len());
}
