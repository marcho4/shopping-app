use std::error::Error as ErrTrait;

#[derive(Debug)]
pub struct Error(pub String, pub u16);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ErrTrait for Error {}