pub fn solve(input: &String) {
    let mut count = 0;

    for str in input.lines() {
        if is_nice(str) {
            count += 1;
        }
    }

    println!("Part 1: {count}");
}

fn is_nice(str: &str) -> bool {
    let mut vowel_count = 0;
    let mut double_letters_count = 0;
    let mut last_char = '\0';

    for c in str.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,

            _ => {}
        }

        if last_char == c {
            double_letters_count += 1;
        }

        if last_char == 'a' && c == 'b' || last_char == 'c' && c == 'd'
        || last_char == 'p' && c == 'q' || last_char == 'x' && c == 'y' {
            return false;
        }

        last_char = c;
    }

    vowel_count >= 3 && double_letters_count > 0
}
