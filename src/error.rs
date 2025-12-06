use std::num::ParseIntError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to parse {0} as a range")]
    ParseRangeError(String),
    #[error("Failed to parse as int: {0}")]
    ParseIntError(ParseIntError),
}
