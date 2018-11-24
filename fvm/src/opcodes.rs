type Gas = u32;

/// Opcodes supported by the Ethereum VM. https://github.com/trailofbits/evm-opcodes is a good reference for them.
#[derive(PartialEq, Hash, Debug)]
pub enum Opcode {
    STOP,
    ADD,
    MUL,
    SUB,
    DIV,
    SLOAD,
    STORE,
    MLOAD,
    MSTORE,
    MSTORE8,
    MSIZE,
    PUSH1,
    UNKNOWN
}

impl<'a> From<&'a u8> for Opcode {
  fn from(bytes: &u8) -> Self {
    match bytes {
      0x0 => {
        Opcode::STOP
      },
      0x01 => {
        Opcode::ADD
      },
      0x02 => {
        Opcode::MUL
      },
      0x03 => {
        Opcode::SUB
      },
      0x04 => {
        Opcode::DIV
      },
      0x54 => {
        Opcode::SLOAD
      },
      0x55 => {
        Opcode::STORE
      },
      0x51 => {
        Opcode::MLOAD
      },
      0x52 => {
        Opcode::MSTORE
      },
      0x53 => {
        Opcode::MSTORE8
      },
      0x59 => {
        Opcode::MSIZE
      },
      0x60 => {
        Opcode::PUSH1
      }
      _ => {
        Opcode::UNKNOWN
      }
    }
  }
}