//! # utils
//! Holds utility functions in order to help validator service.

pub fn assertString(input: T) {
    match input {
        String => Ok(input),
        &str => Ok(input.to_string()),
        _ => panic!("Expected a String but received {}", input)
    };
}