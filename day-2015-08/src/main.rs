use util::input::lines_of;

fn main() {
    let mut excess_code_char_counts = Vec::new();

    for line in lines_of("input.txt") {
        let line_len = line.len();
        let mut char_iter = line.chars().enumerate();

        // skip first character
        char_iter.next();

        let mut num_code_chars: usize = 2;
        let mut num_memory_chars: usize = 0;

        let mut previous_char;
        let mut current_char = char_iter.next().unwrap();

        loop {
            previous_char = current_char;

            // exit before last code char
            if current_char.0 == line_len - 1 {
                break;
            }

            current_char = char_iter.next().unwrap();

            match previous_char.1 {
                '\\' => match current_char.1 {
                    'x' => {
                        char_iter.next();
                        char_iter.next();
                        current_char = char_iter.next().unwrap();
                        num_code_chars += 3;
                    }
                    _ => {
                        current_char = char_iter.next().unwrap();
                        num_code_chars += 1;
                    }
                },
                _ => {
                    num_code_chars += 1;
                    num_memory_chars += 1;
                }
            }
        }

        let num_excess_code_chars = num_code_chars - num_memory_chars;

        println!("{line}: {num_code_chars} - {num_memory_chars} = {num_excess_code_chars}");

        excess_code_char_counts.push(num_excess_code_chars);
    }

    let num_excess_code_chars: usize = excess_code_char_counts.iter().sum();

    println!("{num_excess_code_chars}");

    // Part 2

    let mut excess_encoded_char_counts = Vec::new();

    for line in lines_of("input.txt") {
        let num_code_chars: usize = line.len();
        let num_encoded_chars: usize =
            line.len() + 2 + line.chars().filter(|c| *c == '\"' || *c == '\\').count();

        let num_excess_encoded_chars = num_encoded_chars - num_code_chars;

        println!("{line}: {num_encoded_chars} - {num_code_chars} = {num_excess_encoded_chars}");

        excess_encoded_char_counts.push(num_excess_encoded_chars);
    }

    let num_excess_encoded_chars: usize = excess_encoded_char_counts.iter().sum();

    println!("{num_excess_encoded_chars}");
}
