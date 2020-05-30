use std::collections::HashMap;

use crate::errors::error;
use crate::errors::Error;
use crate::errors::ErrorMsg;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
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
    #[allow(dead_code)]
    UnknownX,
    #[allow(dead_code)]
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
pub fn address_mode_length(address_mode: AddressMode) -> u16 {
    match address_mode {
        AddressMode::Absolute => 3,
        AddressMode::AbsoluteX => 3,
        AddressMode::AbsoluteY => 3,
        AddressMode::Immediate => 2,
        AddressMode::Implied => 1,
        AddressMode::Indirect => 3,
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn get_instruction(
    instruction_set: &InstructionMap,
    mnemonic: Mnemonic,
    address_mode: AddressMode,
) -> Result<u8, (Error, ErrorMsg)> {
    match instruction_set.get(&InstructionKey {
        mnemonic,
        address_mode,
    }) {
        Some(opcode) => *opcode,
        None => return Err(error(Error::NoValidOpcode, "No valid opcode".to_string())),
    };
    Ok(0)
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

#[derive(Eq, PartialEq)]
pub enum Value {
    String(String),
    U8(u8),
    U16(u16),
    Null,
}

pub fn str_to_mnemonic(instruction: String) -> Result<Mnemonic, (Error, ErrorMsg)> {
    match instruction.as_str() {
        "adc" => return Ok(Mnemonic::ADC),
        "and" => return Ok(Mnemonic::AND),
        "asl" => return Ok(Mnemonic::ASL),
        "bbr0" => return Ok(Mnemonic::BBR0),
        "bbr1" => return Ok(Mnemonic::BBR1),
        "bbr2" => return Ok(Mnemonic::BBR2),
        "bbr3" => return Ok(Mnemonic::BBR3),
        "bbr4" => return Ok(Mnemonic::BBR4),
        "bbr5" => return Ok(Mnemonic::BBR5),
        "bbr6" => return Ok(Mnemonic::BBR6),
        "bbr7" => return Ok(Mnemonic::BBR7),
        "bbs0" => return Ok(Mnemonic::BBS0),
        "bbs1" => return Ok(Mnemonic::BBS1),
        "bbs2" => return Ok(Mnemonic::BBS2),
        "bbs3" => return Ok(Mnemonic::BBS3),
        "bbs4" => return Ok(Mnemonic::BBS4),
        "bbs5" => return Ok(Mnemonic::BBS5),
        "bbs6" => return Ok(Mnemonic::BBS6),
        "bbs7" => return Ok(Mnemonic::BBS7),
        "bcc" => return Ok(Mnemonic::BCC),
        "bcs" => return Ok(Mnemonic::BCS),
        "beq" => return Ok(Mnemonic::BEQ),
        "bit" => return Ok(Mnemonic::BIT),
        "bmi" => return Ok(Mnemonic::BMI),
        "bne" => return Ok(Mnemonic::BNE),
        "bpl" => return Ok(Mnemonic::BPL),
        "bra" => return Ok(Mnemonic::BRA),
        "brk" => return Ok(Mnemonic::BRK),
        "bvc" => return Ok(Mnemonic::BVC),
        "bvs" => return Ok(Mnemonic::BVS),
        "clc" => return Ok(Mnemonic::CLC),
        "cld" => return Ok(Mnemonic::CLD),
        "cli" => return Ok(Mnemonic::CLI),
        "clv" => return Ok(Mnemonic::CLV),
        "cmp" => return Ok(Mnemonic::CMP),
        "cpx" => return Ok(Mnemonic::CPX),
        "cpy" => return Ok(Mnemonic::CPY),
        "dec" => return Ok(Mnemonic::DEC),
        "dex" => return Ok(Mnemonic::DEX),
        "dey" => return Ok(Mnemonic::DEY),
        "eor" => return Ok(Mnemonic::EOR),
        "inc" => return Ok(Mnemonic::INC),
        "inx" => return Ok(Mnemonic::INX),
        "iny" => return Ok(Mnemonic::INY),
        "jmp" => return Ok(Mnemonic::JMP),
        "jsr" => return Ok(Mnemonic::JSR),
        "lda" => return Ok(Mnemonic::LDA),
        "ldx" => return Ok(Mnemonic::LDX),
        "ldy" => return Ok(Mnemonic::LDY),
        "lsr" => return Ok(Mnemonic::LSR),
        "nop" => return Ok(Mnemonic::NOP),
        "ora" => return Ok(Mnemonic::ORA),
        "pha" => return Ok(Mnemonic::PHA),
        "php" => return Ok(Mnemonic::PHP),
        "phx" => return Ok(Mnemonic::PHX),
        "phy" => return Ok(Mnemonic::PHY),
        "pla" => return Ok(Mnemonic::PLA),
        "plp" => return Ok(Mnemonic::PLP),
        "plx" => return Ok(Mnemonic::PLX),
        "ply" => return Ok(Mnemonic::PLY),
        "rmb0" => return Ok(Mnemonic::RMB0),
        "rmb1" => return Ok(Mnemonic::RMB1),
        "rmb2" => return Ok(Mnemonic::RMB2),
        "rmb3" => return Ok(Mnemonic::RMB3),
        "rmb4" => return Ok(Mnemonic::RMB4),
        "rmb5" => return Ok(Mnemonic::RMB5),
        "rmb6" => return Ok(Mnemonic::RMB6),
        "rmb7" => return Ok(Mnemonic::RMB7),
        "rol" => return Ok(Mnemonic::ROL),
        "ror" => return Ok(Mnemonic::ROR),
        "rts" => return Ok(Mnemonic::RTS),
        "sbc" => return Ok(Mnemonic::SBC),
        "sec" => return Ok(Mnemonic::SEC),
        "sed" => return Ok(Mnemonic::SED),
        "sei" => return Ok(Mnemonic::SEI),
        "smb0" => return Ok(Mnemonic::SMB0),
        "smb1" => return Ok(Mnemonic::SMB1),
        "smb2" => return Ok(Mnemonic::SMB2),
        "smb3" => return Ok(Mnemonic::SMB3),
        "smb4" => return Ok(Mnemonic::SMB4),
        "smb5" => return Ok(Mnemonic::SMB5),
        "smb6" => return Ok(Mnemonic::SMB6),
        "smb7" => return Ok(Mnemonic::SMB7),
        "sta" => return Ok(Mnemonic::STA),
        "stp" => return Ok(Mnemonic::STP),
        "stx" => return Ok(Mnemonic::STX),
        "sty" => return Ok(Mnemonic::STY),
        "stz" => return Ok(Mnemonic::STZ),
        "tax" => return Ok(Mnemonic::TAX),
        "tay" => return Ok(Mnemonic::TAY),
        "trb" => return Ok(Mnemonic::TRB),
        "tsb" => return Ok(Mnemonic::TSB),
        "tsx" => return Ok(Mnemonic::TSX),
        "txa" => return Ok(Mnemonic::TXA),
        "txs" => return Ok(Mnemonic::TXS),
        "tya" => return Ok(Mnemonic::TYA),
        "wai" => return Ok(Mnemonic::WAI),
        _ => {
            return Err(error(
                Error::UnknownInstruction,
                format!("Unknown instruction: {}", instruction),
            ))
        }
    }
}
