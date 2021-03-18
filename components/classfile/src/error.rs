use crate::model::ConstantIndex;
use rustjvm_opcode::DisasmError;
use std::io;

#[derive(Debug)]
pub enum JvmParseError {
    Io(io::Error),
    InvalidFormat(String),
    MissingConstant(ConstantIndex),
    WrongConstantType(ConstantIndex, String),
    InvalidCode(DisasmError),
}

pub type JvmParseResult<T> = Result<T, JvmParseError>;

impl From<io::Error> for JvmParseError {
    fn from(err: io::Error) -> Self {
        JvmParseError::Io(err)
    }
}

impl From<DisasmError> for JvmParseError {
    fn from(err: DisasmError) -> Self {
        JvmParseError::InvalidCode(err)
    }
}
