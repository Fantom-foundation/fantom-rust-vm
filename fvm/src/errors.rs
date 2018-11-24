use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, VMError>;

#[derive(Debug, Clone)]
pub enum VMError {
    UnknownOpcodeError,
    MemoryError
}

impl Error for VMError {}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VMError::UnknownOpcodeError => {
                write!(f, "an unrecognized opcode was found")
            },
            VMError::MemoryError => {
                write!(f, "out of memory")
            },
            _ => {
                write!(f, "unknown error occurred")
            },
        }
        
    }
}