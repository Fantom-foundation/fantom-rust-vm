use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;

use bigint::{B256, H256, H64};
use bloom;

/// Genesis block data structure
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Genesis {
    #[serde(rename = "parentHash")]
    parent_hash: String,
    #[serde(rename = "ommersHash")]
    ommers_hash: String,
    nonce: u64,
    timestamp: u64,
    extra_data: Vec<u8>,
    #[serde(rename = "gasLimit")]
    gas_limit: u64,
    #[serde(rename = "gasUsed")]
    gas_used: u64,
    pub difficulty: u64,
    coinbase: String,
    mixhash: String,
    alloc: Option<HashMap<String, GenesisAlloc>>,
    config: GenesisConfig,
}

/// Simple struct to contain the initial accounts for allocation in
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenesisAlloc {
    balance: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenesisConfig {
    #[serde(rename = "chainId")]
    chain_id: u64,
    #[serde(rename = "homesteadBlock")]
    homestead_block: u64,
    #[serde(rename = "eip155Block")]
    eip155_block: u64,
    #[serde(rename = "eip158Block")]
    eip158_block: u64,
}

impl Genesis {
    /// Attempts to load a Genesis block from a json file
    pub fn load(path: PathBuf) -> Result<Box<Genesis>, serde_json::Error> {
        match File::open(path) {
            Ok(mut fh) => {
                let mut buf = String::new();
                let _ = fh.read_to_string(&mut buf);
                let block: Genesis = serde_json::from_str(&buf)?;
                Ok(Box::new(block))
            }
            Err(e) => Err(serde_json::Error::io(e)),
        }
    }
}

impl Into<block::Header> for Genesis {
    fn into(self) -> block::Header {
        let mut account_bytes: Vec<u8> = vec![];
        for (address, _amt) in self.alloc.unwrap() {
            account_bytes.append(&mut address.clone().into_bytes());
        }

        let state_root_hash = H256::from_slice(&account_bytes);
        block::Header {
            parent_hash: H256::from_str(&self.parent_hash).unwrap(),
            ommers_hash: H256::from_str(&self.ommers_hash).unwrap(),
            beneficiary: 0.into(),
            state_root: state_root_hash,
            transactions_root: H256::zero(),
            receipts_root: H256::zero(),
            logs_bloom: bloom::LogsBloom::new(),
            difficulty: self.difficulty.into(),
            number: 0.into(),
            gas_limit: self.gas_limit.into(),
            gas_used: self.gas_used.into(),
            timestamp: self.timestamp,
            extra_data: B256::new(&self.extra_data),
            mix_hash: H256::from_str(&self.mixhash).unwrap(),
            nonce: H64::from(self.nonce),
        }
    }
}
