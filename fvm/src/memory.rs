//! Module for the volatile memory that is cleared between transactions
use bigint::{M256, U256};
use errors::*;

/// A volatile area of memory that is created per-transaction. The follow constraints must be observed when interacting with it:
/// 1. A Read must be 256 bits
/// 2. A Write can be 8 bits or 256 bits
/// 3. Any expansion of the Memory area costs gas, and the cost scales quadratically
/// 4. Expansion is done by the word, so 256-bits at a time
pub trait Memory {
    fn read(&self, index: U256) -> M256;
    fn read_byte(&self, index: U256) -> u8;
    fn write(&mut self, index: U256, value: U256) -> Result<()>;
    fn write_byte(&mut self, index: U256, value: u8) -> Result<()>;
}

/// Simple implementation of memory using Rust Vecs
pub struct SimpleMemory {
    memory: Vec<u8>,
}

impl SimpleMemory {
    /// Creates and returns a new SimpleMemory
    /// 
    /// # Example
    /// ```
    /// use memory::SimpleMemory;
    /// let mem = SimpleMemory::new();
    /// ```
    pub fn new() -> SimpleMemory {
        SimpleMemory { memory: Vec::new() }
    }

    // Resizes the memory vector if needed. This will be called automatically
    fn resize_if_needed(&mut self, index: usize) -> Result<()> {
        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }
        Ok(())
    }
}

impl Memory for SimpleMemory {
    /// Reads a `word` at the provided index
    fn read(&self, index: U256) -> M256 {
        let index = index.as_usize();
        self.memory[index..index + 32]
            .iter()
            .map(|v| v.clone())
            .collect::<Vec<u8>>().as_slice().into()
    }

    /// Reads a single byte at the provided index
    fn read_byte(&self, index: U256) -> u8 {
        self.memory[index.as_usize()].clone()
    }

    /// Writes a `word` at the specified index. This will resize extend the capacity
    /// if needed, and will overwrite any existing bytes if there is overlap.
    fn write(&mut self, index: U256, value: U256) -> Result<()> {
        let index = index.as_usize();
        self.resize_if_needed(index)?;
        for i in 0..32 {
            let idx = U256::from(index + i);
            self.write_byte(idx, value.index(i))?;
        }
        Ok(())
    }

    fn write_byte(&mut self, index: U256, value: u8) -> Result<()> {
        let index = index.as_usize();
        self.memory[index] = value;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_simple_mem_with_data() -> SimpleMemory {
        let test_value = U256::from(5000);
        let mut mem = SimpleMemory {
            memory: vec![0; 32]
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