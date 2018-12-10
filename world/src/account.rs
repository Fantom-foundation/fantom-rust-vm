use bigint::U256;

pub struct Account {
  nonce: u64,
  balance: u64,
  storage_root: U256,
  code: Vec<u8>,
}