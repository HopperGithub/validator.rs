//! # validate
//! This module includes necessary structs and functions to validate string.
use regex::Regex;
use lazy_static::lazy_static;

/// validate record is numeric
#[allow(dead_code)]
pub fn is_numeric(record: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]+$").unwrap();
    }
    
    RE.is_match(record)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_numeric_test() {
        for i in 0..10 {
            assert_eq!(is_numeric(&*i.to_string()), true);
        }
    }
}