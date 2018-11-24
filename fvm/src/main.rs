#[macro_use]
extern crate error_chain;

extern crate bigint;
extern crate trie;

#[macro_use] extern crate log;
extern crate env_logger;

mod memory;
mod account;
mod opcodes;
mod storage;
mod vm;
mod errors;

fn main() {
    env_logger::init();
    println!("Hello!");
}