//! Module that contains the VM that executes bytecode

use bigint::U256;
use errors::*;
use memory::{SimpleMemory, Memory};

pub struct VM {
    registers: [U256; 32],
    memory: Option<Box<Memory>>
}

impl VM {
    /// Creates and returns a new VM
    pub fn new() -> VM {
        VM {
            registers: [0.into(); 32],
            memory: None
        }
    }

    /// Sets the volatile memory of the VM to the SimpleMemory type
    pub fn with_simple_memory(mut self) -> VM {
        self.memory = Some(Box::new(SimpleMemory::new()));
        self
    }
}

impl Default for VM {
    fn default() -> VM {
        VM {
            registers: [0.into(); 32],
            memory: Some(Box::new(SimpleMemory::new()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let vm = VM::new();
        assert_eq!(vm.registers.len(), 32);
    }
}