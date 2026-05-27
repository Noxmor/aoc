use std::io::{self, Read};

mod part1;
mod part2;

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    part1::solve(&input);
    part2::solve(&input);
}
