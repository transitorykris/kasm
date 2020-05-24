use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum AddressMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied, // Note: doubling up Accumulator addressing here
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    Zeropage,
    ZeropageX,
    ZeropageY,
    Unknown, // Used in the first pass
    UnknownX,
    UnknownY,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Mnemonic {
    ADC,
    AND,
    ASL,
    BBR0,
    BBR1,
    BBR2,
    BBR3,
    BBR4,
    BBR5,
    BBR6,
    BBR7,
    BBS0,
    BBS1,
    BBS2,
    BBS3,
    BBS4,
    BBS5,
    BBS6,
    BBS7,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRA,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PHX,
    PHY,
    PLA,
    PLP,
    PLX,
    PLY,
    RMB0,
    RMB1,
    RMB2,
    RMB3,
    RMB4,
    RMB5,
    RMB6,
    RMB7,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    SMB0,
    SMB1,
    SMB2,
    SMB3,
    SMB4,
    SMB5,
    SMB6,
    SMB7,
    STA,
    STP,
    STX,
    STY,
    STZ,
    TAX,
    TAY,
    TRB,
    TSB,
    TSX,
    TXA,
    TXS,
    TYA,
    WAI,
}

#[derive(PartialEq, Eq, Hash)]
pub struct InstructionKey {
    pub mnemonic: Mnemonic,
    pub address_mode: AddressMode,
}

type Opcode = u8;
pub type InstructionMap = HashMap<InstructionKey, Opcode>;

// returns the length in bytes of instruction for the given the AddressMode
pub fn address_mode_length(address_mode: AddressMode) -> u8 {
    match address_mode {
        AddressMode::Absolute => 3,
        AddressMode::AbsoluteX => 3,
        AddressMode::AbsoluteY => 3,
        AddressMode::Immediate => 2,
        AddressMode::Implied => 1,
        AddressMode::Indirect => 2,
        AddressMode::IndirectX => 2,
        AddressMode::IndirectY => 2,
        AddressMode::Relative => 2,
        AddressMode::Zeropage => 2,
        AddressMode::ZeropageX => 2,
        AddressMode::ZeropageY => 2,
        AddressMode::Unknown => 0,
        AddressMode::UnknownX => 0,
        AddressMode::UnknownY => 0,
    }
}

pub fn address_mode_name(address_mode: AddressMode) -> String {
    match address_mode {
        AddressMode::Absolute => String::from("Absolute"),
        AddressMode::AbsoluteX => String::from("AbsoluteX"),
        AddressMode::AbsoluteY => String::from("AbsoluteY"),
        AddressMode::Immediate => String::from("Immediate"),
        AddressMode::Implied => String::from("Implied"),
        AddressMode::Indirect => String::from("Indirect"),
        AddressMode::IndirectX => String::from("IndirectX"),
        AddressMode::IndirectY => String::from("IndirectY"),
        AddressMode::Relative => String::from("Relative"),
        AddressMode::Zeropage => String::from("Zeropage"),
        AddressMode::ZeropageX => String::from("ZeropageX"),
        AddressMode::ZeropageY => String::from("ZeropageY"),
        AddressMode::Unknown => String::from("Unknown"),
        AddressMode::UnknownX => String::from("UnknownX"),
        AddressMode::UnknownY => String::from("UnknownY"),
    }
}

pub fn get_instruction(
    instruction_set: &InstructionMap,
    mnemonic: Mnemonic,
    address_mode: AddressMode,
) -> u8 {
    match instruction_set.get(&InstructionKey {
        mnemonic,
        address_mode,
    }) {
        Some(opcode) => *opcode,
        None => panic!("No valid opcode"),
    }
}

