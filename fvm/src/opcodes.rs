type Gas = u32;

/// Opcodes supported by the Ethereum VM. https://github.com/trailofbits/evm-opcodes is a good reference for them.
#[derive(PartialEq, Hash, Debug)]
pub enum Opcode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SDIV,
    MOD,
    SMOD,
    LT,
    GT,
    SLT,
    SGT,
    EQ,
    ISZERO,
    AND,
    OR,
    XOR,
    NOT,
    BYTE,
    SLOAD,
    STORE,
    MLOAD,
    MSTORE,
    MSTORE8,
    MSIZE,
    PUSH1,
    UNKNOWN,
}

impl<'a> From<&'a u8> for Opcode {
    fn from(bytes: &u8) -> Self {
        match bytes {
            0x0 => Opcode::STOP,
            0x01 => Opcode::ADD,
            0x02 => Opcode::MUL,
            0x03 => Opcode::SUB,
            0x04 => Opcode::DIV,
            0x05 => Opcode::SDIV,
            0x06 => Opcode::MOD,
            0x07 => Opcode::SMOD,
            0x0a => Opcode::LT,
            0x0b => Opcode::GT,
            0x0c => Opcode::SLT,
            0x0d => Opcode::SGT,
            0x0e => Opcode::EQ,
            0x0f => Opcode::ISZERO,
            0x10 => Opcode::AND,
            0x11 => Opcode::OR,
            0x12 => Opcode::XOR,
            0x13 => Opcode::NOT,
            0x14 => Opcode::BYTE,
            0x54 => Opcode::SLOAD,
            0x55 => Opcode::STORE,
            0x51 => Opcode::MLOAD,
            0x52 => Opcode::MSTORE,
            0x53 => Opcode::MSTORE8,
            0x59 => Opcode::MSIZE,
            0x60 => Opcode::PUSH1,
            _ => Opcode::UNKNOWN,
        }
    }
}
