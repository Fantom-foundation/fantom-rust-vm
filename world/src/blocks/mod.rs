use bigint::{Address, Gas, B256, H256, H64, U256};
use block::Transaction;
use bloom::LogsBloom;
use rlp::{Decodable, DecoderError, Encodable, RlpStream, UntrustedRlp};
pub mod genesis;
pub mod receipt;

pub type LastHashes = Vec<H256>;

/// A block, encoded as it is on the block chain.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Block {
    /// The header of this block.
    pub header: Header,
    /// The transactions in this block.
    pub transactions: Vec<Transaction>,
    /// The uncles of this block.
    pub ommers: Vec<Header>,
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

impl Encodable for Block {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(3);
        s.append(&self.header);
        s.append_list(&self.transactions);
        s.append_list(&self.ommers);
    }
}

impl Decodable for Block {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        Ok(Self {
            header: rlp.val_at(0)?,
            transactions: rlp.list_at(1)?,
            ommers: rlp.list_at(2)?,
        })
    }
}

impl Encodable for Header {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(15);
        s.append(&self.parent_hash);
        s.append(&self.ommers_hash);
        s.append(&self.beneficiary);
        s.append(&self.state_root);
        s.append(&self.transactions_root);
        s.append(&self.receipts_root);
        s.append(&self.logs_bloom);
        s.append(&self.difficulty);
        s.append(&self.number);
        s.append(&self.gas_limit);
        s.append(&self.gas_used);
        s.append(&self.timestamp);
        s.append(&self.extra_data);
        s.append(&self.mix_hash);
        s.append(&self.nonce);
    }
}

impl Decodable for Header {
    fn decode(rlp: &UntrustedRlp) -> Result<Self, DecoderError> {
        Ok(Self {
            parent_hash: rlp.val_at(0)?,
            ommers_hash: rlp.val_at(1)?,
            beneficiary: rlp.val_at(2)?,
            state_root: rlp.val_at(3)?,
            transactions_root: rlp.val_at(4)?,
            receipts_root: rlp.val_at(5)?,
            logs_bloom: rlp.val_at(6)?,
            difficulty: rlp.val_at(7)?,
            number: rlp.val_at(8)?,
            gas_limit: rlp.val_at(9)?,
            gas_used: rlp.val_at(10)?,
            timestamp: rlp.val_at(11)?,
            extra_data: rlp.val_at(12)?,
            mix_hash: rlp.val_at(13)?,
            nonce: rlp.val_at(14)?,
        })
    }
}
