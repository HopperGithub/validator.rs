//! # validator
//! This module includes necessary structs and functions to validate string.
use std::cmp::PartialEq;
use crate::validate;

/// A simple struct to hold necessary information about a validator.
#[derive(Debug,Eq)]
pub struct Validator {
    /// key for checked.
    pub key: String,
    /// array of unpassed message.
    pub message: Vec<String>,
}

impl Validator {
    /// Creates new instance for the validator
    #[allow(dead_code)]
    pub fn new<S: Into<String>>(key: S) -> Validator {
        Validator {
            key: key.into(),
            message: Vec::<String>::new()
        }
    }
}

impl PartialEq for Validator {
    #[inline]
    fn eq(&self, other: &Validator) -> bool {
        self.key == other.key
    }
}

impl Validator {
    #[inline]
    pub fn eq_string(&self, other: &str) -> bool {
        self.key == other
    }

    pub fn is_boolean(&mut self) -> &mut Self {
        if !validate::boolean::is_boolean(&*self.key) {
            self.message.push(format!("{} is not boolean", self.key))
        }

        self
    }

    pub fn is_numeric(&mut self) -> &mut Self {
        if !validate::numeric::is_numeric(&*self.key) {
            self.message.push(format!("{} is not numeric", self.key))
        }
        self
    }

    pub fn is_ok(&self) -> bool {
        self.message.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validator_test() {
        let v1 = Validator::new("TestKey1");
        let v2 = Validator::new("TestKey2");

        assert_eq!(v1.eq_string("TestKey"), false);
        assert_eq!(v1.eq(&v2), false);
    }

    #[test]
    fn validator_is_test() {
        let mut v3 = Validator::new("TrUe");
        let mut v4 = Validator::new("10");

        assert_eq!(v3.is_boolean().is_ok(), true);
        assert_eq!(v4.is_numeric().is_ok(), true);
    }

    #[test]
    fn validator_chain_test() {
        let mut v5 = Validator::new("1");
        assert_eq!(v5.is_boolean().is_numeric().is_ok(), true);
    }
}