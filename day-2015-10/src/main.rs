use std::char;

fn main() {
    let input = "3113322113";

    let mut look_and_say = input.to_string();

    for _ in 0..50 {
        let digit_counts = DigitCount::parse(&look_and_say);

        look_and_say.clear();

        for digit_count in digit_counts {
            look_and_say.push_str(&digit_count.count.to_string());
            look_and_say.push_str(&digit_count.digit.to_string());
        }
    }

    println!("{}", look_and_say.chars().count());
}

#[derive(PartialEq, Debug)]
struct DigitCount {
    digit: char,
    count: usize,
}

impl DigitCount {
    fn parse(str: &str) -> Vec<DigitCount> {
        let mut digit_counts = vec![];

        let mut current_char = ' ';
        let mut current_char_count = 0;

        for new_char in str.chars() {
            if current_char == ' ' {
                current_char = new_char;
            }

            if new_char != current_char {
                digit_counts.push(DigitCount {
                    digit: current_char,
                    count: current_char_count,
                });
                current_char = new_char;
                current_char_count = 0;
            }

            current_char_count += 1;
        }

        digit_counts.push(DigitCount {
            digit: current_char,
            count: current_char_count,
        });

        digit_counts
    }
}

#[test]
fn parse_digit_counts() {
    assert_eq!(
        DigitCount::parse("1"),
        vec![DigitCount {
            digit: '1',
            count: 1
        }]
    );
    assert_eq!(
        DigitCount::parse("11"),
        vec![DigitCount {
            digit: '1',
            count: 2
        }]
    );
    assert_eq!(
        DigitCount::parse("111"),
        vec![DigitCount {
            digit: '1',
            count: 3
        }]
    );
    assert_eq!(
        DigitCount::parse("12"),
        vec![
            DigitCount {
                digit: '1',
                count: 1
            },
            DigitCount {
                digit: '2',
                count: 1
            }
        ]
    );
    assert_eq!(
        DigitCount::parse("122"),
        vec![
            DigitCount {
                digit: '1',
                count: 1
            },
            DigitCount {
                digit: '2',
                count: 2
            }
        ]
    );
    assert_eq!(
        DigitCount::parse("1122"),
        vec![
            DigitCount {
                digit: '1',
                count: 2
            },
            DigitCount {
                digit: '2',
                count: 2
            }
        ]
    );
    assert_eq!(
        DigitCount::parse("112333"),
        vec![
            DigitCount {
                digit: '1',
                count: 2
            },
            DigitCount {
                digit: '2',
                count: 1
            },
            DigitCount {
                digit: '3',
                count: 3
            }
        ]
    );
}
