use crate::model::ConstantIndex;
use std::io;

#[derive(Debug)]
pub enum JvmParseError {
    Io(io::Error),
    InvalidFormat(String),
    MissingConstant(ConstantIndex),
    WrongConstantType(ConstantIndex, String),
}

pub type JvmParseResult<T> = Result<T, JvmParseError>;

impl From<io::Error> for JvmParseError {
    fn from(err: io::Error) -> Self {
        JvmParseError::Io(err)
    }
}
