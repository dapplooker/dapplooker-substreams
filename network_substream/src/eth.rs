use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct DecodeError {
    pub msg: String,
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid decoding")
    }
}

impl Error for DecodeError {}

pub fn read_string_from_bytes(input: &[u8]) -> String {
    // we have to check if we have a valid utf8 representation and if we do
    // we return the value if not we return a DecodeError
    if let Some(last) = input.to_vec().iter().rev().position(|&pos| pos != 0) {
        return String::from_utf8_lossy(&input[0..input.len() - last]).to_string();
    }

    // use case when all the bytes are set to 0
    "".to_string()
}
