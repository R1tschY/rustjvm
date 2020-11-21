use std::io;

#[derive(Debug)]
pub enum JvmParseError {
    Io(io::Error),
    InvalidFormat(String),
}

pub type JvmParseResult<T> = Result<T, JvmParseError>;

impl From<io::Error> for JvmParseError {
    fn from(err: io::Error) -> Self {
        JvmParseError::Io(err)
    }
}
