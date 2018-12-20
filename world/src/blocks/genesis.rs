use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::collections::HashMap;

/// Genesis block data structure
#[derive(Serialize, Deserialize, Debug)]
pub struct Genesis {
  nonce: u64,
  timestamp: u64,
  extra_data: Vec<u8>,
  #[serde(rename = "gasLimit")]
  gas_limit: u64,
  difficulty: u64,
  coinbase: String,
  mixhash: String,
  alloc: Option<HashMap<String, GenesisAlloc>>,
  config: GenesisConfig,
  #[serde(rename = "parentHash")]
  parent_hash: String,
}

/// Simple struct to contain the initial accounts for allocation in 
#[derive(Serialize, Deserialize, Debug)]
pub struct GenesisAlloc {
  balance: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenesisConfig {
  #[serde(rename = "chainId")]
  chain_id: u64,
  #[serde(rename = "homesteadBlock")]
  homestead_block: u64,
  #[serde(rename = "eip155Block")]
  eip155_block: u64,
  #[serde(rename = "eip158Block")]
  eip158_block: u64
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
      },
      Err(e) => {
        Err(serde_json::Error::io(e))
      }
    }
  }
}