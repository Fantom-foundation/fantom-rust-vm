#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use]
extern crate clap;
extern crate fvm;
extern crate secp256k1;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;
extern crate uuid;
extern crate rpassword;
extern crate openssl;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate pbkdf2;

use std::{fs, io};
use std::process::exit;
use rand::os::OsRng;
use rand::Rng;

use clap::App;

pub mod servers;
pub mod keys;

use openssl::symm;
use openssl::rsa::{Padding, Rsa};
use openssl::symm::Cipher;

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
    }

    if let Some(account_matches) = matches.subcommand_matches("account") {
        if let Some(_) = account_matches.subcommand_matches("new") {
            debug!("Creating new account");
            let passphrase = keys::get_passphrase();
            match keys::generate_random_keypair() {
                Ok(keypair) => {
                    let cipher = symm::Cipher::aes_128_ctr();
                    let passphrase = passphrase.expect("Unable to get passphrase");
                    let r = pbkdf2::pbkdf2_simple(&passphrase.as_str(), 10).expect("Unable to generate key");
                    let account_id = uuid::Uuid::new_v4();
                    exit(0);
                },
                Err(e) => {
                    error!("There was an error creating a new account: {:?}", e);
                    exit(1);
                }
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("transaction-test") {
        if matches.is_present("INPUT") {
            let _filename = matches.value_of("INPUT").unwrap();
            // let _bytecode = read_bytecode(filename);
        } else {
            error!("Please specify the file containing the EVM bytecode");
            exit(1);
        }
    }

    servers::web::start_web();
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