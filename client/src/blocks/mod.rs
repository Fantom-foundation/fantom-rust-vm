use world::{db::RDB, transaction::*};
use std::collections::HashSet;
use bigint::{B256, U256, H256, Address, H64, Gas};
use bloom::LogsBloom;
use std::sync::Arc;

pub mod receipt;

pub type LastHashes = Vec<H256>;

pub struct ExecutedBlock {
  /// Executed block header.
	pub header: Header,
	/// Executed transactions.
	pub transactions: Vec<SignedTransaction>,
	/// Uncles.
	pub uncles: Vec<Header>,
	/// Transaction receipts.
	//pub receipts: Vec<Receipt>,
	/// Hashes of already executed transactions.
	pub transactions_set: HashSet<H256>,
	/// Underlaying state.
	pub state: RDB,
	// Transaction traces.
	//pub traces: Tracing,
	/// Hashes of last 256 blocks.
	pub last_hashes: Arc<LastHashes>,
}

/// A block, encoded as it is on the block chain.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Block {
	/// The header of this block.
	pub header: Header,
	/// The transactions in this block.
	pub transactions: Vec<UnverifiedTransaction>,
	/// The uncles of this block.
	pub uncles: Vec<Header>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Header {
    pub parent_hash: H256,
    pub ommers_hash: H256,
    pub beneficiary: Address,
    pub state_root: H256,
    pub transactions_root: H256,
    pub receipts_root: H256,
    pub logs_bloom: LogsBloom,
    pub difficulty: U256,
    pub number: U256,
    pub gas_limit: Gas,
    pub gas_used: Gas,
    pub timestamp: u64,
    pub extra_data: B256,
    pub mix_hash: H256,
    pub nonce: H64,
}