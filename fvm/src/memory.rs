//! Module for the volatile memory that is cleared between transactions
use bigint::{M256, U256};
use errors::*;

/// A volatile area of memory that is created per-transaction. The follow constraints must be observed when interacting with it:
/// 1. A Read must be 256 bits
/// 2. A Write can be 8 bits or 256 bits
/// 3. Any expansion of the Memory area costs gas, and the cost scales quadratically
/// 4. Expansion is done by the word, so 256-bits at a time
pub trait Memory {
    fn read(&self, index: M256) -> M256;
    fn read_slice(&self, init_off_u: U256, init_size_u: U256) -> &[u8];
    fn read_byte(&self, index: M256) -> u8;
    fn write(&mut self, index: M256, value: M256) -> Result<()>;
    fn write_byte(&mut self, index: M256, value: u8) -> Result<()>;
    fn size(&self) -> M256;
    fn print(&self) -> String;
    fn copy_from_memory(&self, start: U256, len: U256) -> Vec<u8>;
    fn copy_into_memory(&mut self, values: &[u8], start: U256, value_start: U256, len: U256);
}

/// Simple implementation of memory using Rust Vecs
#[derive(Debug, PartialEq)]
pub struct SimpleMemory {
    memory: Vec<u8>,
    expansions: usize,
}

impl SimpleMemory {
    /// Creates and returns a new SimpleMemory
    pub fn new() -> SimpleMemory {
        SimpleMemory {
            memory: Vec::new(),
            expansions: 0,
        }
    }

    // Resizes the memory vector if needed. This will be called automatically
    fn resize_if_needed(&mut self, index: usize) -> Result<()> {
        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
            self.expansions += 1;
        }
        Ok(())
    }
}

impl Memory for SimpleMemory {
    /// Reads a `word` at the provided index
    fn read(&self, index: M256) -> M256 {
        let index = index.as_usize();
        self.memory[index..index + 32]
            .iter()
            .map(|v| v.clone())
            .collect::<Vec<u8>>()
            .as_slice()
            .into()
    }

    /// Reads a single byte at the provided index
    fn read_byte(&self, index: M256) -> u8 {
        self.memory[index.as_usize()].clone()
    }

	fn read_slice(&self, init_off_u: U256, init_size_u: U256) -> &[u8] {
		let off = init_off_u.low_u64() as usize;
		let size = init_size_u.low_u64() as usize;
        &self.memory[off..off+size]
	}

    /// Writes a `word` at the specified index. This will resize the capacity
    /// if needed, and will overwrite any existing bytes if there is overlap.
    fn write(&mut self, index: M256, value: M256) -> Result<()> {
        let index = index.as_usize();
        self.resize_if_needed(index)?;
        for i in 0..32 {
            let idx = M256::from(index + i);
            self.write_byte(idx, value.index(i))?;
        }
        Ok(())
    }

    /// Writes a single byte to the memory. This will resize the memory if
    /// needed.
    fn write_byte(&mut self, index: M256, value: u8) -> Result<()> {
        let index = index.as_usize();
        self.resize_if_needed(index)?;
        self.memory[index] = value;
        Ok(())
    }

    /// Returns the current size of memory in words
    fn size(&self) -> M256 {
        M256::from(self.memory.len())
    }

    /// Prints the contents of memory
    fn print(&self) -> String {
        String::from(format!("{:#?}", self.memory))
    }

    fn copy_from_memory(&self, start: U256, len: U256) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut i = start;
        while i < start + len {
            result.push(self.read_byte(i.into()));
            i = i + U256::from(1u64);
        }
        result
    }

    /// Copies a slice of values into memory with a start and end index
    fn copy_into_memory(&mut self, values: &[u8], start: U256, value_start: U256, len: U256) {
        let value_len = U256::from(values.len());
        let mut i = start;
        let mut j = value_start;
        while i < start + len {
            if j < value_len {
                let ju: usize = j.as_usize();
                self.write_byte(i.into(), values[ju]).unwrap();
                j = j + U256::from(1u64);
            } else {
                self.write_byte(i.into(), 0u8).unwrap();
            }
            i = i + U256::from(1u64);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_simple_mem_with_data() -> SimpleMemory {
        let test_value = U256::from(5000);
        let mut mem = SimpleMemory {
            memory: vec![0; 32],
            expansions: 0,
        };
        test_value.to_big_endian(&mut mem.memory);
        mem
    }

    #[test]
    fn create_simple_memory() {
        let mem = SimpleMemory::new();
        assert_eq!(mem.memory.len(), 0);
    }

    #[test]
    fn resize_memory() {
        let mut mem = SimpleMemory::new();
        let result = mem.resize_if_needed(1024);
        assert!(result.is_ok());
        assert_eq!(mem.memory.len(), 1025);
    }

    #[test]
    fn read_word() {
        let mem = gen_simple_mem_with_data();
        let read_data = mem.read(0.into());
        assert_eq!(read_data.as_u32(), 5000 as u32);
    }

    #[test]
    fn read_byte() {
        let mem = gen_simple_mem_with_data();
        let read_data = mem.read_byte(31.into());
        assert_eq!(read_data, 136 as u8);
    }
}
