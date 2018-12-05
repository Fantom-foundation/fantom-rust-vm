#[macro_use]
extern crate clap;
extern crate fvm;
extern crate world;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::exit;

use clap::App;
use fvm::vm::VM;

pub fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("transaction-test") {
        if matches.is_present("INPUT") {
            let filename = matches.value_of("INPUT").unwrap();
            let bytecode = read_bytecode(filename);
        } else {
            println!("Please specify the file containing the EVM bytecode");
            exit(1);
        }
    }
}

fn read_bytecode(filename: &str) -> std::io::Result<()> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(())
}

fn create_transaction_vm(bytecode: String) -> fvm::vm::VM {
    VM::new(bytecode.into_bytes())
}

#[cfg(test)]
mod tests {
    use world::db;

}
