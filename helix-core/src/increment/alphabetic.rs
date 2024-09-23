fn validate_selected_text(selected_text: &str) -> Option<(char, char)> {
    let mut selected_chars = selected_text.chars();
    let start = selected_chars.next()?;
    let letter = selected_chars.next()?;
    let end = selected_chars.next()?;

    // Ensure selected_chars is only 3 characters long
    if !selected_chars.next().is_none() {
        return None;
    }

    // Ensure start and end chars are the same and are single or double quotes
    if start != end || !(start == '\'' || start == '"') {
        return None;
    }

    if !letter.is_ascii_alphabetic() {
        return None;
    }

    Some((start, letter))
}

fn increment_char(letter: char, amount: i64) -> char {
    let alpha = if letter.is_ascii_uppercase() {
        b'A'
    } else {
        b'a'
    };
    let char_index = letter as u8 - alpha;
    let updated_char_index = ((char_index as i64 + amount).rem_euclid(26)) as u8;
    (alpha + updated_char_index) as char
}

pub fn increment(selected_text: &str, amount: i64) -> Option<String> {
    let (quote_char, original_letter) = validate_selected_text(selected_text)?;
    let updated_letter = increment_char(original_letter, amount);

    Some(format!("{quote_char}{updated_letter}{quote_char}"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lowercase_increment() {
        let tests = [
            ("'a'", 1, "'b'"),
            ("'a'", 25, "'z'"),
            ("'a'", 26, "'a'"),
            ("'y'", 1, "'z'"),
            ("'z'", 1, "'a'"),
        ];
        for (original, amount, expected) in tests {
            assert_eq!(increment(original, amount).unwrap(), expected);
        }
    }
}
