pub fn solve(input: &String) {
    let mut result = 0;

    for line in input.lines() {
        let mut dimensions: Vec<i32> = line.split('x').map(|d| d.parse().unwrap()).collect();

        dimensions.sort();

        result += 2 * dimensions[0] + 2 * dimensions[1];
        result += dimensions[0] * dimensions[1] * dimensions[2];
    }

    println!("Part 2: {result}");
}
