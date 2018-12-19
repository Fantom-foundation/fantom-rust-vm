use opcodes::Opcode;

pub fn get_cost(op: Opcode) -> Option<usize> {
    match op {
        Opcode::STOP => Some(0),
        Opcode::JUMPDEST => Some(1),

        Opcode::ADDRESS
        | Opcode::ORIGIN
        | Opcode::CALLER
        | Opcode::CALLVALUE
        | Opcode::CALLDATASIZE
        | Opcode::CODESIZE
        | Opcode::GASPRICE
        | Opcode::RETURNDATASIZE
        | Opcode::COINBASE
        | Opcode::TIMESTAMP
        | Opcode::NUMBER
        | Opcode::DIFFICULTY
        | Opcode::GASLIMIT
        | Opcode::POP
        | Opcode::PC
        | Opcode::MSIZE
        | Opcode::GAS => Some(2),

        Opcode::LT
        | Opcode::GT
        | Opcode::SLT
        | Opcode::SGT
        | Opcode::EQ
        | Opcode::ISZERO
        | Opcode::AND
        | Opcode::OR
        | Opcode::XOR
        | Opcode::NOT
        | Opcode::BYTE
        | Opcode::ADD
        | Opcode::SUB
        | Opcode::PUSH(_)
        | Opcode::DUP(_)
        | Opcode::SWAP(_) => Some(3),

        Opcode::MUL | Opcode::DIV | Opcode::SDIV | Opcode::MOD | Opcode::SMOD | Opcode::SIGNEXTEND => Some(5),
        Opcode::ADDMOD | Opcode::MULMOD => Some(8),
        Opcode::EXP => Some(10),
        Opcode::BLOCKHASH => Some(20),
        Opcode::SHA3 => Some(30),

        Opcode::BALANCE => Some(400),
        Opcode::LOG(0) => Some(375),
        Opcode::LOG(1) => Some(750),
        Opcode::LOG(2) => Some(1125),
        Opcode::LOG(3) => Some(1500),
        Opcode::LOG(4) => Some(1875),

        _ => None,
    }
}
