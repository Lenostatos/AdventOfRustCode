use std::collections::HashSet;

fn main() {
    let mut input = "hxbxwxba".to_string();

    while !is_valid_pw(&input) {
        increment(&mut input);
    }

    println!("{input}");

    increment(&mut input);

    while !is_valid_pw(&input) {
        increment(&mut input);
    }

    println!("{input}");
}

fn increment(str: &mut String) {
    let mut new_str_end = "".to_string();
    let mut new_str_end_char_index = 0;

    for (i, char) in str.char_indices().rev() {
        match char {
            'z' => {
                new_str_end.push('a');
                continue;
            }
            'h' => new_str_end.push('j'),
            'n' => new_str_end.push('p'),
            'k' => new_str_end.push('m'),
            _ => new_str_end.push(char::from_u32(char as u32 + 1).unwrap()),
        }

        new_str_end_char_index = i;
        break;
    }

    new_str_end = new_str_end.chars().rev().collect::<String>();

    str.replace_range(new_str_end_char_index.., &new_str_end);
}

#[test]
fn increment_str() {
    let mut str = "".to_string();

    increment(&mut str);
    assert_eq!(str, "".to_string());

    str = "a".to_string();

    increment(&mut str);
    assert_eq!(str, "b".to_string());
    increment(&mut str);
    assert_eq!(str, "c".to_string());
    increment(&mut str);
    assert_eq!(str, "d".to_string());
    increment(&mut str);
    assert_eq!(str, "e".to_string());

    str = "abc".to_string();

    increment(&mut str);
    assert_eq!(str, "abd".to_string());
    increment(&mut str);
    assert_eq!(str, "abe".to_string());

    str = "z".to_string();

    increment(&mut str);
    assert_eq!(str, "a".to_string());
    increment(&mut str);
    assert_eq!(str, "b".to_string());

    str = "wy".to_string();

    increment(&mut str);
    assert_eq!(str, "wz".to_string());
    increment(&mut str);
    assert_eq!(str, "xa".to_string());
    increment(&mut str);
    assert_eq!(str, "xb".to_string());

    str = "zzz".to_string();

    increment(&mut str);
    assert_eq!(str, "aaa".to_string());
    increment(&mut str);
    assert_eq!(str, "aab".to_string());
}

fn is_valid_pw(pw: &str) -> bool {
    let pw_as_u32 = pw.chars().map(|c| c as u32).collect::<Vec<_>>();

    let has_increasing_straight_of_3 = pw_as_u32
        .windows(3)
        .any(|w| w[0] == w[1] - 1 && w[1] == w[2] - 1);

    if !has_increasing_straight_of_3 {
        return false;
    }

    let contains_ambiguous_char = pw.contains(|char| char == 'i' || char == 'o' || char == 'l');

    if contains_ambiguous_char {
        return false;
    }

    let mut pair_letters = HashSet::new();

    for w in pw_as_u32.windows(2) {
        if w[0] == w[1] {
            pair_letters.insert(w[0]);
        }
    }

    let has_different_letter_pairs = pair_letters.len() >= 2;

    has_different_letter_pairs
}

#[test]
fn password_validation() {
    let mut str = "".to_string();

    assert!(!is_valid_pw(&str));

    str = "abc".to_string();
    assert!(!is_valid_pw(&str));

    str = "abcc".to_string();
    assert!(!is_valid_pw(&str));

    str = "abccddi".to_string();
    assert!(!is_valid_pw(&str));

    str = "hijklmmn".to_string();
    assert!(!is_valid_pw(&str));

    str = "abbceffg".to_string();
    assert!(!is_valid_pw(&str));

    str = "abbcegjk".to_string();
    assert!(!is_valid_pw(&str));

    str = "abccdd".to_string();
    assert!(is_valid_pw(&str));
}
