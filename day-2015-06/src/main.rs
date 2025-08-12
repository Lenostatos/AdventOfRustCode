use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::usize;

use util::grid::{Grid, GridPosition, GridSection};

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let lines_iterator = f.lines().map(|l| l.unwrap());

    let mut lights = Grid::new(1000, 1000, false);

    for line in lines_iterator {
        let instruction = Instruction::parse(&line);

        match instruction.command {
            InstructionKind::On => lights.set_section(
                true,
                &GridSection {
                    from: instruction.from,
                    to: instruction.to,
                },
            ),
            InstructionKind::Off => lights.set_section(
                false,
                &GridSection {
                    from: instruction.from,
                    to: instruction.to,
                },
            ),
            InstructionKind::Toggle => lights.mut_section(
                &GridSection {
                    from: instruction.from,
                    to: instruction.to,
                },
                |l| *l = !*l,
            ),
        }
    }

    let num_lights_on = count_lights(&lights, None);

    println!("{num_lights_on}");

    // Part 2

    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let lines_iterator = f.lines().map(|l| l.unwrap());

    let mut lights = Grid::<usize>::new(1000, 1000, 0);

    for line in lines_iterator {
        let instruction = Instruction::parse(&line);

        match instruction.command {
            InstructionKind::On => lights.mut_section(
                &GridSection {
                    from: instruction.from,
                    to: instruction.to,
                },
                |l| *l += 1,
            ),
            InstructionKind::Off => lights.mut_section(
                &GridSection {
                    from: instruction.from,
                    to: instruction.to,
                },
                |l| {
                    if *l > 0 {
                        *l -= 1;
                    }
                },
            ),
            InstructionKind::Toggle => lights.mut_section(
                &GridSection {
                    from: instruction.from,
                    to: instruction.to,
                },
                |l| *l += 2,
            ),
        }
    }

    let sum_brightness = lights.get_all().iter().sum::<usize>();

    println!("{sum_brightness}");

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Instruction {
    command: InstructionKind,
    from: GridPosition,
    to: GridPosition,
}

#[derive(Debug, PartialEq)]
enum InstructionKind {
    On,
    Off,
    Toggle,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let instruction_kind = if input.starts_with("toggle") {
            InstructionKind::Toggle
        } else if input.starts_with("turn on") {
            InstructionKind::On
        } else if input.starts_with("turn off") {
            InstructionKind::Off
        } else {
            panic!("Input should start with one of the three instruction kinds.");
        };

        let position_input = match instruction_kind {
            InstructionKind::Toggle => input.strip_prefix("toggle "),
            InstructionKind::On => input.strip_prefix("turn on "),
            InstructionKind::Off => input.strip_prefix("turn off "),
        }
        .unwrap();

        let mut positions = position_input
            .split(" through ")
            .map(|position_str| Self::parse_grid_position(position_str));

        Self {
            command: instruction_kind,
            from: positions.next().unwrap(),
            to: positions.next().unwrap(),
        }
    }

    fn parse_grid_position(input: &str) -> GridPosition {
        let mut values = input
            .split(',')
            .map(|value_str| value_str.parse::<usize>().unwrap());

        GridPosition(values.next().unwrap(), values.next().unwrap())
    }
}

#[test]
fn instruction_parsing() {
    assert_eq!(
        Instruction::parse("turn off 231,492 through 790,976"),
        Instruction {
            command: InstructionKind::Off,
            from: GridPosition(231, 492),
            to: GridPosition(790, 976)
        }
    );

    assert_eq!(
        Instruction::parse("turn on 874,57 through 93,684"),
        Instruction {
            command: InstructionKind::On,
            from: GridPosition(874, 57),
            to: GridPosition(93, 684)
        }
    );

    assert_eq!(
        Instruction::parse("toggle 911,8 through 990,2"),
        Instruction {
            command: InstructionKind::Toggle,
            from: GridPosition(911, 8),
            to: GridPosition(990, 2)
        }
    );
}

fn count_lights(lights: &Grid<bool>, from_to: Option<&GridSection>) -> usize {
    if let Some(from_to) = from_to {
        lights.get_section(from_to).iter().filter(|l| ***l).count()
    } else {
        lights.get_all().iter().filter(|l| **l).count()
    }
}

#[test]
fn test_count_lights() {
    let mut lights = Grid::new(3, 3, false);
    assert_eq!(count_lights(&lights, None), 0);

    lights.set(true, &GridPosition(0, 0));
    assert_eq!(count_lights(&lights, None), 1);

    lights.set_section(
        true,
        &GridSection {
            from: GridPosition(0, 0),
            to: GridPosition(1, 1),
        },
    );
    assert_eq!(count_lights(&lights, None), 4);
}