// Something will inevitably wrong in the below!
pub fn generate_instruction_set() -> InstructionMap {
    let mut instruction_set = InstructionMap::new();
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ADC,
            address_mode: AddressMode::Immediate,
        },
        0x69,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ADC,
            address_mode: AddressMode::Immediate,
        },
        0x69,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ADC,
            address_mode: AddressMode::Zeropage,
        },
        0x65,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ADC,
            address_mode: AddressMode::ZeropageX,
        },
        0x75,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ADC,
            address_mode: AddressMode::Absolute,
        },
        0x6d,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ADC,
            address_mode: AddressMode::AbsoluteX,
        },
        0x7d,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ADC,
            address_mode: AddressMode::AbsoluteY,
        },
        0x79,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ADC,
            address_mode: AddressMode::IndirectX,
        },
        0x61,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ADC,
            address_mode: AddressMode::IndirectY,
        },
        0x71,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::AND,
            address_mode: AddressMode::Immediate,
        },
        0x29,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::AND,
            address_mode: AddressMode::Zeropage,
        },
        0x25,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::AND,
            address_mode: AddressMode::ZeropageX,
        },
        0x35,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::AND,
            address_mode: AddressMode::Absolute,
        },
        0x2d,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::AND,
            address_mode: AddressMode::AbsoluteX,
        },
        0x3d,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::AND,
            address_mode: AddressMode::AbsoluteY,
        },
        0x39,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::AND,
            address_mode: AddressMode::IndirectX,
        },
        0x21,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::AND,
            address_mode: AddressMode::IndirectY,
        },
        0x31,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ASL,
            address_mode: AddressMode::Implied,
        },
        0x0a,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ASL,
            address_mode: AddressMode::Zeropage,
        },
        0x06,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ASL,
            address_mode: AddressMode::ZeropageX,
        },
        0x16,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ASL,
            address_mode: AddressMode::Absolute,
        },
        0x0e,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ASL,
            address_mode: AddressMode::AbsoluteX,
        },
        0x1e,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBR0,
            address_mode: AddressMode::Relative,
        },
        0x0f,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBR1,
            address_mode: AddressMode::Relative,
        },
        0x1f,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBR2,
            address_mode: AddressMode::Relative,
        },
        0x2f,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBR3,
            address_mode: AddressMode::Relative,
        },
        0x3f,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBR4,
            address_mode: AddressMode::Relative,
        },
        0x4f,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBR5,
            address_mode: AddressMode::Relative,
        },
        0x5f,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBR6,
            address_mode: AddressMode::Relative,
        },
        0x6f,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBR7,
            address_mode: AddressMode::Relative,
        },
        0x7f,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBS0,
            address_mode: AddressMode::Relative,
        },
        0x8f,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBS1,
            address_mode: AddressMode::Relative,
        },
        0x9f,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBS2,
            address_mode: AddressMode::Relative,
        },
        0xaf,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBS3,
            address_mode: AddressMode::Relative,
        },
        0xbf,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBS4,
            address_mode: AddressMode::Relative,
        },
        0xcf,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBS5,
            address_mode: AddressMode::Relative,
        },
        0xdf,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBS6,
            address_mode: AddressMode::Relative,
        },
        0xef,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BBS7,
            address_mode: AddressMode::Relative,
        },
        0xff,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BCC,
            address_mode: AddressMode::Relative,
        },
        0x90,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BCS,
            address_mode: AddressMode::Relative,
        },
        0xb0,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BEQ,
            address_mode: AddressMode::Relative,
        },
        0xf0,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BIT,
            address_mode: AddressMode::Absolute,
        },
        0x2c,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BIT,
            address_mode: AddressMode::Zeropage,
        },
        0x24,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BMI,
            address_mode: AddressMode::Relative,
        },
        0x30,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BNE,
            address_mode: AddressMode::Relative,
        },
        0xd0,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BPL,
            address_mode: AddressMode::Relative,
        },
        0x10,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BRA,
            address_mode: AddressMode::Relative,
        },
        0x80,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BRK,
            address_mode: AddressMode::Implied,
        },
        0x00,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BVC,
            address_mode: AddressMode::Relative,
        },
        0x50,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::BVS,
            address_mode: AddressMode::Relative,
        },
        0x70,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CLC,
            address_mode: AddressMode::Implied,
        },
        0x18,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CLD,
            address_mode: AddressMode::Implied,
        },
        0xd8,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CLI,
            address_mode: AddressMode::Implied,
        },
        0x58,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CLV,
            address_mode: AddressMode::Implied,
        },
        0xb8,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CMP,
            address_mode: AddressMode::Immediate,
        },
        0xc9,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CMP,
            address_mode: AddressMode::Zeropage,
        },
        0xc5,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CMP,
            address_mode: AddressMode::ZeropageX,
        },
        0xd5,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CMP,
            address_mode: AddressMode::Absolute,
        },
        0xcd,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CMP,
            address_mode: AddressMode::AbsoluteX,
        },
        0xdd,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CMP,
            address_mode: AddressMode::AbsoluteY,
        },
        0xd9,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CMP,
            address_mode: AddressMode::IndirectX,
        },
        0xc1,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CMP,
            address_mode: AddressMode::IndirectY,
        },
        0xd1,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CPX,
            address_mode: AddressMode::Implied,
        },
        0xe0,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CPX,
            address_mode: AddressMode::Zeropage,
        },
        0xe4,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CPX,
            address_mode: AddressMode::Absolute,
        },
        0xec,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CPY,
            address_mode: AddressMode::Implied,
        },
        0xc0,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CPY,
            address_mode: AddressMode::Zeropage,
        },
        0xc4,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::CPY,
            address_mode: AddressMode::Absolute,
        },
        0xcc,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::DEC,
            address_mode: AddressMode::Zeropage,
        },
        0xc6,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::DEC,
            address_mode: AddressMode::ZeropageX,
        },
        0xd6,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::DEC,
            address_mode: AddressMode::Absolute,
        },
        0xce,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::DEC,
            address_mode: AddressMode::AbsoluteX,
        },
        0xde,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::DEX,
            address_mode: AddressMode::Implied,
        },
        0xca,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::DEY,
            address_mode: AddressMode::Implied,
        },
        0x88,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::EOR,
            address_mode: AddressMode::Immediate,
        },
        0x49,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::EOR,
            address_mode: AddressMode::Zeropage,
        },
        0x45,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::EOR,
            address_mode: AddressMode::ZeropageX,
        },
        0x45,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::EOR,
            address_mode: AddressMode::Absolute,
        },
        0x4d,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::EOR,
            address_mode: AddressMode::AbsoluteX,
        },
        0x5d,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::EOR,
            address_mode: AddressMode::AbsoluteY,
        },
        0x59,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::EOR,
            address_mode: AddressMode::IndirectX,
        },
        0x41,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::EOR,
            address_mode: AddressMode::IndirectY,
        },
        0x51,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::INC,
            address_mode: AddressMode::Implied,
        },
        0x1a,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::INC,
            address_mode: AddressMode::Zeropage,
        },
        0xe6,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::INC,
            address_mode: AddressMode::ZeropageX,
        },
        0xf6,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::INC,
            address_mode: AddressMode::Absolute,
        },
        0xee,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::INC,
            address_mode: AddressMode::AbsoluteX,
        },
        0xfe,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::INX,
            address_mode: AddressMode::Implied,
        },
        0xe8,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::INY,
            address_mode: AddressMode::Implied,
        },
        0xc8,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::JMP,
            address_mode: AddressMode::Absolute,
        },
        0x4c,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::JMP,
            address_mode: AddressMode::Indirect,
        },
        0x6c,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::JSR,
            address_mode: AddressMode::Absolute,
        },
        0x20,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDA,
            address_mode: AddressMode::Immediate,
        },
        0xa9,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDA,
            address_mode: AddressMode::Zeropage,
        },
        0xa5,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDA,
            address_mode: AddressMode::ZeropageX,
        },
        0xb5,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDA,
            address_mode: AddressMode::Absolute,
        },
        0xad,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDA,
            address_mode: AddressMode::AbsoluteX,
        },
        0xbd,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDA,
            address_mode: AddressMode::AbsoluteY,
        },
        0xb9,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDA,
            address_mode: AddressMode::IndirectX,
        },
        0xa1,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDA,
            address_mode: AddressMode::IndirectY,
        },
        0xb1,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDX,
            address_mode: AddressMode::Immediate,
        },
        0xa2,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDX,
            address_mode: AddressMode::Zeropage,
        },
        0xa6,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDX,
            address_mode: AddressMode::ZeropageX,
        },
        0xb6,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDX,
            address_mode: AddressMode::Absolute,
        },
        0xae,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDX,
            address_mode: AddressMode::AbsoluteX,
        },
        0xbe,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDY,
            address_mode: AddressMode::Immediate,
        },
        0xa0,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDY,
            address_mode: AddressMode::Zeropage,
        },
        0xa4,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDY,
            address_mode: AddressMode::ZeropageX,
        },
        0xb4,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDY,
            address_mode: AddressMode::Absolute,
        },
        0xac,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LDY,
            address_mode: AddressMode::AbsoluteX,
        },
        0xbc,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LSR,
            address_mode: AddressMode::Immediate,
        },
        0x4a,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LSR,
            address_mode: AddressMode::Zeropage,
        },
        0x46,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LSR,
            address_mode: AddressMode::ZeropageX,
        },
        0x56,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LSR,
            address_mode: AddressMode::Absolute,
        },
        0x4e,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::LSR,
            address_mode: AddressMode::AbsoluteX,
        },
        0x5e,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::NOP,
            address_mode: AddressMode::Implied,
        },
        0xea,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ORA,
            address_mode: AddressMode::Immediate,
        },
        0x09,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ORA,
            address_mode: AddressMode::Zeropage,
        },
        0x05,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ORA,
            address_mode: AddressMode::ZeropageX,
        },
        0x15,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ORA,
            address_mode: AddressMode::Absolute,
        },
        0x0d,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ORA,
            address_mode: AddressMode::AbsoluteX,
        },
        0x1d,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ORA,
            address_mode: AddressMode::AbsoluteY,
        },
        0x19,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ORA,
            address_mode: AddressMode::IndirectX,
        },
        0x01,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ORA,
            address_mode: AddressMode::IndirectY,
        },
        0x11,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::PHA,
            address_mode: AddressMode::Implied,
        },
        0x48,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::PHP,
            address_mode: AddressMode::Implied,
        },
        0x08,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::PHX,
            address_mode: AddressMode::Implied,
        },
        0xda,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::PHY,
            address_mode: AddressMode::Implied,
        },
        0x5a,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::PLA,
            address_mode: AddressMode::Implied,
        },
        0x68,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::PLP,
            address_mode: AddressMode::Implied,
        },
        0x28,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::PLX,
            address_mode: AddressMode::Relative,
        },
        0xfa,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::PLY,
            address_mode: AddressMode::Relative,
        },
        0x7a,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::RMB0,
            address_mode: AddressMode::Zeropage,
        },
        0x07,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::RMB1,
            address_mode: AddressMode::Zeropage,
        },
        0x17,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::RMB2,
            address_mode: AddressMode::Zeropage,
        },
        0x27,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::RMB3,
            address_mode: AddressMode::Zeropage,
        },
        0x37,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::RMB4,
            address_mode: AddressMode::Zeropage,
        },
        0x47,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::RMB5,
            address_mode: AddressMode::Zeropage,
        },
        0x57,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::RMB6,
            address_mode: AddressMode::Zeropage,
        },
        0x67,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::RMB7,
            address_mode: AddressMode::Zeropage,
        },
        0x77,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ROL,
            address_mode: AddressMode::Implied,
        },
        0x2a,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ROL,
            address_mode: AddressMode::Zeropage,
        },
        0x26,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ROL,
            address_mode: AddressMode::ZeropageX,
        },
        0x36,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ROL,
            address_mode: AddressMode::Absolute,
        },
        0x2e,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ROL,
            address_mode: AddressMode::AbsoluteX,
        },
        0x3e,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ROR,
            address_mode: AddressMode::Implied,
        },
        0x6a,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ROR,
            address_mode: AddressMode::Zeropage,
        },
        0x66,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ROR,
            address_mode: AddressMode::ZeropageX,
        },
        0x76,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ROR,
            address_mode: AddressMode::Absolute,
        },
        0x6e,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::ROR,
            address_mode: AddressMode::AbsoluteX,
        },
        0x7e,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::RTI,
            address_mode: AddressMode::Implied,
        },
        0x40,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::RTS,
            address_mode: AddressMode::Implied,
        },
        0x60,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SBC,
            address_mode: AddressMode::Immediate,
        },
        0xe9,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SBC,
            address_mode: AddressMode::Zeropage,
        },
        0xe5,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SBC,
            address_mode: AddressMode::ZeropageX,
        },
        0xf5,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SBC,
            address_mode: AddressMode::Absolute,
        },
        0xed,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SBC,
            address_mode: AddressMode::AbsoluteX,
        },
        0xfd,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SBC,
            address_mode: AddressMode::AbsoluteY,
        },
        0xf9,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SBC,
            address_mode: AddressMode::IndirectX,
        },
        0xe1,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SBC,
            address_mode: AddressMode::IndirectY,
        },
        0xf1,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SEC,
            address_mode: AddressMode::Implied,
        },
        0x38,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SED,
            address_mode: AddressMode::Implied,
        },
        0xf8,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SEI,
            address_mode: AddressMode::Implied,
        },
        0x78,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SMB0,
            address_mode: AddressMode::Zeropage,
        },
        0x87,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SMB1,
            address_mode: AddressMode::Zeropage,
        },
        0x97,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SMB2,
            address_mode: AddressMode::Zeropage,
        },
        0xa7,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SMB3,
            address_mode: AddressMode::Zeropage,
        },
        0xb7,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SMB4,
            address_mode: AddressMode::Zeropage,
        },
        0xc7,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SMB5,
            address_mode: AddressMode::Zeropage,
        },
        0xd7,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SMB6,
            address_mode: AddressMode::Zeropage,
        },
        0xe7,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::SMB7,
            address_mode: AddressMode::Zeropage,
        },
        0xf7,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STA,
            address_mode: AddressMode::Zeropage,
        },
        0x85,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STA,
            address_mode: AddressMode::ZeropageX,
        },
        0x95,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STA,
            address_mode: AddressMode::Absolute,
        },
        0x8d,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STA,
            address_mode: AddressMode::AbsoluteX,
        },
        0x9d,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STA,
            address_mode: AddressMode::AbsoluteY,
        },
        0x99,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STA,
            address_mode: AddressMode::IndirectX,
        },
        0x81,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STA,
            address_mode: AddressMode::IndirectY,
        },
        0x91,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STP,
            address_mode: AddressMode::Implied,
        },
        0xdb,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STX,
            address_mode: AddressMode::Zeropage,
        },
        0x85,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STX,
            address_mode: AddressMode::ZeropageY,
        },
        0x95,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STX,
            address_mode: AddressMode::Absolute,
        },
        0x8d,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STY,
            address_mode: AddressMode::Zeropage,
        },
        0x85,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STY,
            address_mode: AddressMode::ZeropageX,
        },
        0x95,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STY,
            address_mode: AddressMode::Absolute,
        },
        0x8d,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STZ,
            address_mode: AddressMode::Zeropage,
        },
        0x64,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STZ,
            address_mode: AddressMode::ZeropageX,
        },
        0x94,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::STZ,
            address_mode: AddressMode::Absolute,
        },
        0x9c,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::TAX,
            address_mode: AddressMode::Implied,
        },
        0xaa,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::TAY,
            address_mode: AddressMode::Implied,
        },
        0xa8,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::TRB,
            address_mode: AddressMode::Absolute,
        },
        0x1c,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::TRB,
            address_mode: AddressMode::Zeropage,
        },
        0x14,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::TSB,
            address_mode: AddressMode::Absolute,
        },
        0x0c,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::TSB,
            address_mode: AddressMode::Zeropage,
        },
        0x04,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::TSX,
            address_mode: AddressMode::Implied,
        },
        0xba,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::TXA,
            address_mode: AddressMode::Implied,
        },
        0x8a,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::TXS,
            address_mode: AddressMode::Implied,
        },
        0x9a,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::TYA,
            address_mode: AddressMode::Implied,
        },
        0x98,
    );
    instruction_set.insert(
        InstructionKey {
            mnemonic: Mnemonic::WAI,
            address_mode: AddressMode::Relative,
        },
        0xcb,
    );
    instruction_set
}

