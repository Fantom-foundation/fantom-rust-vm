//! Contains the base BlockChain structure
use bigint;
use blocks::genesis::Genesis;
use consensus::miner::Miner;
use db::RDB;
use block::Block;

/// Core data structure that contains the Blocks that make up the chain
pub struct BlockChain {
    #[allow(dead_code)]
    db: RDB,
    genesis_block: Box<Genesis>,
    pub miner: Miner,
    current_block: bigint::U256,
    blocks: Vec<Block>
}

impl BlockChain {
    pub fn new_from_genesis(db: RDB, genesis_block: Box<Genesis>) -> BlockChain {
        BlockChain {
            db,
            genesis_block,
            miner: Miner::new(),
            current_block: bigint::U256::from(0),
            blocks: vec![],
        }
    }

    pub fn mine(&mut self) -> (bigint_miner::H64, bigint_miner::H256) {
        let tmp_block: block::Header = (*self.genesis_block.clone()).into();
        self.miner.mine(&tmp_block, 1)
    }

    pub fn get_current_block(&self) -> bigint::U256 {
        self.current_block
    }

    pub fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    pub fn start(&mut self) {

    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::{BlockChain, Genesis};
    use db::create_temporary_db;

    fn load_test_genesis_block() -> Box<Genesis> {
        let path = PathBuf::from("../templates/genesis.json");
        let block = Genesis::load(path).expect("Unable to load genesis block");
        block
    }

    #[test]
    fn create_new_blockchain() {
        let tmp_db = create_temporary_db().expect("Unable to create temporary DB");
        let genesis_block = load_test_genesis_block();
        let mut test_chain = BlockChain::new_from_genesis(tmp_db.0, genesis_block);
    }

    #[test]
    #[ignore]
    fn save_blockchain() {
        let tmp_db = create_temporary_db().expect("Unable to create temporary DB");
        let genesis_block = load_test_genesis_block();
        let mut test_chain = BlockChain::new_from_genesis(tmp_db.0, genesis_block);
        let _ = test_chain.mine();
    }
}
