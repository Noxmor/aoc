use std::io::{self, Read};

mod part1;

fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    part1::solve(&input);
}
