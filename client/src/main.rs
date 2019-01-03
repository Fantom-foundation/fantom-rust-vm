#![feature(proc_macro_hygiene, decl_macro)]
extern crate base64;
extern crate bigint;
extern crate block;
extern crate byteorder;
extern crate chrono;
#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate ethash;
extern crate fvm;
extern crate hmac;
#[macro_use]
extern crate log;
extern crate mac;
extern crate openssl;
extern crate pbkdf2;
extern crate rand;
extern crate rlp;
#[macro_use]
extern crate rocket;
extern crate rpassword;
extern crate rustc_serialize;
extern crate secp256k1;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;
extern crate sha3;
extern crate uuid;
extern crate world;

use std::{fs, fs::File, io};
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;
use std::{str, thread};

use clap::App;
use hmac::Hmac;
use sha2::Sha256;

pub mod accounts;
pub mod keys;
pub mod servers;

type HmacSha256 = Hmac<Sha256>;

const DIRECTORIES: [&'static str; 6] = ["data", "raft", "eth", "lachesis", "keys", "chaindata"];
pub fn main() {
    env_logger::init();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Setup data directories if not present
    let base_dir = matches
        .value_of("data-directory")
        .expect("Data directory parameter not found");
    if create_directories(base_dir).is_err() {
        error!("Unable to create all directories");
        exit(1);
    }

    info!("All needed directories present");

    debug!("Checking for genesis block...");
    if genesis_block_exists(base_dir) {
        debug!("Genesis block exists!");
        let _genesis_block_data = load_genesis_block(base_dir).expect("Unable to load genesis block");
    } else {
        debug!("Genesis block does not exist, please create it.");
    }

    // This handles the user wanting to do account-related functions
    if let Some(account_matches) = matches.subcommand_matches("account") {
        accounts::handler::handle_cli_command(account_matches, base_dir);
        exit(0);
    }

    info!("Opening DB...");
    match world::db::create_persistent_db(&(base_dir.to_string() + "data/"), "fantom") {
        Ok((rdb, _store)) => {
            info!("Created: {:?}", rdb);
            thread::spawn(move || {
                let _miner = world::consensus::miner::Miner::new();
            });
        }
        Err(e) => {
            error!("Could not create database: {:?}", e);
            exit(1);
        }
    }

    servers::web::start_web();
    exit(0);
}

fn create_directories(path: &str) -> Result<(), io::Error> {
    debug!("Creating data directories from base: {:?}", path);
    for dir_name in &DIRECTORIES {
        match create_directory(path, dir_name) {
            Ok(_) => {
                debug!("Directory {:?} created", dir_name);
            },
            Err(e) => {
                error!("Error creating directory {:?}. Error was: {:?}", dir_name, e);
            }
        }
    }
    Ok(())
}

fn create_directory(path: &str, end: &str) -> Result<(), io::Error> {
    fs::create_dir_all(path.to_string() + end)?;
    Ok(())
}

fn data_directory(path: &str) -> PathBuf {
    PathBuf::from(path.to_string() + "data")
}

fn genesis_block_exists(path: &str) -> bool {
    let genesis_block_path = path.to_string() + "/eth/genesis.json";
    std::path::Path::new(&genesis_block_path).exists()
}

fn load_genesis_block(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let genesis_block_path = path.to_string() + "/eth/genesis.json";
    match File::open(genesis_block_path) {
        Ok(mut fh) => {
            let mut buf: Vec<u8> = vec![];
            match fh.read_to_end(&mut buf) {
                Ok(_bytes) => info!("Read genesis block!"),
                Err(_e) => error!("Unable to read genesis block"),
            };
            Ok(buf)
        }
        Err(e) => Err(e),
    }
}

fn chain_data_path(base_dir: &str) -> PathBuf {
    std::path::PathBuf::from(base_dir.to_string() + "chaindata")
}
