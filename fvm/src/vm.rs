//! Module that contains the VM that executes bytecode

use ethereum_types::U256;
use errors::*;
use memory::Memory;

pub struct VM {
    registers: [U256; 32],
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0.into(); 32]
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