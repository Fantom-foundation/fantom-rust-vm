//! Module for the volatile memory that is cleared between transactions
use ethereum_types::U256;
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

