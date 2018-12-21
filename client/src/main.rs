#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate clap;
extern crate fvm;
extern crate secp256k1;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate openssl;
extern crate rand;
extern crate rpassword;
extern crate uuid;
#[macro_use]
extern crate serde_derive;
extern crate base64;
extern crate bigint;
extern crate block;
extern crate byteorder;
extern crate chrono;
extern crate ethash;
extern crate hmac;
extern crate mac;
extern crate pbkdf2;
extern crate rlp;
extern crate rustc_serialize;
extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate sha3;
extern crate world;

use std::process::exit;
use std::{fs, io};

use clap::App;
use rand::os::OsRng;
use rand::Rng;
use std::path::PathBuf;
use std::thread;

use std::io::prelude::*;

use hmac::Hmac;
use sha2::Sha256;
use sha3::{Digest, Keccak256};

use rustc_serialize::hex::ToHex;

pub mod accounts;
pub mod keys;
pub mod servers;

use std::fs::File;
use std::str;

type HmacSha256 = Hmac<Sha256>;

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
        if let Some(_) = account_matches.subcommand_matches("new") {
            debug!("Creating new account");
            // This is the passphrase we'll use to encrypt their secret key, and they will need to
            // provide to decrypt it
            let passphrase = keys::get_passphrase();
            // Generate a random public/private key
            match keys::generate_random_keypair() {
                Ok((secret_key, public_key)) => {
                    // Generate a random account ID
                    let account_id = uuid::Uuid::new_v4();
                    let mut generator = OsRng::new().expect("Unable to generate OsRng");
                    let (ciphertext, iv) = accounts::Account::generate_cipher_text(&mut generator, &secret_key);

                    let context_flag = secp256k1::ContextFlag::Full;
                    let context = secp256k1::Secp256k1::with_caps(context_flag);
                    let address = public_key.serialize_vec(&context, false).to_hex();
                    // This section generates a key from the passphrase so we don't have to keep the actual passphrase
                    // provided by the user

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
                    let mut new_account =
                        accounts::Account::new(account_id.to_hyphenated().to_string(), address.to_string(), 3);
                    new_account = new_account
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
                        .with_mac(mac.to_string());
                    let account_json = serde_json::to_string(&new_account).unwrap();
                    let now = chrono::Utc::now();
                    let filename = "UTC--".to_string()
                        + &now.format("%Y-%m-%d").to_string()
                        + "T"
                        + &now.format("%H-%M-%SZ").to_string()
                        + "--"
                        + &account_id.to_hyphenated().to_string()
                        + ".json";
                    let path = key_file_path(base_dir, &filename);
                    debug!("Path is: {:#?}", path);
                    if let Ok(mut fh) = File::create(path) {
                        match fh.write_all(account_json.as_bytes()) {
                            Ok(_) => info!("Keyfile written: {}", filename),
                            Err(e) => error!("Error writing keyfile: {:#?}", e),
                        }
                        exit(0);
                    } else {
                        error!("Unable to create keyfile: {}", filename);
                        exit(1);
                    }
                }
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
    create_chaindata_directory(path)?;
    Ok(())
}

fn create_data_directory(path: &str) -> Result<(), io::Error> {
    fs::create_dir_all(path.to_string() + "data")?;
    Ok(())
}

fn data_directory(path: &str) -> PathBuf {
    PathBuf::from(path.to_string() + "data")
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

fn create_chaindata_directory(path: &str) -> Result<(), io::Error> {
    fs::create_dir_all(path.to_string() + "chaindata")?;
    Ok(())
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

fn key_file_path(base_dir: &str, filename: &str) -> PathBuf {
    std::path::PathBuf::from(base_dir.to_string() + &filename)
}

fn chain_data_path(base_dir: &str) -> PathBuf {
    std::path::PathBuf::from(base_dir.to_string() + "chaindata")
}
