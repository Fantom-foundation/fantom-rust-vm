//! Contains the Transaction module

use bigint::{U256, H160};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
pub mod pool;

/// Core data structure for interacting with the EVM
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Transaction {
    /// Nonce
    pub nonce: U256,
    /// Gas Price
    pub gas_price: U256,
    /// Start Gas
    pub start_gas: U256,
    /// Recipient
    /// If None, then this is a contract creation
    pub to: Option<H160>,
    /// Transferred value
    pub value: U256,
    /// Data
    pub data: Vec<u8>,
    /// The standardised V field of the signature.
    pub v: U256,
    /// The R field of the signature.
    pub r: U256,
    /// The S field of the signature.
    pub s: U256,
}

impl Serialize for Transaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Transaction", 9)?;
        state.serialize_field("nonce", &self.nonce.as_u64())?;
        state.serialize_field("gasPrice", &self.gas_price.as_u64())?;
        state.serialize_field("startGas", &self.start_gas.as_u64())?;
        if self.to.is_none() {
            state.serialize_field("to", &0)?;
        } else {
            state.serialize_field("to", &self.value.as_u64())?;
        }
        state.serialize_field("value", &self.value.as_u64())?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("v", &self.v.as_u64())?;
        state.serialize_field("r", &self.r.as_u64())?;
        state.serialize_field("s", &self.s.as_u64())?;
        state.end()
    }
}

/// A valid transaction is one where:
/// (i) the signature is well-formed (ie. 0 <= v <= 3, 0 <= r < P, 0 <= s < N, 0 <= r < P - N if v >= 2),
/// and (ii) the sending account has enough funds to pay the fee and the value.
impl Transaction {
    pub fn is_valid(&self) -> bool {
        unimplemented!()
    }

    fn sender_account(&mut self) {
        unimplemented!()
    }
}