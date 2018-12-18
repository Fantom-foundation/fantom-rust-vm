use world::{db, transaction::*};
use std::collections::HashSet;
use bigint::H256;
use std::sync::Arc;

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
	// Underlaying state.
	//pub state: State<StateDB>,
	// Transaction traces.
	//pub traces: Tracing,
	// Hashes of last 256 blocks.
	// pub last_hashes: Arc<LastHashes>,
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

}