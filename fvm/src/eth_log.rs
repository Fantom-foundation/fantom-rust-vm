//! Contains the Log data structure

use bigint::{Address, Gas, B256, H256, H64, U256};
use rlp::{Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};

/// A Log entry for the EVM
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Log {
    pub address: Address,
    pub topics: Vec<H256>,
    pub data: Vec<u8>,
}

impl Log {
    /// Creates and returns a new Log entry
    pub fn new(address: Address) -> Log {
        Log {
            address: address,
            topics: vec![],
            data: vec![],
        }
    }
}

/// Implements rlp::Encodable so we can write it to the DB
impl Encodable for Log {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(3);
        s.append(&self.address);
        s.append_list(&self.topics);
        s.append(&self.data);
    }
}

/// Implements rlp::Decodable so we can read it from the DB
impl Decodable for Log {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        Ok(Self {
            address: rlp.val_at(0)?,
            topics: rlp.list_at(1)?,
            data: rlp.val_at(2)?,
        })
    }
}
