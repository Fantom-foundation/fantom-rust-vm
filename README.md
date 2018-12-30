# Fantom VM

This is a register-based virtual machine intended to execute transactions for the Fantom cryptocurrency. In its current state, it can function as an Ethereum VM and full node.

## Consensus Algorithms

By default, the FVM utilizes the ethash consensus algorithm used by Ethereum. The goal is for it to use the Lachesis protocol.

## Structure

This project is structured as a Cargo workspace, with three sub-projects: `client`, `fvm`, and `world`.

### Client

This provides the CLI binary, and is responsible for setting up the various servers over which the FVM will accept requests. It also contains utilities for managing keys and accounts.

#### CLI Options

These are defined in `client/src/cli.yml`, and uses the Rust `clap` library.

### FVM

This contains the Fantom Virtual Machine. It implements the Ethereum Virtual Machine, but emulates registers rather than a stack. Any valid Solidity bytecode should run on the FVM.