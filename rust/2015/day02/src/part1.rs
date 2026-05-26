pub fn solve(input: &String) {
    let mut result = 0;

    for line in input.lines() {
        let mut dimensions: Vec<i32> = line.split('x').map(|d| d.parse().unwrap()).collect();

        let (l, w, h) = (dimensions[0], dimensions[1], dimensions[2]);
        dimensions.sort();

        let surface_area = 2 * l * w + 2 * w * h + 2 * h * l;
        result += surface_area + dimensions[0] * dimensions[1];
    }

    println!("{result}");
}
