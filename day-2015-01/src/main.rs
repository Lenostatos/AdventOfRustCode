use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("input.txt")?;

    let mut floor = 0;

    for char in input.chars() {
        match char {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("Input should only contain ( and ) characters."),
        }
    }

    println!("final floor: {floor}");

    let mut current_floor = 0;
    let mut entry_position = 0;

    for (i, char) in input.chars().enumerate() {
        match char {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => panic!("Input should only contain ( and ) characters."),
        }

        if current_floor == -1 {
            entry_position = i + 1;
            break;
        };
    }

    println!("entry position: {entry_position}");

    Ok(())
}
