//! Handles the CLI commands related to accounts

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process::exit;

use clap;

use super::account::Account;
use keys;

/// Routes the account-related CLI command to the proper function
pub fn handle_cli_command(account_matches: &clap::ArgMatches, base_dir: &str) {
    if account_matches.subcommand_matches("new").is_some() {
        handle_create_new_account(base_dir);
    }
}

// Handles creating a new account
fn handle_create_new_account(base_dir: &str) {
    // This handles the user wanting to create a new account
    debug!("Creating new account");
    // Generate a random public/private key
    match keys::generate_random_keypair() {
        Ok((public_key, secret_key)) => {
            let (ct, iv) = Account::generate_cipher_text(&secret_key);
            let address = Account::get_address(public_key);
            let new_account = match Account::account_from_passphrase(&iv, &ct, &address, base_dir) {
                Ok(account) => account,
                Err(e) => {
                    error!("There was an error generating a new account: {:?}", e);
                    exit(1);
                }
            };
            let filename = new_account.get_account_filename();
            let path = PathBuf::from(base_dir.to_string() + "keys");
            let account_json = serde_json::to_string(&new_account).unwrap();
            match File::create(path) {
                Ok(mut fh) => match fh.write_all(account_json.as_bytes()) {
                    Ok(_) => {
                        info!("Keyfile written: {}", filename);
                        exit(0);
                    }
                    Err(e) => {
                        error!("Error writing keyfile: {:#?}", e);
                        exit(1);
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
