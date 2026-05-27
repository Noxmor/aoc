use md5::{Md5, Digest};

pub fn solve(input: &String) {
    let mut number = 0;

    loop {
        let mut hasher = Md5::new();
        hasher.update(input.trim());
        hasher.update(number.to_string());
        let hash = hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect::<String>();

        let mut found_solution = true;
        for c in hash.chars().take(6) {
            if c != '0' {
                found_solution = false;
            }
        }

        if found_solution {
            break;
        }

        number += 1;
    }

    println!("Part 2: {number}");
}
