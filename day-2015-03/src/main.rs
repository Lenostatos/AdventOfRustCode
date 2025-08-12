use std::error::Error;
use std::fmt::Display;
use std::{char, fs};

fn main() -> Result<(), Box<dyn Error>> {
    // One Santa only
    let input: String = fs::read_to_string("input.txt")?;

    let mut position = Position::new();
    let mut position_record = vec![position.to_string()];

    for char in input.chars() {
        position.move_it(char);
        position_record.push(position.to_string());
    }

    position_record.sort_unstable();
    position_record.dedup();

    let num_unique_positions = position_record.len();

    println!("{num_unique_positions}");

    // Two Santas taking turns
    let mut position_1 = Position::new();
    let mut position_2 = Position::new();

    let mut position_record = vec![position_1.to_string()];

    let mut turn_1 = true;

    for char in input.chars() {
        if turn_1 {
            position_1.move_it(char);
            position_record.push(position_1.to_string());
            turn_1 = !turn_1;
        } else {
            position_2.move_it(char);
            position_record.push(position_2.to_string());
            turn_1 = !turn_1;
        }
    }

    position_record.sort_unstable();
    position_record.dedup();

    let num_unique_positions = position_record.len();

    println!("{num_unique_positions}");

    Ok(())
}

struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    fn move_it(&mut self, direction: char) {
        match direction {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _ => panic!("Moving directions should only be given via one of the four valid chars."),
        };
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}|{}", self.x, self.y)
    }
}
