pub fn char_to_col(ch: char) -> Option<usize> {
    match ch {
        'a'..='z' => Some(ch as usize - 'a' as usize),
        'A'..='Z' => Some(ch as usize - 'A' as usize),
        _ => None,
    }
}

pub fn col_to_char(col: usize) -> Option<char> {
    match col {
        0..26 => Some((b'A' + col as u8) as char),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_col_lowercase_conversions() {
        for (i, ch) in ('a'..='z').enumerate() {
            assert_eq!(char_to_col(ch), Some(i));
        }
    }

    #[test]
    fn test_char_to_col_uppercase_conversions() {
        for (i, ch) in ('A'..='Z').enumerate() {
            assert_eq!(char_to_col(ch), Some(i));
        }
    }

    #[test]
    fn test_col_char_conversions_are_reversible() {
        for ch in 'A'..='Z' {
            assert_eq!(char_to_col(ch).map(col_to_char), Some(Some(ch)));
        }
    }
}
