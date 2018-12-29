use bigint_miner;
use block;
use consensus::ethash;

#[derive(Default)]
pub struct Miner;

impl Miner {
    pub fn new() -> Miner {
        Miner {}
    }

    pub fn mine(&self, header: &block::Header, epoch: usize) -> (bigint_miner::H64, bigint_miner::H256) {
        println!("Getting sizes...");
        let full_size = ethash::get_full_size(epoch);
        let cache_size = ethash::get_cache_size(epoch);
        println!("Populating datasets with zeros");
        let mut dataset: Vec<u8> = vec![0; full_size];
        let mut cache: Vec<u8> = vec![0; cache_size];
        println!("Getting seed");
        let seed = ethash::get_seedhash(epoch);
        println!(
            "Making cache and dataset. Cache is: {:?}. Dataset is {:?}",
            cache_size, full_size
        );
        ethash::make_cache(&mut cache, seed);
        println!("Cache done");
        ethash::make_dataset(&mut dataset, &cache);
        println!("Dataset done");
        let diff = header.difficulty.as_u32();
        let difficulty = bigint_miner::U256::from(diff);
        println!("Mining difficulty is: {:?}", difficulty);
        ethash::mine(header, full_size, &dataset, bigint_miner::H64::random(), difficulty)
    }
}

#[cfg(test)]
mod tests {}
