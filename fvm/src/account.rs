
use std::rc::Rc;
use bigint::{M256, U256, Address};
use storage::Storage;

#[derive(Debug, Clone)]
/// A single account commitment.
pub enum AccountCommitment {
    /// Full account commitment. The client that committed account
    /// should not change the account in other EVMs if it decides to
    /// accept the result.
    Full {
        /// Nonce of the account.
        nonce: U256,
        /// Account address.
        address: Address,
        /// Account balance.
        balance: U256,
        /// Code associated with this account.
        code: Rc<Vec<u8>>,
    },
    /// Commit only code of the account. The client can keep changing
    /// it in other EVMs if the code remains unchanged.
    Code {
        /// Account address.
        address: Address,
        /// Code associated with this account.
        code: Rc<Vec<u8>>,
    },
    /// Commit a storage. Must be used given a full account.
    Storage {
        /// Account address.
        address: Address,
        /// Account storage index.
        index: U256,
        /// Value at the given account storage index.
        value: M256,
    },
    /// Indicate that an account does not exist, or is a suicided
    /// account.
    Nonexist(Address),
}

impl AccountCommitment {
    /// Address of this account commitment.
    pub fn address(&self) -> Address {
        match *self {
            AccountCommitment::Full {
                address,
                ..
            } => address,
            AccountCommitment::Code {
                address,
                ..
            } => address,
            AccountCommitment::Storage {
                address,
                ..
            } => address,
            AccountCommitment::Nonexist(address) => address,
        }
    }
}

#[derive(Debug, Clone)]
/// Represents an account. This is usually returned by the EVM.
pub enum AccountChange {
    /// A full account. The client is expected to replace its own account state with this.
    Full {
        /// Account nonce.
        nonce: U256,
        /// Account address.
        address: Address,
        /// Account balance.
        balance: U256,
        /// Change storage with given indexes and values.
        changing_storage: Storage,
        /// Code associated with this account.
        code: Rc<Vec<u8>>,
    },
    /// Only balance is changed, and it is increasing for this address.
    IncreaseBalance(Address, U256),
    /// Create or delete a (new) account.
    Create {
        /// Account nonce.
        nonce: U256,
        /// Account address.
        address: Address,
        /// Account balance.
        balance: U256,
        /// All storage values of this account, with given indexes and values.
        storage: Storage,
        /// Code associated with this account.
        code: Rc<Vec<u8>>
    },
    /// The account should remain nonexist, or should be deleted if
    /// exists.
    Nonexist(Address)
}

impl AccountChange {
    /// Address of this account.
    pub fn address(&self) -> Address {
        match *self {
            AccountChange::Full {
                address,
                ..
            } => address,
            AccountChange::IncreaseBalance(address, _) => address,
            AccountChange::Create {
                address,
                ..
            } => address,
            AccountChange::Nonexist(address) => address,
        }
    }
}