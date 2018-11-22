use std::collections::HashMap;

use bigint::{U256, M256};

use storage::Storage;

/// Represents an External Account
pub struct Account {
    pub nonce: U256,
    pub balance: U256,
    pub storage: Box<Storage>,
    pub code: Vec<u8>,
}