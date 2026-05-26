pub fn solve(input: &String) {
    let mut floor = 0;

    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
    }

    println!("Part 1: {floor}");
}
