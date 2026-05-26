pub fn solve(input: &String) {
    let mut floor = 0;
    let mut index = 0;

    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }

        index += 1;

        if floor == -1 {
            break;
        }
    }

    println!("Part 2: {index}");
}
