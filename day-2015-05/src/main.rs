use std::char;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use util::window_iter::WindowStrIntoIter;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let lines_iterator = f.lines().map(|l| l.unwrap());

    let num_nice_lines = lines_iterator.filter(|l| is_nice(l)).count();

    println!("{num_nice_lines}");

    let f = File::open("input.txt")?;
    let f = BufReader::new(f);

    let lines_iterator = f.lines().map(|l| l.unwrap());

    let num_nice_lines = lines_iterator.filter(|line| is_nicer(line)).count();

    println!("{num_nice_lines}");

    Ok(())
}

fn is_a_vowel(char: char) -> bool {
    match char {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn is_nice(str: &str) -> bool {
    let mut num_vowels = 0;

    let mut prev_char = None;
    let mut has_double_char = false;

    let mut has_naughty_double = true;

    for char in str.chars() {
        if is_a_vowel(char) {
            num_vowels += 1;
        }

        has_double_char = has_double_char || prev_char.map_or(false, |prev_char| prev_char == char);

        if prev_char.is_some() {
            has_naughty_double = match [prev_char.unwrap(), char] {
                ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'] => true,
                _ => false,
            };
            if has_naughty_double {
                break;
            }
        }

        prev_char = Some(char);
    }

    num_vowels >= 3 && has_double_char && !has_naughty_double
}

fn is_nicer(str: &str) -> bool {
    let letter_pairs = collect_letter_pairs(&str);

    let double_pair = letter_pairs.iter().find(|(_pair, positions)| {
        positions.len() >= 2
            && positions.iter().max().unwrap() - positions.iter().min().unwrap() > 1
    });

    let windowed_str = WindowStrIntoIter::new(str, 3);

    let nxn_pattern = windowed_str
        .into_iter()
        .find(|str| str.chars().nth(0).unwrap() == str.chars().nth(2).unwrap());

    double_pair.is_some() && nxn_pattern.is_some()
}

#[test]
fn is_correctly_nicer() {
    assert!(is_nicer("qjhvhtzxzqqjkmpb"));
    assert!(is_nicer("xxyxx"));
    assert!(!is_nicer("uurcxstgmygtbstg"));
    assert!(!is_nicer("ieodomkazucvgmuy"));
}

fn collect_letter_pairs(str: &str) -> HashMap<(char, char), Vec<usize>> {
    let mut pairs = HashMap::<(char, char), Vec<usize>>::new();

    let windowed_str = WindowStrIntoIter::new(str, 2);

    for (position, window) in windowed_str.into_iter().enumerate() {
        let pair = (
            window.chars().nth(0).unwrap(),
            window.chars().nth(1).unwrap(),
        );

        pairs
            .entry(pair)
            .and_modify(|positions| positions.push(position))
            .or_insert(vec![position]);
    }

    pairs
}
