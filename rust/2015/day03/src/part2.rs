use std::collections::HashMap;

pub fn solve(input: &String) {
    let mut map = HashMap::new();

    let mut idx = 0;
    let mut positions = [(0, 0), (0, 0)];


    map.insert(positions[idx], 2);

    for c in input.chars() {
        let pos = &mut positions[idx];
        match c {
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            '<' => pos.0 -= 1,
            '>' => pos.0 += 1,
            _ => {}
        }

        let value = if map.contains_key(pos) {
            map.get(pos).unwrap() + 1
        } else {
            1
        };

        map.insert(*pos, value);

        idx = (idx + 1) % 2;
    }

    println!("Part 2: {}", map.len());
}
