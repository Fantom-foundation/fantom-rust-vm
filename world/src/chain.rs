//! Contains the base BlockChain structure
use std::path::PathBuf;
use blocks::Block;
use blocks::genesis::Genesis;
use db::RDB;

/// Core data structure that contains the Blocks that make up the chain
pub struct BlockChain {
    db: RDB,
    genesis_block: Box<Genesis>,
}

impl BlockChain {
    pub fn new(db: RDB, genesis_block: Box<Genesis>) -> BlockChain {
        BlockChain { db, genesis_block }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::create_temporary_db;

    fn load_test_genesis_block() -> Box<Genesis> {
      let path = PathBuf::from("../templates/genesis.json");
      let block = Genesis::load(path).expect("Unable to load genesis block");
      block
    }

    #[test]
    fn create_new_blockchain() {
        let mut tmp_db = create_temporary_db().expect("Unable to create temporary DB");
        let mut genesis_block = load_test_genesis_block();
        let mut test_chain = BlockChain::new(tmp_db.0, genesis_block);
    }
}
