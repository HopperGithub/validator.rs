//! # validate
//! This module includes necessary structs and functions to validate string.

/// validate record is boolean
#[allow(dead_code)]
pub fn is_boolean(record: &str) -> bool {
    match &*record.to_lowercase() {
        "true" => true,
        "false" => true,
        "1" => true,
        "0" => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_boolean_test() {
        assert_eq!(is_boolean("True"), true);
        assert_eq!(is_boolean("FalSe"), true);
        assert_eq!(is_boolean("1"), true);
        assert_eq!(is_boolean("0"), true);
        assert_eq!(is_boolean("01"), false);
    }
}