pub enum Value {
    String(String),
    U8(u8),
    U16(u16),
    Null,
}

pub fn str_to_instruction(
    instruction: String,
    operand_type: AddressMode,
    value: Value,
) -> (Mnemonic, AddressMode, Value) {
    match instruction.as_str() {
        "adc" => return (Mnemonic::ADC, operand_type, value),
        "and" => return (Mnemonic::AND, operand_type, value),
        "asl" => return (Mnemonic::ASL, operand_type, value),
        "bbr0" => return (Mnemonic::BBR0, operand_type, value),
        "bbr1" => return (Mnemonic::BBR1, operand_type, value),
        "bbr2" => return (Mnemonic::BBR2, operand_type, value),
        "bbr3" => return (Mnemonic::BBR3, operand_type, value),
        "bbr4" => return (Mnemonic::BBR4, operand_type, value),
        "bbr5" => return (Mnemonic::BBR5, operand_type, value),
        "bbr6" => return (Mnemonic::BBR6, operand_type, value),
        "bbr7" => return (Mnemonic::BBR7, operand_type, value),
        "bbs0" => return (Mnemonic::BBS0, operand_type, value),
        "bbs1" => return (Mnemonic::BBS1, operand_type, value),
        "bbs2" => return (Mnemonic::BBS2, operand_type, value),
        "bbs3" => return (Mnemonic::BBS3, operand_type, value),
        "bbs4" => return (Mnemonic::BBS4, operand_type, value),
        "bbs5" => return (Mnemonic::BBS5, operand_type, value),
        "bbs6" => return (Mnemonic::BBS6, operand_type, value),
        "bbs7" => return (Mnemonic::BBS7, operand_type, value),
        "bcc" => return (Mnemonic::BCC, operand_type, value),
        "bcs" => return (Mnemonic::BCS, operand_type, value),
        "beq" => return (Mnemonic::BEQ, operand_type, value),
        "bit" => return (Mnemonic::BIT, operand_type, value),
        "bmi" => return (Mnemonic::BMI, operand_type, value),
        "bne" => return (Mnemonic::BNE, operand_type, value),
        "bpl" => return (Mnemonic::BPL, operand_type, value),
        "bra" => return (Mnemonic::BRA, operand_type, value),
        "brk" => return (Mnemonic::BRK, operand_type, value),
        "bvc" => return (Mnemonic::BVC, operand_type, value),
        "bvs" => return (Mnemonic::BVS, operand_type, value),
        "clc" => return (Mnemonic::CLC, operand_type, value),
        "cld" => return (Mnemonic::CLD, operand_type, value),
        "cli" => return (Mnemonic::CLI, operand_type, value),
        "clv" => return (Mnemonic::CLV, operand_type, value),
        "cmp" => return (Mnemonic::CMP, operand_type, value),
        "cpx" => return (Mnemonic::CPX, operand_type, value),
        "cpy" => return (Mnemonic::CPY, operand_type, value),
        "dec" => return (Mnemonic::DEC, operand_type, value),
        "dex" => return (Mnemonic::DEX, operand_type, value),
        "dey" => return (Mnemonic::DEY, operand_type, value),
        "eor" => return (Mnemonic::EOR, operand_type, value),
        "inc" => return (Mnemonic::INC, operand_type, value),
        "inx" => return (Mnemonic::INX, operand_type, value),
        "iny" => return (Mnemonic::INY, operand_type, value),
        "jmp" => return (Mnemonic::JMP, operand_type, value),
        "jsr" => return (Mnemonic::JSR, operand_type, value),
        "lda" => return (Mnemonic::LDA, operand_type, value),
        "ldx" => return (Mnemonic::LDX, operand_type, value),
        "ldy" => return (Mnemonic::LDY, operand_type, value),
        "lsr" => return (Mnemonic::LSR, operand_type, value),
        "nop" => return (Mnemonic::NOP, operand_type, value),
        "ora" => return (Mnemonic::ORA, operand_type, value),
        "pha" => return (Mnemonic::PHA, operand_type, value),
        "php" => return (Mnemonic::PHP, operand_type, value),
        "phx" => return (Mnemonic::PHX, operand_type, value),
        "phy" => return (Mnemonic::PHY, operand_type, value),
        "pla" => return (Mnemonic::PLA, operand_type, value),
        "plp" => return (Mnemonic::PLP, operand_type, value),
        "plx" => return (Mnemonic::PLX, operand_type, value),
        "ply" => return (Mnemonic::PLY, operand_type, value),
        "rmb0" => return (Mnemonic::RMB0, operand_type, value),
        "rmb1" => return (Mnemonic::RMB1, operand_type, value),
        "rmb2" => return (Mnemonic::RMB2, operand_type, value),
        "rmb3" => return (Mnemonic::RMB3, operand_type, value),
        "rmb4" => return (Mnemonic::RMB4, operand_type, value),
        "rmb5" => return (Mnemonic::RMB5, operand_type, value),
        "rmb6" => return (Mnemonic::RMB6, operand_type, value),
        "rmb7" => return (Mnemonic::RMB7, operand_type, value),
        "rol" => return (Mnemonic::ROL, operand_type, value),
        "ror" => return (Mnemonic::ROR, operand_type, value),
        "rts" => return (Mnemonic::RTS, operand_type, value),
        "sbc" => return (Mnemonic::SBC, operand_type, value),
        "sec" => return (Mnemonic::SEC, operand_type, value),
        "sed" => return (Mnemonic::SED, operand_type, value),
        "sei" => return (Mnemonic::SEI, operand_type, value),
        "smb0" => return (Mnemonic::SMB0, operand_type, value),
        "smb1" => return (Mnemonic::SMB1, operand_type, value),
        "smb2" => return (Mnemonic::SMB2, operand_type, value),
        "smb3" => return (Mnemonic::SMB3, operand_type, value),
        "smb4" => return (Mnemonic::SMB4, operand_type, value),
        "smb5" => return (Mnemonic::SMB5, operand_type, value),
        "smb6" => return (Mnemonic::SMB6, operand_type, value),
        "smb7" => return (Mnemonic::SMB7, operand_type, value),
        "sta" => return (Mnemonic::STA, operand_type, value),
        "stp" => return (Mnemonic::STP, operand_type, value),
        "stx" => return (Mnemonic::STX, operand_type, value),
        "sty" => return (Mnemonic::STY, operand_type, value),
        "stz" => return (Mnemonic::STZ, operand_type, value),
        "tax" => return (Mnemonic::TAX, operand_type, value),
        "tay" => return (Mnemonic::TAY, operand_type, value),
        "trb" => return (Mnemonic::TRB, operand_type, value),
        "tsb" => return (Mnemonic::TSB, operand_type, value),
        "tsx" => return (Mnemonic::TSX, operand_type, value),
        "txa" => return (Mnemonic::TXA, operand_type, value),
        "txs" => return (Mnemonic::TXS, operand_type, value),
        "tya" => return (Mnemonic::TYA, operand_type, value),
        "wai" => return (Mnemonic::WAI, operand_type, value),
        _ => panic!("Unknown instruction: {}", instruction),
    }
}
