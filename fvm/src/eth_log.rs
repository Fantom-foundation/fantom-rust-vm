//! Contains the Log data structure

use bigint::{Address, H256};

/// A Log entry for the EVM
#[derive(Debug)]
pub struct Log {
    pub address: Address,
    pub topics: Vec<H256>,
    pub data: Vec<u8>,
}

impl Log {
  pub fn new(address: Address) -> Log {
    Log {
      address: address,
      topics: vec![],
      data: vec![]
    }
  }
}