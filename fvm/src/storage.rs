//! Module for the non-volatile memory that is maintained between transactions and holds account data
use bigint::U256;
use errors::*;

/// Non-volatile storage Trait, corresponds to Ethereum Storage. Can be backed by multiple implementations if desired.
pub trait Storage {
    fn read(&self, index: U256) -> U256;
    fn write(&mut self, index: U256, value: U256) -> Result<()>;
}