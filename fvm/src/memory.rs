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
    pub fn new() -> SimpleMemory {
        SimpleMemory { memory: Vec::new() }
    }

    /// Resizes the memory vector if neexed
    fn resize_if_needed(&mut self, index: usize) -> Result<()> {
        if index >= self.memory.len() {
            self.memory.resize(index + 1, 0);
        }
        Ok(())
    }
}

impl Memory for SimpleMemory {
    fn read(&self, index: U256) -> M256 {
        let index = index.as_usize();
        self.memory[index..index + 32]
            .iter()
            .map(|v| v.clone())
            .collect::<Vec<u8>>()
            .as_slice()
            .into()
    }

    fn read_byte(&self, index: U256) -> u8 {
        self.memory[index.as_usize()].clone()
    }

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
