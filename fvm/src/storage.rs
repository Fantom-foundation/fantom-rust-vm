//! Module for the non-volatile memory that is maintained between transactions and holds account data
use std::collections::HashMap;
use bigint::{U256, M256, Address};

use errors::VMError;

/// Non-volatile storage Trait, corresponds to Ethereum Storage. Can be backed by multiple implementations if desired.
pub trait Storage {
    fn read(&self, index: U256) -> M256;
    fn write(&mut self, index: U256, value: U256) -> Result<(), VMError>;
}

/// Simple persistent storage
pub struct SimpleStorage {
  address: Address,
  storage: HashMap<U256, M256>
}

// impl SimpleStorage {
//   pub fn new() -> SimpleStorage {
//     SimpleStorage {
//       FixedSecureTrie
//     }
//   }
// }