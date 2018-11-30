use bigint::{Address, M256, U256};
use errors::StorageError;
use std::collections::HashMap;
type Map<U256, M256> = HashMap<U256, M256>;

#[derive(Debug, Clone)]
pub struct Storage {
    address: Address,
    storage: Map<U256, M256>,
}

impl Into<Map<U256, M256>> for Storage {
    fn into(self) -> Map<U256, M256> {
        self.storage
    }
}

impl Storage {
    /// Create a new storage.
    pub fn new(address: Address) -> Storage {
        Storage {
            address,
            storage: Map::new(),
        }
    }

    /// Commit a value into the storage.
    fn commit(&mut self, index: U256, value: M256) -> Result<(), StorageError> {
        if self.storage.contains_key(&index) {
            return Err(StorageError::AlreadyCommitted);
        }

        self.storage.insert(index, value);
        Ok(())
    }

    /// Read a value from the storage.
    pub fn read(&self, index: U256) -> Result<M256, StorageError> {
        match self.storage.get(&index) {
            Some(&v) => Ok(v),
            None => Ok(M256::zero()),
        }
    }

    /// Write a value into the storage.
    pub fn write(&mut self, index: U256, value: M256) -> Result<(), StorageError> {
        if !self.storage.contains_key(&index) {
            return Err(StorageError::RequireError);
            //::AccountStorage(self.address, index));
        }
        self.storage.insert(index, value);
        Ok(())
    }

    /// Return the number of changed/full items in storage.
    pub fn len(&self) -> usize {
        self.storage.len()
    }

    /// Return true if storage is empty, false otherwise
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
