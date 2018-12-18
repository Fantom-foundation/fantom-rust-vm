use bigint::{U256, Address, H256};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Transaction {
  nonce: U256,
  gasprice: U256,
  startgas: U256,
  to: Address,
  value: U256,
  data: Vec<u8>,
  v: U256,
  r: U256,
  s: U256
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SignedTransaction {
	transaction: UnverifiedTransaction,
	sender: Address,
	public: Option<Address>,
}

/// Signed transaction information without verified signature.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UnverifiedTransaction {
	/// Plain Transaction.
	unsigned: Transaction,
	/// The V field of the signature; the LS bit described which half of the curve our point falls
	/// in. The MS bits describe which chain this transaction is for. If 27/28, its for all chains.
	v: u64,
	/// The R field of the signature; helps describe the point on the curve.
	r: U256,
	/// The S field of the signature; helps describe the point on the curve.
	s: U256,
	/// Hash of the transaction
	hash: H256,
}