//! This module contains errors related to the Fantom VM itself
use std::error::Error;
use std::fmt;

/// Convenience wrapper around T and a VMError
pub type Result<T> = std::result::Result<T, VMError>;

#[derive(Debug, Clone)]
/// Errors related to the VM
pub enum VMError {
    // VM has encountered an unknown opcode
    UnknownOpcodeError,
    // VM has run out of memory
    MemoryError,
}

impl Error for VMError {}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VMError::UnknownOpcodeError => write!(f, "an unrecognized opcode was found"),
            VMError::MemoryError => write!(f, "out of memory"),
            _ => write!(f, "unknown error occurred"),
        }
    }
}

#[derive(Debug, Clone)]
/// Errors related to Storage
pub enum StorageError {
    CommitError,
    RequireError,
    InvalidCommitment,
    AlreadyCommitted,
}

impl Error for StorageError {}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageError::RequireError => write!(f, "require area"),
            StorageError::CommitError => write!(f, "commit area"),
            _ => write!(f, "unknown storage error occurred"),
        }
    }
}
