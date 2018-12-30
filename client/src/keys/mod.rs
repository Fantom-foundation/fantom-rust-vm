use rand::os::OsRng;
use rpassword::read_password;
use std::{io, io::Write, io::BufRead, io::BufReader};
use secp256k1;
use secp256k1::key::{PublicKey, SecretKey};
use secp256k1::Error;
use std::{fmt, fs::File};
use std::string::ToString;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Wrapper type around a String to represent a password
pub struct Password(String);

impl fmt::Debug for Password {
    /// We do not want to potentially print out passwords in logs, so we implement `fmt` here
    /// such that it obfuscates it
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Password(******)")
    }
}

/// Converts a String into a Password
impl From<String> for Password {
    fn from(s: String) -> Password {
        Password(s)
    }
}

/// Converts a &str into a Password
impl<'a> From<&'a str> for Password {
    fn from(s: &'a str) -> Password {
        Password::from(String::from(s))
    }
}

/// Converts a Password back into a String
impl ToString for Password {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

/// So we can get the password as bytes or as a str
impl Password {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// This generates a random public/private keypair, and is used when creating a new account.
pub fn generate_random_keypair() -> Result<(SecretKey, PublicKey), Error> {
    let context_flag = secp256k1::ContextFlag::Full;
    let context = secp256k1::Secp256k1::with_caps(context_flag);
    let mut rng = OsRng::new().expect("OsRng");
    match context.generate_keypair(&mut rng) {
        Ok((secret_key, public_key)) => Ok((secret_key, public_key)),
        Err(e) => Err(e),
    }
}

/// Prompts the user for a passphrase. They will have to enter this to do anything with
/// their account.
pub fn get_passphrase() -> Result<Password, String> {
    const STDIN_ERROR: &str = "Unable to ask for password on non-interactive terminal.";
    print!("Enter password: ");
    io::stdout().flush().map_err(|_| "Error flushing stdout".to_owned())?;
    let password = read_password().map_err(|_| STDIN_ERROR.to_owned())?.into();
    print!("Repeat password: ");
    io::stdout().flush().map_err(|_| "Error flushing stdout".to_owned())?;
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
    passwords
        .get(0)
        .map(Password::clone)
        .ok_or_else(|| "Password file seems to be empty.".to_owned())
}

/// Reads passwords from files. Treats each line as a separate password.
pub fn passwords_from_files(files: &[String]) -> Result<Vec<Password>, String> {
    let passwords = files
        .iter()
        .map(|filename| {
            let file = File::open(filename).map_err(|_| {
                format!(
                    "{} Unable to read password file. Ensure it exists and permissions are correct.",
                    filename
                )
            })?;
            let reader = BufReader::new(&file);
            let lines = reader
                .lines()
                .filter_map(|l| l.ok())
                .map(|pwd| pwd.trim().to_owned().into())
                .collect::<Vec<Password>>();
            Ok(lines)
        })
        .collect::<Result<Vec<Vec<Password>>, String>>();
    Ok(passwords?.into_iter().flat_map(|x| x).collect())
}
