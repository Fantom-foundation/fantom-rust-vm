use rand::os::OsRng;
use std::io;
use std::io::Write;
use std::io::{BufReader, BufRead};

use std::fs::File;
use std::fmt;
use std::string::ToString;
use secp256k1;
use secp256k1::key::{PublicKey, SecretKey};
use secp256k1::Error;


#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Password(String);

impl fmt::Debug for Password {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Password(******)")
    }
}

impl From<String> for Password {
	fn from(s: String) -> Password {
		Password(s)
	}
}

impl<'a> From<&'a str> for Password {
	fn from(s: &'a str) -> Password {
		Password::from(String::from(s))
	}
}

impl ToString for Password {
	fn to_string(&self) -> String {
		String::from(self.0.clone())
	}
}

impl Password {
	pub fn as_bytes(&self) -> &[u8] {
		self.0.as_bytes()
	}

	pub fn as_str(&self) -> &str {
		self.0.as_str()
	}
}

pub fn generate_random_keypair() -> Result<(SecretKey, PublicKey), Error> {
  let context_flag = secp256k1::ContextFlag::Full;
  let context = secp256k1::Secp256k1::with_caps(context_flag);
  let mut rng = OsRng::new().expect("OsRng");
  match context.generate_keypair(&mut rng) {
    Ok((secret_key, public_key)) => {
      Ok((secret_key, public_key))
    }
    Err(e) => Err(e),
  }
}

pub fn get_passphrase() -> Result<Password, String> {
	use rpassword::read_password;
	const STDIN_ERROR: &'static str = "Unable to ask for password on non-interactive terminal.";
	print!("Enter passphrase: ");
	let _ = io::stdout().flush();

	let password = read_password().map_err(|_| STDIN_ERROR.to_owned())?.into();

	print!("Repeat password: ");
	let _ = io::stdout().flush();

	let password_repeat = read_password().map_err(|_| STDIN_ERROR.to_owned())?.into();

	if password != password_repeat {
		return Err("Passwords do not match!".into());
	}
	Ok(password)
}

/// Read a password from password file.
pub fn get_passphrase_file(path: String) -> Result<Password, String> {
	let passwords = passwords_from_files(&[path])?;
	// use only first password from the file
	passwords.get(0).map(Password::clone)
		.ok_or_else(|| "Password file seems to be empty.".to_owned())
}

/// Reads passwords from files. Treats each line as a separate password.
pub fn passwords_from_files(files: &[String]) -> Result<Vec<Password>, String> {
	let passwords = files.iter().map(|filename| {
		let file = File::open(filename).map_err(|_| format!("{} Unable to read password file. Ensure it exists and permissions are correct.", filename))?;
		let reader = BufReader::new(&file);
		let lines = reader.lines()
			.filter_map(|l| l.ok())
			.map(|pwd| pwd.trim().to_owned().into())
			.collect::<Vec<Password>>();
		Ok(lines)
	}).collect::<Result<Vec<Vec<Password>>, String>>();
	Ok(passwords?.into_iter().flat_map(|x| x).collect())
}