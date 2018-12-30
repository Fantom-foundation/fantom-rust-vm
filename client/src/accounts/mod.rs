//! Contains functionality related to dealing with Accounts

use std::{fmt, error::Error};
use std::fs::File;
use std::path::PathBuf;
use rand::Rng;
use std::collections::HashMap;
use secp256k1::key::{PublicKey};
use openssl::symm;
use rustc_serialize::hex::ToHex;

#[derive(Default, Debug, Serialize, Deserialize)]
/// Basic Account structure for Fantom system
pub struct Account {
    /// Public address that can be used to send coins to this account
    address: String,
    /// A unique ID for the account. This is different from the public address
    id: String,
    /// What version this account is
    version: usize,
    /// Struct that contains various crypto options for this account
    crypto: AccountCrypto,
}

impl Account {
    /// Creates and returns a new Account
    pub fn new(id: String, address: &str, version: usize) -> Account {
        let end = address.len();
        let start = end - 40;
        Account {
            address: address[start..end].to_string(),
            crypto: AccountCrypto::new(),
            id,
            version,
        }
    }

    /// Gets and returns the ID of the account
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    /// Part of the Builder, allows setting of the cipher algorithm
    pub fn with_cipher(mut self, cipher: String) -> Account {
        self.crypto.cipher = Some(cipher);
        self
    }

    /// Part of the Builder, allows setting of the cipher text
    pub fn with_ciphertext(mut self, cipher_text: String) -> Account {
        self.crypto.ciphertext = Some(cipher_text);
        self
    }

    /// Part of the Builder, allows setting various cipher parameters
    pub fn with_cipher_params(mut self, iv: String) -> Account {
        let mut new_cipher_params = HashMap::new();
        new_cipher_params.insert("iv".to_string(), iv);
        self.crypto.cipherparams = new_cipher_params;
        self
    }

    /// Part of the Builder, allows setting the KDF (Key Derivation Function)
    pub fn with_kdf(mut self, kdf: String) -> Account {
        self.crypto.kdf = Some(kdf);
        self
    }

    /// Part of the Builder, allows setting the MAC of the account
    pub fn with_mac(mut self, mac: String) -> Account {
        self.crypto.mac = Some(mac);
        self
    }

    /// Part of the Builder, allows setting the params when using pdkdf2
    pub fn with_pdkdf2_params(mut self, dklen: usize, salt: String, prf: String, c: usize) -> Account {
        self.crypto.kdfparams.dklen = Some(dklen);
        self.crypto.kdfparams.salt = Some(salt);
        self.crypto.kdfparams.prf = Some(prf);
        self.crypto.kdfparams.c = Some(c);
        self
    }

    /// Generates the ciphertext version of the secret key
    pub fn generate_cipher_text(generator: &mut rand::OsRng, secret_key: &secp256k1::key::SecretKey) -> (String, Vec<u8>) {
        // This section generates the ciphertext version of the secret key
        let cipher = symm::Cipher::aes_128_ctr();
        let mut key: Vec<u8> = vec![];
        let mut iv: Vec<u8> = vec![];
        for _ in 0..16 {
            key.push(generator.gen());
            iv.push(generator.gen());
        }
        // Takes the slice of data containing the secret key and encrypts it
        let data: &[u8] = &secret_key[0..secret_key.len()];
        let ciphertext = symm::encrypt(cipher, &key, Some(&iv), data).expect("Unable to encrypt secret key");
        // Return the hex and iv
        (ciphertext.to_hex(), iv)
    }

    /// Saves an account to a file in JSON format
    pub fn save(&self, base_dir: &str, filename: &str) -> Result<(), Box<Error>> {
        let path = std::path::PathBuf::from(base_dir.to_string() + filename);
        match File::create(path) {
            Ok(_) => {
                Ok(())
            },
            Err(e) => {
                return Err(Box::new(AccountError::new("There was an error saving")));
            }
        }
    }

    /// Gets the address of the account
    pub fn get_address(public_key: PublicKey) -> String {
        let context_flag = secp256k1::ContextFlag::Full;
        let context = secp256k1::Secp256k1::with_caps(context_flag);
        public_key.serialize_vec(&context, false).to_hex()
    }

    /// Gets the path to the keyfile for an account
    pub fn key_file_path(base_dir: &str, filename: &str) -> PathBuf {
        std::path::PathBuf::from(base_dir.to_string() + filename)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
/// Contains parameters specifically for cryptographic functions
pub struct AccountCrypto {
    // Name of the cipher algorithm
    cipher: Option<String>,
    // The ciphered text
    ciphertext: Option<String>,
    // The parameters given to the cipher algorithm
    cipherparams: HashMap<String, String>,
    // The key derivation function
    kdf: Option<String>,
    // Parameters for the KDF
    kdfparams: AccountKDFParams,
    // Mac for the cipher text
    mac: Option<String>,
}

impl AccountCrypto {
    /// Creates and returns a new AccountCrypto
    pub fn new() -> AccountCrypto {
        AccountCrypto {
            cipher: None,
            ciphertext: None,
            cipherparams: HashMap::new(),
            kdf: None,
            kdfparams: AccountKDFParams::new(),
            mac: None,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
/// Containers parameters specific to the key derivation functions
pub struct AccountKDFParams {
    dklen: Option<usize>,
    prf: Option<String>,
    c: Option<usize>,
    n: Option<usize>,
    p: Option<usize>,
    r: Option<usize>,
    salt: Option<String>,
}

impl AccountKDFParams {
    /// Create and return a new set of KDF parameters
    pub fn new() -> AccountKDFParams {
        AccountKDFParams {
            dklen: None,
            prf: None,
            c: None,
            n: None,
            p: None,
            r: None,
            salt: None,
        }
    }
}

#[derive(Debug)]
pub struct AccountError {
    details: String
}

impl AccountError {
    fn new(msg: &str) -> AccountError {
        AccountError{details: msg.to_string()}
    }
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.details)
    }
}

impl Error for AccountError {
    fn description(&self) -> &str {
        &self.details
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use keys;
    use rand::OsRng;
    use rand::Rng;
    #[test]
    fn create_kdf_params() {
        let test_params = AccountKDFParams::new();
        assert_eq!(test_params.dklen, None);
    }

    #[test]
    fn create_account_crypto() {
        let test_crypto = AccountCrypto::new();
        assert!(test_crypto.cipher.is_none());
    }

    #[test]
    fn create_account() {
        let mut generator = OsRng::new().expect("Unable to generate OsRng");
        let tmp_id = uuid::Uuid::new_v4();
        let (s, p) = keys::generate_random_keypair().unwrap();
        let (ciphertext, iv) = Account::generate_cipher_text(&mut generator, &s);
        let tmp_address = Account::get_address(p);
        let test_account = Account::new(tmp_id.to_string(), &tmp_address, 3);
        let test_account_json = serde_json::to_string(&test_account);
        assert!(test_account_json.is_ok());
    }
}