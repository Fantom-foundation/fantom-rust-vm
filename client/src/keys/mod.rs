use rand::os::OsRng;
use secp256k1;
use secp256k1::key::{PublicKey, SecretKey};
use secp256k1::Error;

pub fn generate_random_keypair() -> Result<(SecretKey, PublicKey), Error> {
  let context_flag = secp256k1::ContextFlag::Full;
  let context = secp256k1::Secp256k1::with_caps(context_flag);
  let mut rng = OsRng::new().expect("OsRng");
  match context.generate_keypair(&mut rng) {
    Ok((secret_key, public_key)) => {
      Ok((secret_key, public_key))
    },
    Err(e) => {
      Err(e)
    }
  }
}