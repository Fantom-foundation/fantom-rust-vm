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
extern crate rustc_serialize;
//extern crate pbkdf2;
extern crate hmac;
extern crate sha2;
extern crate sha3;
extern crate byteorder;
extern crate base64;

use std::{fs, io};
use std::process::exit;

use clap::App;
use rand::os::OsRng;
use rand::Rng;
use std::path::PathBuf;
use byteorder::{ByteOrder, BigEndian};

use hmac::Hmac;
use hmac::Mac;
use sha2::{Sha256};
use sha3::{Digest, Keccak256};

use rustc_serialize::hex::ToHex;

pub mod servers;
pub mod keys;
pub mod accounts;

use openssl::symm;

use std::str;

type HmacSha256 = Hmac<Sha256>;

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
                    
                    // This section generates the ciphertext version of the secret key
                    let cipher = symm::Cipher::aes_128_ctr();
                    let mut key: Vec<u8> = vec![];
                    let mut iv: Vec<u8> = vec![];
                    for _ in 0..16 {
                        key.push(generator.gen());
                    }
                    for _ in 0..16 {
                        iv.push(generator.gen());
                    }

                    let data: &[u8] = &secret_key[0..secret_key.len()];
                    let ciphertext = symm::encrypt(
                        cipher,
                        &key,
                        Some(&iv),
                        data).expect("Unable to encrypt secret key");
                    let ciphertext = ciphertext.to_hex();
                    
                    let context_flag = secp256k1::ContextFlag::Full;
                    let context = secp256k1::Secp256k1::with_caps(context_flag);
                    let address = public_key.serialize_vec(&context, false).to_hex();
                    // This section generates a key from the passphrase so we don't have to keep the actual passphrase
                    // provided by the user

                    // How many hashing iterations to do
                    
                    let count: u32 = 64000 + rand::thread_rng().gen_range(0, 20000);
                    debug!("Hashing salt {:?} times...", count);
                    let mut result: Vec<u8> = vec![];
                    let mut dk: Vec<u8> = vec![];
                    let mut salt: Vec<u8> = vec![];
                    debug!("Creating salt");
                    for _ in 0..16 {
                        salt.push(generator.gen());
                    }
                    debug!("Generated Salt: {:?}", salt);
                    let passphrase = passphrase.expect("Unable to get passphrase");
                    debug!("Deriving passphrase from {:?}", passphrase);
                    pbkdf2::pbkdf2::<Hmac<Sha256>>(&passphrase.as_bytes(), &salt[..], count as usize, &mut dk);
                    debug!("KDF result is: {:#?}", result);
                    let mut bytes_to_hash: Vec<u8> = vec![];
                    for i in &result[16..32] {
                        bytes_to_hash.push(*i);
                    }
                    let mut result = "$rpbkdf2$0$".to_string();
                    let mut tmp = [0u8; 4];
                    debug!("4 slot allocated");
                    BigEndian::write_u32(&mut tmp, count);
                    result.push_str(&base64::encode(&tmp));
                    result.push('$');
                    result.push_str(&base64::encode(&salt));
                    result.push('$');
                    result.push_str(&base64::encode(&dk));
                    result.push('$');
                    
                    bytes_to_hash.extend(ciphertext.bytes());
                    let mut hasher = Keccak256::new();
                    hasher.input(&bytes_to_hash);
                    let mac: &[u8] = &hasher.result();
                    let mac_string = str::from_utf8(mac).expect("Cannot convert mac to string");
                    println!("Account ID: {:?}", account_id.to_hyphenated().to_string());
                    println!("Public Address: {:?}", public_key);
                    println!("Secret Key: {:?}", result);
                    println!("Passphrase: {:?}", passphrase);
                    println!("MAC is: {:?}", mac);
                    let new_account = accounts::Account::new(account_id.to_hyphenated().to_string(), address.to_string(), 3).with_cipher("aes-128-ctr".to_string())
                    .with_ciphertext(ciphertext.to_string())
                    .with_cipher_params(iv.to_hex())
                    .with_kdf("pbkdf".to_string())
                    .with_mac(mac_string.to_string());

                    println!("New account is: {:#?}", new_account);
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

fn data_directory(path: &str) -> PathBuf {
    PathBuf::from(path.to_string()+"data")
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