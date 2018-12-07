#[macro_use]
extern crate clap;
extern crate fvm;
extern crate secp256k1;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

use std::{fs, io};
use std::process::exit;

use clap::App;
use fvm::vm::VM;

pub mod servers;
pub mod keys;

pub fn main() {
    env_logger::init();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    // Setup data directories if not present
    let base_dir = matches.value_of("data-directory").expect("Data directory parameter not found");
    if create_directories(base_dir).is_err() {
            error!("Unable to create all directories");
            exit(1);
    }
    info!("All needed directories present");


    debug!("Checking for genesis block...");
    if genesis_block_exists(base_dir) {
        debug!("Genesis block exists!");
    } else {
        debug!("Genesis block does not exist, please create it.");
        exit(1);
    }

    if let Some(account_matches) = matches.subcommand_matches("account") {
        if let Some(new_matches) = account_matches.subcommand_matches("new") {

        }
    }

    if let Some(matches) = matches.subcommand_matches("transaction-test") {
        if matches.is_present("INPUT") {
            let filename = matches.value_of("INPUT").unwrap();
            // let _bytecode = read_bytecode(filename);
        } else {
            error!("Please specify the file containing the EVM bytecode");
            exit(1);
        }
    }

    println!("Gooodbye!");
    exit(0);
}

fn create_directories(path: &str) -> Result<(), io::Error> {
    debug!("Creating data directories from base: {:?}", path);
    create_raft_directory(path)?;
    create_eth_directory(path)?;
    create_lachesis_directory(path)?;
    create_data_directory(path)?;
    create_keys_directory(path)?;
    Ok(())
}

fn create_data_directory(path: &str) -> Result<(), io::Error> {
    fs::create_dir_all(path.to_string() + "data")?;
    Ok(())
}

fn create_keys_directory(path: &str) -> Result<(), io::Error> {
    fs::create_dir_all(path.to_string() + "keys")?;
    Ok(())
}

fn create_raft_directory(path: &str) -> Result<(), io::Error> {
    fs::create_dir_all(path.to_string() + "raft")?;
    Ok(())
}

fn create_eth_directory(path: &str) -> Result<(), io::Error> {
    fs::create_dir_all(path.to_string() + "eth")?;
    Ok(())
}

fn create_lachesis_directory(path: &str) -> Result<(), io::Error> {
    fs::create_dir_all(path.to_string() + "lachesis")?;
    Ok(())
}

fn genesis_block_exists(path: &str) -> bool {
    let genesis_block_path = path.to_string() + "/eth/genesis.json";
    std::path::Path::new(&genesis_block_path).exists()
}