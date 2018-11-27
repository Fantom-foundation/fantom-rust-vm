//! Module that contains the VM that executes bytecode

use bigint::{Sign, M256, MI256, U256};
use errors::{Result, VMError};
use memory::{Memory, SimpleMemory};
use opcodes::Opcode;

pub struct VM {
    registers: [M256; 1024],
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

    /// Starts the execution loop for the VM
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

    /// Executes the next instruction only
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
            Opcode::SDIV => {
                self.stack_pointer -= 1;
                let s1 = MI256::from(self.registers[self.stack_pointer]);
                let s2 = MI256::from(self.registers[self.stack_pointer - 1]);
                let result = s1 / s2;
                let result: M256 = result.into();
                self.registers[self.stack_pointer - 1] = result.into();
                self.pc += 1;
            }
            Opcode::SMOD => {
                self.stack_pointer -= 1;
                let s1 = MI256::from(self.registers[self.stack_pointer]);
                let s2 = MI256::from(self.registers[self.stack_pointer - 1]);
                let result = s1 / s2;
                self.registers[self.stack_pointer - 1] = result.into();
                self.pc += 1;
            }
            Opcode::MOD => {
                self.stack_pointer -= 1;
                let result = self.registers[self.stack_pointer] % self.registers[self.stack_pointer - 1];
                self.registers[self.stack_pointer - 1] = result;
                self.pc += 1;
            }
            Opcode::ADDMOD => {
                self.stack_pointer -= 1;
                let result = (self.registers[self.stack_pointer] + self.registers[self.stack_pointer - 1])
                    % self.registers[self.stack_pointer - 2];
                if result == self.registers[self.stack_pointer - 2] {
                    self.registers[self.stack_pointer - 2] = result;
                } else {
                    self.registers[self.stack_pointer - 2] = 0.into();
                }
            }
            Opcode::MULMOD => {
                self.stack_pointer -= 1;
                let result = (self.registers[self.stack_pointer] * self.registers[self.stack_pointer - 1])
                    % self.registers[self.stack_pointer - 2];
                if result == self.registers[self.stack_pointer - 2] {
                    self.registers[self.stack_pointer - 2] = result;
                } else {
                    self.registers[self.stack_pointer - 2] = 0.into();
                }
            }
            Opcode::EXP => unimplemented!(),
            Opcode::SIGNEXTEND => unimplemented!(),
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
                self.pc += 1;
            }
            Opcode::SLT => {
                self.stack_pointer -= 1;
                let s1 = MI256::from(self.registers[self.stack_pointer]);
                let s2 = MI256::from(self.registers[self.stack_pointer - 1]);
                let result = s1 > s2;
                self.registers[self.stack_pointer - 1] = result.into();
                self.pc += 1;
            }
            Opcode::SGT => {
                self.stack_pointer -= 1;
                let s1 = MI256::from(self.registers[self.stack_pointer]);
                let s2 = MI256::from(self.registers[self.stack_pointer - 1]);
                let result = s1 < s2;
                self.registers[self.stack_pointer - 1] = result.into();
                self.pc += 1;
            }
            Opcode::EQ => {
                self.stack_pointer -= 1;
                if self.registers[self.stack_pointer] == self.registers[self.stack_pointer - 1] {
                    self.registers[self.stack_pointer - 1] = 1.into();
                } else {
                    self.registers[self.stack_pointer - 1] = 0.into();
                }
                self.pc += 1;
            }
            Opcode::ISZERO => {
                self.stack_pointer -= 1;
                if self.registers[self.stack_pointer] == 0.into() {
                    self.registers[self.stack_pointer] = 1.into()
                } else {
                    self.registers[self.stack_pointer] = 0.into()
                }
                self.pc += 1;
            }
            Opcode::AND => {
                self.stack_pointer -= 1;
                let s1 = self.registers[self.stack_pointer];
                let s2 = self.registers[self.stack_pointer - 1];
                self.registers[self.stack_pointer - 1] = s1 & s2;
                self.pc += 1;
            }
            Opcode::OR => {
                self.stack_pointer -= 1;
                let s1 = self.registers[self.stack_pointer];
                let s2 = self.registers[self.stack_pointer - 1];
                self.registers[self.stack_pointer - 1] = s1 | s2;
                self.pc += 1;
            }
            Opcode::XOR => {
                self.stack_pointer -= 1;
                let s1 = self.registers[self.stack_pointer];
                let s2 = self.registers[self.stack_pointer - 1];
                self.registers[self.stack_pointer - 1] = s1 ^ s2;
                self.pc += 1;
            }
            Opcode::NOT => {
                self.stack_pointer -= 1;
                let s1 = self.registers[self.stack_pointer];
                self.registers[self.stack_pointer] = !s1;
                self.pc += 1;
            }
            Opcode::BYTE => {
                self.stack_pointer -= 1;
                let s1 = self.registers[self.stack_pointer];
                let s2 = self.registers[self.stack_pointer - 1];
                let mut ret = M256::zero();
                for i in 0..256 {
                    if i < 8 && s1 < 32.into() {
                        let o: usize = s1.as_usize();
                        let t = 255 - (7 - i + 8 * o);
                        let bit_mask = M256::one() << t;
                        let value = (s2 & bit_mask) >> t;
                        ret = ret + (value << i);
                    }
                }
                self.registers[self.stack_pointer] = ret;
            }
            Opcode::SLOAD => unimplemented!(),
            Opcode::STORE => unimplemented!(),
            Opcode::MLOAD => unimplemented!(),
            Opcode::MSTORE => {
                self.stack_pointer -= 1;
                let offset = self.registers[self.stack_pointer];
                let value = self.registers[self.stack_pointer - 1];
                println!("Storing {:?} at {:?}", value, offset);
                if let Some(ref mut mem) = self.memory {
                    mem.write(value, offset)?;
                }
            }
            Opcode::MSTORE8 => {
                self.stack_pointer -= 1;
                let offset = self.registers[self.stack_pointer];
                let value = self.registers[self.stack_pointer - 1] % 256.into();
                if let Some(ref mut mem) = self.memory {
                    mem.write_byte(offset, (value.0.low_u32() & 0xFF) as u8)?;
                }
            }
            Opcode::MSIZE => {
                if let Some(ref mut mem) = self.memory {
                    self.registers[self.stack_pointer] = mem.size();
                } else {
                    return Err(VMError::MemoryError);
                }
            }
            Opcode::PUSH1 => {
                self.registers[self.stack_pointer] = M256::from(self.code[self.pc + 1] as i32);
                self.stack_pointer += 1;
                self.pc += 2;
            }
            _ => {
                return Err(VMError::UnknownOpcodeError);
            }
        };

        Ok(())
    }

    /// Utility function to print the values of a range of registers
    pub fn print_registers(&self, start: usize, end: usize) {
        println!("Registers are: ");
        for register in self.registers[start..end].iter() {
            print!("{:?} ", register);
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
    fn test_sdiv_opcode() {
        let default_code = vec![0x60, 0xa, 0x60, 0xa, 0x05];
        let mut vm = VM::new(default_code);
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        vm.print_registers(0, 10);
        assert_eq!(vm.registers[0], 1.into());
    }

    #[test]
    fn test_smod_opcode() {
        let default_code = vec![0x60, 0xa, 0x60, 0xb, 0x07];
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
        let default_code = vec![0x60, 0xa, 0x60, 0xb, 0x10];
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
        let default_code = vec![0x60, 0xa, 0x60, 0xb, 0x11];
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
    fn test_bitwise_and_opcode() {
        let default_code = vec![0x60, 0xa, 0x60, 0xb, 0x16];
        let mut vm = VM::new(default_code);
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        assert_eq!(vm.registers[0], 10.into());
    }

    #[test]
    fn test_bitwise_or_opcode() {
        let default_code = vec![0x60, 0xa, 0x60, 0xb, 0x17];
        let mut vm = VM::new(default_code);
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        assert_eq!(vm.registers[0], 11.into());
    }

    #[test]
    fn test_bitwise_xor_opcode() {
        let default_code = vec![0x60, 0xa, 0x60, 0xb, 0x18];
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
    fn test_addmod_opcode() {
        let default_code = vec![0x60, 0x0d, 0x60, 0x03, 0x60, 0x05, 0x08];
        let mut vm = VM::new(default_code);
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        assert_eq!(vm.registers[0], 13.into());
    }

    #[test]
    fn test_mulmod_opcode() {
        let default_code = vec![0x60, 0x10, 0x60, 0x05, 0x60, 0x05, 0x09];
        let mut vm = VM::new(default_code);
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        assert_eq!(vm.registers[0], 16.into());
    }

    #[test]
    fn test_memstore_opcode() {
        let default_code = vec![0x60, 0x05, 0x60, 0x01, 0x52];
        let mut vm = VM::new(default_code).with_simple_memory();
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let result = vm.execute_one();
        assert!(result.is_ok());
        let memory = vm.memory.unwrap();
        assert!(memory.size() > 0.into());
    }
}
