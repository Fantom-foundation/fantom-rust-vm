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
use rand::{os::OsRng, Rng};
use rustc_serialize::hex::ToHex;
use sha2::Sha256;
use sha3::{Digest, Keccak256};

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
        // This handles the user wanting to create a new account
        if account_matches.subcommand_matches("new").is_some() {
            debug!("Creating new account");
            // Generate a random public/private key
            match keys::generate_random_keypair() {
                Ok((secret_key, public_key)) => {
                    let mut generator = OsRng::new().expect("Unable to generate OsRng");
                    let account_id = uuid::Uuid::new_v4();
                    let (ciphertext, iv) = accounts::Account::generate_cipher_text(&mut generator, &secret_key);
                    let address = accounts::Account::get_address(public_key);
                    let new_account = account_from_passphrase(account_id, &iv, &ciphertext, &address);
                    let account_json = serde_json::to_string(&new_account).unwrap();
                    let filename = get_account_filename(&new_account);
                    let path = accounts::Account::key_file_path(base_dir, &filename);
                    debug!("Path is: {:#?}", path);
                    match File::create(path) {
                        Ok(mut fh) => {
                            match fh.write_all(account_json.as_bytes()) {
                                Ok(_) => {
                                    info!("Keyfile written: {}", filename);
                                    exit(0);
                                }
                                Err(e) => {
                                    error!("Error writing keyfile: {:#?}", e);
                                    exit(1);
                                }
                            }
                        },
                        Err(e) => {
                            error!("Error creating account file: {:#?}", e);
                            exit(1);
                        }
                    }
                }
                Err(e) => {
                    error!("There was an error creating a new account: {:?}", e);
                    exit(1);
                }
            }
        }
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

fn get_account_filename(account_id: &accounts::Account) -> String {
    let now = chrono::Utc::now();
    "UTC--".to_string()
        + &now.format("%Y-%m-%d").to_string()
        + "T"
        + &now.format("%H-%M-%SZ").to_string()
        + "--"
        + &account_id.get_id()
        + ".json"
}

fn account_from_passphrase(account_id: uuid::Uuid, iv: &[u8], ciphertext: &str, address: &str) -> accounts::Account {
    // This is the passphrase we'll use to encrypt their secret key, and they will need to
    // provide to decrypt it
    let mut generator = OsRng::new().expect("Unable to generate OsRng");
    let passphrase = keys::get_passphrase();
    let count: u32 = 64000 + rand::thread_rng().gen_range(0, 20000);
    let salt: Vec<u8> = generator.gen_iter::<u8>().take(16).collect();
    let passphrase = passphrase.expect("Unable to get passphrase");
    let mut dk = [0u8; 32];
    pbkdf2::pbkdf2::<Hmac<Sha256>>(&passphrase.as_bytes(), &salt, count as usize, &mut dk);
    let mut bytes_to_hash: Vec<u8> = vec![];
    bytes_to_hash.extend(&dk[16..32]);
    bytes_to_hash.extend(ciphertext.bytes());
    let mut hasher = Keccak256::new();
    hasher.input(&bytes_to_hash);
    let mac: &[u8] = &hasher.result();
    let mac = mac.to_hex();
    let new_account = accounts::Account::new(account_id.to_hyphenated().to_string(), &address, 3);
    new_account
        .with_cipher("aes-128-ctr".to_string())
        .with_ciphertext(ciphertext.to_string())
        .with_cipher_params(iv.to_hex())
        .with_kdf("pbkdf2".to_string())
        .with_pdkdf2_params(
            dk.len(),
            salt.to_hex().to_string(),
            "hmac-sha256".to_string(),
            count as usize,
        )
        .with_mac(mac.to_string())
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
