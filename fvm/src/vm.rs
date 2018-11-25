//! Module that contains the VM that executes bytecode

use bigint::{M256, MI256, U256};
use errors::{Result, VMError};
use memory::{Memory, SimpleMemory};
use opcodes::Opcode;

pub struct VM {
    registers: [U256; 1024],
    memory: Option<Box<Memory>>,
    code: Vec<u8>,
    pc: usize,
    stack_pointer: usize,
}

impl VM {
    /// Creates and returns a new VM
    pub fn new(code: Vec<u8>) -> VM {
        VM {
            registers: [0.into(); 1024],
            memory: None,
            stack_pointer: 0,
            code: code,
            pc: 0,
        }
    }

    /// Sets the volatile memory of the VM to the SimpleMemory type
    pub fn with_simple_memory(mut self) -> VM {
        self.memory = Some(Box::new(SimpleMemory::new()));
        self
    }

    pub fn execute(&mut self) -> Result<()> {
        loop {
            match self.execute_one() {
                Ok(_) => {
                    continue;
                }
                Err(e) => {
                    return Err(e);
                }
            };
        }
    }
    pub fn execute_one(&mut self) -> Result<()> {
        let opcode = Opcode::from(&self.code[self.pc]);
        match opcode {
            Opcode::STOP => {
                return Ok(());
            }
            Opcode::ADD => {
                self.stack_pointer -= 1;
                let result = self.registers[self.stack_pointer] + self.registers[self.stack_pointer - 1];
                self.registers[self.stack_pointer - 1] = result;
                self.pc += 1;
            }
            Opcode::MUL => {
                self.stack_pointer -= 1;
                let result = self.registers[self.stack_pointer] * self.registers[self.stack_pointer - 1];
                self.registers[self.stack_pointer - 1] = result;
                self.pc += 1;
            }
            Opcode::SUB => {
                self.stack_pointer -= 1;
                let result = self.registers[self.stack_pointer] - self.registers[self.stack_pointer - 1];
                self.registers[self.stack_pointer - 1] = result;
                self.pc += 1;
            }
            Opcode::DIV => {
                self.stack_pointer -= 1;
                let result = self.registers[self.stack_pointer] / self.registers[self.stack_pointer - 1];
                self.registers[self.stack_pointer - 1] = result;
                self.pc += 1;
            }
            Opcode::SDIV => unimplemented!(),
            Opcode::SMOD => unimplemented!(),
            Opcode::MOD => {
                self.stack_pointer -= 1;
                let result = self.registers[self.stack_pointer] % self.registers[self.stack_pointer - 1];
                self.registers[self.stack_pointer - 1] = result;
                self.pc += 1;
            }
            Opcode::LT => {
                self.stack_pointer -= 1;
                if self.registers[self.stack_pointer] > self.registers[self.stack_pointer - 1] {
                    self.registers[self.stack_pointer - 1] = 1.into();
                } else {
                    self.registers[self.stack_pointer - 1] = 0.into();
                }
                self.pc += 2;
            }
            Opcode::GT => {
                self.stack_pointer -= 1;
                if self.registers[self.stack_pointer] < self.registers[self.stack_pointer - 1] {
                    self.registers[self.stack_pointer - 1] = 1.into();
                } else {
                    self.registers[self.stack_pointer - 1] = 0.into();
                }
                self.pc += 2;
            }
            Opcode::SLT => unimplemented!(),
            Opcode::SGT => unimplemented!(),
            Opcode::EQ => unimplemented!(),
            Opcode::ISZERO => unimplemented!(),
            Opcode::AND => unimplemented!(),
            Opcode::OR => unimplemented!(),
            Opcode::XOR => unimplemented!(),
            Opcode::NOT => unimplemented!(),
            Opcode::BYTE => unimplemented!(),
            Opcode::SLOAD => unimplemented!(),
            Opcode::STORE => unimplemented!(),
            Opcode::MLOAD => unimplemented!(),
            Opcode::MSTORE => unimplemented!(),
            Opcode::MSTORE8 => unimplemented!(),
            Opcode::MSIZE => unimplemented!(),
            Opcode::PUSH1 => {
                self.registers[self.stack_pointer] = self.code[self.pc + 1].into();
                self.stack_pointer += 1;
                self.pc += 2;
            }
            _ => {
                return Err(VMError::UnknownOpcodeError);
            }
        };

        Ok(())
    }

    fn extract_opcode(&self, byte: &u8) -> Opcode {
        Opcode::from(byte)
    }

    pub fn print_registers(&self, start: usize, end: usize) {
        println!("Registers are: ");
        for register in self.registers[start..end].iter() {
            print!("{} ", register);
        }
        println!("\nEnd of Registers");
    }
}

impl Default for VM {
    fn default() -> VM {
        VM {
            registers: [0.into(); 1024],
            memory: Some(Box::new(SimpleMemory::new())),
            stack_pointer: 0,
            code: vec![],
            pc: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let default_code = vec![0];
        let vm = VM::new(default_code);
        assert_eq!(vm.registers.len(), 1024);
    }

    #[test]
    fn test_stop_opcode() {
        let default_code = vec![0];
        let mut vm = VM::new(default_code);
        assert!(vm.execute_one().is_ok())
    }

    #[test]
    fn test_push_opcode() {
        let default_code = vec![0x60, 0xa];
        let mut vm = VM::new(default_code);
        let result = vm.execute_one();
        assert!(result.is_ok());
        assert_eq!(vm.registers[0], 10.into());
    }

    #[test]
    fn test_add_opcode() {
        let default_code = vec![0x60, 0xa, 0x60, 0xa, 0x01];
        let mut vm = VM::new(default_code);
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        assert_eq!(vm.registers[0], 20.into());
    }

    #[test]
    fn test_sub_opcode() {
        let default_code = vec![0x60, 0xa, 0x60, 0xa, 0x03];
        let mut vm = VM::new(default_code);
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        assert_eq!(vm.registers[0], 0.into());
    }

    #[test]
    fn test_mul_opcode() {
        let default_code = vec![0x60, 0xa, 0x60, 0xa, 0x02];
        let mut vm = VM::new(default_code);
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        assert_eq!(vm.registers[0], 100.into());
    }

    #[test]
    fn test_div_opcode() {
        let default_code = vec![0x60, 0xa, 0x60, 0xa, 0x04];
        let mut vm = VM::new(default_code);
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        assert_eq!(vm.registers[0], 1.into());
    }

    #[test]
    fn test_mod_opcode() {
        let default_code = vec![0x60, 0xa, 0x60, 0xb, 0x06];
        let mut vm = VM::new(default_code);
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        assert_eq!(vm.registers[0], 1.into());
    }

    #[test]
    fn test_lt_opcode() {
        let default_code = vec![0x60, 0xa, 0x60, 0xb, 0x0a];
        let mut vm = VM::new(default_code);
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        assert_eq!(vm.registers[0], 1.into());
    }

    #[test]
    fn test_gt_opcode() {
        let default_code = vec![0x60, 0xa, 0x60, 0xb, 0x0b];
        let mut vm = VM::new(default_code);
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        assert_eq!(vm.registers[0], 0.into());
    }

}
