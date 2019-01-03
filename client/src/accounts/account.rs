//! Contains functionality related to dealing with Accounts

use std::{fmt, error::Error};
use std::fs::File;
use std::path::PathBuf;
use rand::Rng;
use rand::OsRng;
use std::collections::HashMap;
use secp256k1;
use secp256k1::key::{PublicKey, SecretKey};
use openssl::symm;
use keys;
use uuid;
use rustc_serialize::hex::ToHex;
use hmac::Hmac;
use sha2::Sha256;
use sha3::{Digest, Keccak256};
use std::string::ToString;

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
    /// Contains the base data directory
    base_directory: PathBuf

}

impl Account {
    /// Creates and returns a new Account
    pub fn new(id: String, version: usize, base_directory: PathBuf) -> Result<Account, Box<Error>> {
        let secp = secp256k1::Secp256k1::with_caps(secp256k1::ContextFlag::Full);
        let mut hasher = Keccak256::new();
        let (p, s) = keys::generate_random_keypair()?;
        let public_vec = p.serialize_vec(&secp, false).to_vec();
        hasher.input(public_vec);
        let public_result = hasher.result().to_hex().to_string();
        let end = public_result.len();
        let start = end - 40;
        let address = public_result[start..end].to_string();

        Ok(Account {
            address: address,
            crypto: AccountCrypto::new(p, s),
            id,
            version,
            base_directory
        })
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
    pub fn generate_cipher_text(secret_key: &secp256k1::key::SecretKey) -> (String, Vec<u8>) {
        let mut generator = OsRng::new().unwrap();
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
                return Err(
                    Box::new(
                        AccountError::new(&format!("There was an error saving: {:?}", e))));
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

    /// Generates an ID, the ciphertext, and iv. 
    pub fn generate_ancillary_data(&mut self, public_key: PublicKey, secret_key: SecretKey) -> Result<Box<Account>, Box<Error>> {
        let mut generator = OsRng::new().expect("Unable to generate OsRng");
        self.id = uuid::Uuid::new_v4().to_hyphenated().to_string();
        let (ciphertext, iv) = Account::generate_cipher_text(&secret_key);
        let address = Account::get_address(public_key);
        Account::account_from_passphrase(&mut self.id.as_bytes(), &iv.to_hex(), &ciphertext, &address)
    }

    pub fn get_account_filename(&self) -> String {
        let now = chrono::Utc::now();
        "UTC--".to_string()
            + &now.format("%Y-%m-%d").to_string()
            + "T"
            + &now.format("%H-%M-%SZ").to_string()
            + "--"
            + &self.get_id()
            + ".json"
    }


    pub fn account_from_passphrase(iv: &[u8], ciphertext: &str, address: &str, base_directory: &str) -> Result<Box<Account>, Box<Error>> {
        // This is the passphrase we'll use to encrypt their secret key, and they will need to
        // provide to decrypt it
        let mut generator = match OsRng::new() {
            Ok(g) => { g },
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        let passphrase = match keys::get_passphrase() {
            Ok(passphrase) => { passphrase },
            Err(e) => { 
                return Err(e.into()); 
            }
        };

        let count: u32 = 64000 + rand::thread_rng().gen_range(0, 20000);
        let salt: Vec<u8> = generator.gen_iter::<u8>().take(16).collect();

        // Pre-allocate an array to hold the derived key
        let mut dk = [0u8; 32];
        // Use the KDF to create a hashed string and put it into `dk`
        pbkdf2::pbkdf2::<Hmac<Sha256>>(&passphrase.as_bytes(), &salt, count as usize, &mut dk);

        // Now we need the MAC. This verifies authenticity of the signature.
        let mut bytes_to_hash: Vec<u8> = vec![];
        bytes_to_hash.extend(&dk[16..32]);
        bytes_to_hash.extend(ciphertext.bytes());
        let mut hasher = Keccak256::new();
        hasher.input(&bytes_to_hash);
        let mac = hasher.result();
        let mac = mac.to_hex();
        let id = uuid::Uuid::new_v4().to_hyphenated().to_string();
        let bd: PathBuf = base_directory.into();
        let new_account = Account::new(id, 3, bd)?;

        Ok(Box::new(new_account
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
            .with_mac(mac.to_string())))
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
    // Public key
    public_key: Vec<u8>,
    // Secret key
    secret_key: Vec<u8>
}

impl AccountCrypto {
    /// Creates and returns a new AccountCrypto
    pub fn new(public_key: PublicKey, secret_key: SecretKey) -> AccountCrypto {
        let secp = secp256k1::Secp256k1::with_caps(secp256k1::ContextFlag::Full);
        let mut secret_bytes = vec![];
        for b in secret_key[..].iter().cloned() {
            secret_bytes.push(b);
        }
        let public_bytes = public_key.serialize_vec(&secp, false).to_vec();

        AccountCrypto {
            cipher: None,
            ciphertext: None,
            cipherparams: HashMap::new(),
            kdf: None,
            kdfparams: AccountKDFParams::new(),
            mac: None,
            public_key: public_bytes,
            secret_key: secret_bytes
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
            salt: None
        }
    }
}

#[derive(Debug)]
/// Struct for Account-specific errors
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
    use rustc_serialize::hex::*;
    use fs;

    fn get_test_secret_public() -> (secp256k1::key::SecretKey, secp256k1::key::PublicKey) {
        let secp = secp256k1::Secp256k1::with_caps(secp256k1::ContextFlag::Full);
        let secret = "bb556511a3f9a71939b6e5ef0834ac606a6b87966a90fb5becd47b3603e2c4cc".from_hex().unwrap();
        let secret = secp256k1::key::SecretKey::from_slice(&secp, &secret).unwrap();
        let public = secp256k1::key::PublicKey::from_secret_key(&secp, &secret).unwrap();
        (secret, public)
    }

    fn get_test_uuid() -> String {
        "527965ec-2091-45c5-ab06-ecb2e54a56bb".to_string()
    }

    fn get_test_data_directory() -> PathBuf {
        let path = "/tmp/fantom-test/".to_string();
        const DIRECTORIES: [&'static str; 6] = ["data", "raft", "eth", "lachesis", "keys", "chaindata"];
        for end in &DIRECTORIES {
            let _ = fs::create_dir_all(path.clone() + end);
        }
        PathBuf::from(path)
    }

    #[test]
    fn create_kdf_params() {
        let test_params = AccountKDFParams::new();
        assert_eq!(test_params.dklen, None);
    }

    #[test]
    fn create_account_crypto() {
        let (p, s) = get_test_secret_public();
        let test_crypto = AccountCrypto::new(s, p);
        assert!(test_crypto.cipher.is_none());
    }

    #[test]
    fn create_account() {
        let tmp_id = uuid::Uuid::new_v4().to_string();
        let base_path = get_test_data_directory();
        let test_account = Account::new(tmp_id.to_string(), 3, base_path).unwrap();
        let test_account_json = serde_json::to_string(&test_account);
        assert!(test_account_json.is_ok());
    }
}