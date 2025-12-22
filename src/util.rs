use crate::Col;

pub fn char_to_col(ch: char) -> Option<Col> {
    match ch {
        'a'..='z' => Some(ch as usize - 'a' as usize),
        'A'..='Z' => Some(ch as usize - 'A' as usize),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_col_lowercase_conversions() {
        for (i, ch) in ('a'..='z').enumerate() {
            assert_eq!(char_to_col(ch), Some(i));
        }
    }

    #[test]
    fn test_col_uppercase_conversions() {
        for (i, ch) in ('A'..='Z').enumerate() {
            assert_eq!(char_to_col(ch), Some(i));
        }
    }
}
