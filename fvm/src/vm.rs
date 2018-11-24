//! Module that contains the VM that executes bytecode

use bigint::U256;
use memory::{Memory, SimpleMemory};
use opcodes::Opcode;
use errors::{Result, VMError};

pub struct VM {
    registers: [U256; 1024],
    memory: Option<Box<Memory>>,
    code: Vec<u8>,
    pc: usize,
    stack_pointer: usize
}

impl VM {
    /// Creates and returns a new VM
    pub fn new(code: Vec<u8>) -> VM {
        VM {
            registers: [0.into(); 1024],
            memory: None,
            stack_pointer: 0,
            code: code,
            pc: 0
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
                Ok(_) => { continue; },
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
                println!("Executing ADD");
                let result = self.registers[self.stack_pointer] + self.registers[self.stack_pointer - 1];
                self.registers[self.stack_pointer - 1] = result;
                self.stack_pointer -= 1;
                self.pc += 1;
            }
            Opcode::MUL => {
                let result = self.registers[self.stack_pointer] * self.registers[self.stack_pointer - 1];
                self.registers[self.stack_pointer - 1] = result;
                self.pc += 1;
                self.stack_pointer -= 1;
            }
            Opcode::SUB => {
                let result = self.registers[self.stack_pointer] - self.registers[self.stack_pointer - 1];
                self.registers[self.stack_pointer - 1] = result;
                self.pc += 1;
                self.stack_pointer -= 1;
            }
            Opcode::DIV => {
                let result = self.registers[self.stack_pointer] / self.registers[self.stack_pointer - 1];
                self.registers[self.stack_pointer - 1] = result;
                self.pc += 1;
                self.stack_pointer -= 1;
            }
            Opcode::PUSH1 => {
                let value = self.code[self.pc + 1];
                println!("Executing PUSH1 with {:?}", value);
                self.registers[self.stack_pointer] = value.into();
                println!("SP1 is: {:?}", self.stack_pointer);
                self.stack_pointer += 1;
                println!("SP2 is: {:?}", self.stack_pointer);
                self.pc += 2;
                self.print_registers(0, 10);
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
            pc: 0
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
        assert_eq!(vm.registers[0], 0x0a.into());
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
}
