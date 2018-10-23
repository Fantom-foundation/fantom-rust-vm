type Gas = u32;

/// Opcodes supported by the Ethereum VM. https://github.com/trailofbits/evm-opcodes is a good reference for them.
#[derive(PartialEq, Hash)]
pub enum Opcode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV
}

