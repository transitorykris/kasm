use std::collections::HashMap;

use crate::errors::error;
use crate::errors::Error;
use crate::errors::ErrorCode;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
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

#[derive(Eq, PartialEq, Debug)]
pub enum Value {
    String(String),
    U8(u8),
    U16(u16),
    Null,
}

// TODO: Shoudl this be Option instead of Result?
pub fn str_to_mnemonic(instruction: &str) -> Result<Mnemonic, Error> {
    match instruction {
        "adc" => Ok(Mnemonic::ADC),
        "and" => Ok(Mnemonic::AND),
        "asl" => Ok(Mnemonic::ASL),
        "bbr0" => Ok(Mnemonic::BBR0),
        "bbr1" => Ok(Mnemonic::BBR1),
        "bbr2" => Ok(Mnemonic::BBR2),
        "bbr3" => Ok(Mnemonic::BBR3),
        "bbr4" => Ok(Mnemonic::BBR4),
        "bbr5" => Ok(Mnemonic::BBR5),
        "bbr6" => Ok(Mnemonic::BBR6),
        "bbr7" => Ok(Mnemonic::BBR7),
        "bbs0" => Ok(Mnemonic::BBS0),
        "bbs1" => Ok(Mnemonic::BBS1),
        "bbs2" => Ok(Mnemonic::BBS2),
        "bbs3" => Ok(Mnemonic::BBS3),
        "bbs4" => Ok(Mnemonic::BBS4),
        "bbs5" => Ok(Mnemonic::BBS5),
        "bbs6" => Ok(Mnemonic::BBS6),
        "bbs7" => Ok(Mnemonic::BBS7),
        "bcc" => Ok(Mnemonic::BCC),
        "bcs" => Ok(Mnemonic::BCS),
        "beq" => Ok(Mnemonic::BEQ),
        "bit" => Ok(Mnemonic::BIT),
        "bmi" => Ok(Mnemonic::BMI),
        "bne" => Ok(Mnemonic::BNE),
        "bpl" => Ok(Mnemonic::BPL),
        "bra" => Ok(Mnemonic::BRA),
        "brk" => Ok(Mnemonic::BRK),
        "bvc" => Ok(Mnemonic::BVC),
        "bvs" => Ok(Mnemonic::BVS),
        "clc" => Ok(Mnemonic::CLC),
        "cld" => Ok(Mnemonic::CLD),
        "cli" => Ok(Mnemonic::CLI),
        "clv" => Ok(Mnemonic::CLV),
        "cmp" => Ok(Mnemonic::CMP),
        "cpx" => Ok(Mnemonic::CPX),
        "cpy" => Ok(Mnemonic::CPY),
        "dec" => Ok(Mnemonic::DEC),
        "dex" => Ok(Mnemonic::DEX),
        "dey" => Ok(Mnemonic::DEY),
        "eor" => Ok(Mnemonic::EOR),
        "inc" => Ok(Mnemonic::INC),
        "inx" => Ok(Mnemonic::INX),
        "iny" => Ok(Mnemonic::INY),
        "jmp" => Ok(Mnemonic::JMP),
        "jsr" => Ok(Mnemonic::JSR),
        "lda" => Ok(Mnemonic::LDA),
        "ldx" => Ok(Mnemonic::LDX),
        "ldy" => Ok(Mnemonic::LDY),
        "lsr" => Ok(Mnemonic::LSR),
        "nop" => Ok(Mnemonic::NOP),
        "ora" => Ok(Mnemonic::ORA),
        "pha" => Ok(Mnemonic::PHA),
        "php" => Ok(Mnemonic::PHP),
        "phx" => Ok(Mnemonic::PHX),
        "phy" => Ok(Mnemonic::PHY),
        "pla" => Ok(Mnemonic::PLA),
        "plp" => Ok(Mnemonic::PLP),
        "plx" => Ok(Mnemonic::PLX),
        "ply" => Ok(Mnemonic::PLY),
        "rmb0" => Ok(Mnemonic::RMB0),
        "rmb1" => Ok(Mnemonic::RMB1),
        "rmb2" => Ok(Mnemonic::RMB2),
        "rmb3" => Ok(Mnemonic::RMB3),
        "rmb4" => Ok(Mnemonic::RMB4),
        "rmb5" => Ok(Mnemonic::RMB5),
        "rmb6" => Ok(Mnemonic::RMB6),
        "rmb7" => Ok(Mnemonic::RMB7),
        "rol" => Ok(Mnemonic::ROL),
        "ror" => Ok(Mnemonic::ROR),
        "rts" => Ok(Mnemonic::RTS),
        "sbc" => Ok(Mnemonic::SBC),
        "sec" => Ok(Mnemonic::SEC),
        "sed" => Ok(Mnemonic::SED),
        "sei" => Ok(Mnemonic::SEI),
        "smb0" => Ok(Mnemonic::SMB0),
        "smb1" => Ok(Mnemonic::SMB1),
        "smb2" => Ok(Mnemonic::SMB2),
        "smb3" => Ok(Mnemonic::SMB3),
        "smb4" => Ok(Mnemonic::SMB4),
        "smb5" => Ok(Mnemonic::SMB5),
        "smb6" => Ok(Mnemonic::SMB6),
        "smb7" => Ok(Mnemonic::SMB7),
        "sta" => Ok(Mnemonic::STA),
        "stp" => Ok(Mnemonic::STP),
        "stx" => Ok(Mnemonic::STX),
        "sty" => Ok(Mnemonic::STY),
        "stz" => Ok(Mnemonic::STZ),
        "tax" => Ok(Mnemonic::TAX),
        "tay" => Ok(Mnemonic::TAY),
        "trb" => Ok(Mnemonic::TRB),
        "tsb" => Ok(Mnemonic::TSB),
        "tsx" => Ok(Mnemonic::TSX),
        "txa" => Ok(Mnemonic::TXA),
        "txs" => Ok(Mnemonic::TXS),
        "tya" => Ok(Mnemonic::TYA),
        "wai" => Ok(Mnemonic::WAI),
        _ => Err(error(
            ErrorCode::UnknownInstruction,
            format!("Unknown instruction: {}", instruction),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::str_to_mnemonic;

    #[test]
    fn test_good_str_to_mnemonic() {
        if str_to_mnemonic("sta").is_err() {
            panic!("Expected sta to return Mnemonic::STA");
        };
    }

    #[test]
    fn test_bad_str_to_mnemonic() {
        if str_to_mnemonic("abc").is_ok() {
            panic!("Did not expect abc to return a valid mnemonic");
        };
    }
}
