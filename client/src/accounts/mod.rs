use rand::Rng;
use std::collections::HashMap;

use openssl::symm;
use rustc_serialize::hex::ToHex;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Account {
    address: String,
    id: String,
    version: usize,
    crypto: AccountCrypto,
}

impl Account {
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

    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    pub fn with_cipher(mut self, cipher: String) -> Account {
        self.crypto.cipher = Some(cipher);
        self
    }

    pub fn with_ciphertext(mut self, cipher_text: String) -> Account {
        self.crypto.ciphertext = Some(cipher_text);
        self
    }

    pub fn with_cipher_params(mut self, iv: String) -> Account {
        let mut new_cipher_params = HashMap::new();
        new_cipher_params.insert("iv".to_string(), iv);
        self.crypto.cipherparams = new_cipher_params;
        self
    }

    pub fn with_kdf(mut self, kdf: String) -> Account {
        self.crypto.kdf = Some(kdf);
        self
    }

    pub fn with_mac(mut self, mac: String) -> Account {
        self.crypto.mac = Some(mac);
        self
    }

    pub fn with_pdkdf2_params(mut self, dklen: usize, salt: String, prf: String, c: usize) -> Account {
        self.crypto.kdfparams.dklen = Some(dklen);
        self.crypto.kdfparams.salt = Some(salt);
        self.crypto.kdfparams.prf = Some(prf);
        self.crypto.kdfparams.c = Some(c);
        self
    }

    pub fn generate_cipher_text(
        generator: &mut rand::OsRng,
        secret_key: &secp256k1::key::SecretKey,
    ) -> (String, Vec<u8>) {
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
        let ciphertext = symm::encrypt(cipher, &key, Some(&iv), data).expect("Unable to encrypt secret key");
        (ciphertext.to_hex(), iv)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct AccountCrypto {
    cipher: Option<String>,
    ciphertext: Option<String>,
    cipherparams: HashMap<String, String>,
    kdf: Option<String>,
    kdfparams: AccountKDFParams,
    mac: Option<String>,
}

impl AccountCrypto {
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
