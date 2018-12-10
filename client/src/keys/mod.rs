use rand::os::OsRng;
use std::io::{stdin, stdout, Write};
use sodiumoxide::crypto::secretbox;

use secp256k1;
use secp256k1::key::{PublicKey, SecretKey};
use secp256k1::Error;

pub fn generate_random_keypair() -> Result<(SecretKey, PublicKey), Error> {
  let context_flag = secp256k1::ContextFlag::Full;
  let context = secp256k1::Secp256k1::with_caps(context_flag);
  let mut rng = OsRng::new().expect("OsRng");
  match context.generate_keypair(&mut rng) {
    Ok((secret_key, public_key)) => {
      // let passphrase = get_passphrase();
      // let mut secret_key_stripped = vec![];
      // for i in secret_key[..].iter().cloned() {
      //   secret_key_stripped.push(i);
      // }
      // println!("Passphrase as bytes: {:?}", passphrase.as_bytes());
      // let key = secretbox::xsalsa20poly1305::Key::from_slice(&passphrase.as_bytes()).unwrap();
      // println!("Key is: {:?}", passphrase);
      // let nonce = secretbox::gen_nonce();
      // let ciphertext = secretbox::seal(&secret_key_stripped, &nonce, &key);
      // debug!("Returning ciphertext");
      Ok((secret_key, public_key))
    }
    Err(e) => Err(e),
  }
}

fn get_passphrase() -> String {
  let mut passphrase = String::new();
  print!("Please enter a passphrase: ");
  let _ = stdout().flush();
  stdin()
    .read_line(&mut passphrase)
    .expect("Invalid passphrase!");
  String::from(passphrase.trim())
}
