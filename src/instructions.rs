use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
pub enum AddressMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied,    // Note: doubling up Accumulator addressing here
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    Zeropage,
    ZeropageX,
    ZeropageY,
    Unknown,    // Used in the first pass
    UnknownX,
    UnknownY,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Mnemonic {
    ADC, AND, ASL, BBR0, BBR1, BBR2, BBR3, BBR4, BBR5, BBR6, BBR7,
    BBS0, BBS1, BBS2, BBS3, BBS4, BBS5, BBS6, BBS7, BCC, BCS, BEQ,
    BIT, BMI, BNE, BPL, BRA, BRK, BVC, BVS, CLC, CLD, CLI, CLV, CMP,
    CPX, CPY, DEC, DEX, DEY, EOR, INC, INX, INY, JMP, JSR, LDA, LDX,
    LDY, LSR, NOP, ORA, PHA, PHP, PHX, PHY, PLA, PLP, PLX, PLY,
    RMB0, RMB1, RMB2, RMB3, RMB4, RMB5, RMB6, RMB7, ROL, ROR, RTI,
    RTS, SBC, SEC, SED, SEI, SMB0, SMB1, SMB2, SMB3, SMB4, SMB5,
    SMB6, SMB7, STA, STP, STX, STY, STZ, TAX, TAY, TRB, TSB, TSX,
    TXA, TXS, TYA, WAI,
}

#[derive(PartialEq, Eq, Hash)]
pub struct InstructionKey{
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

fn add_instruction(instruction_set: &mut InstructionMap, mnemonic: Mnemonic,
        address_mode: AddressMode, opcode: u8) {
    instruction_set.insert(InstructionKey{mnemonic, address_mode}, opcode);
}

pub fn get_instruction(instruction_set: &InstructionMap, mnemonic: Mnemonic,
        address_mode: AddressMode) -> u8 {
    match instruction_set.get(&InstructionKey{mnemonic, address_mode}) {
        Some(opcode) => *opcode,
        None => panic!("No valid opcode"),
    }
}

// Something will inevitably wrong in the below!
pub fn generate_instruction_set() -> InstructionMap {
    let mut instruction_set = InstructionMap::new();
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::Immediate, 0x69);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::Zeropage, 0x65);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::ZeropageX, 0x75);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::Absolute, 0x6d);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::AbsoluteX, 0x7d);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::AbsoluteY, 0x79);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::IndirectX, 0x61);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::IndirectY, 0x71);
    add_instruction(&mut instruction_set, Mnemonic::AND, AddressMode::Immediate, 0x29);
    add_instruction(&mut instruction_set, Mnemonic::AND, AddressMode::Zeropage, 0x25);
    add_instruction(&mut instruction_set, Mnemonic::AND, AddressMode::ZeropageX, 0x35);
    add_instruction(&mut instruction_set, Mnemonic::AND, AddressMode::Absolute, 0x2d);
    add_instruction(&mut instruction_set, Mnemonic::AND, AddressMode::AbsoluteX, 0x3d);
    add_instruction(&mut instruction_set, Mnemonic::AND, AddressMode::AbsoluteY, 0x39);
    add_instruction(&mut instruction_set, Mnemonic::AND, AddressMode::IndirectX, 0x21);
    add_instruction(&mut instruction_set, Mnemonic::AND, AddressMode::IndirectY, 0x31);
    add_instruction(&mut instruction_set, Mnemonic::ASL, AddressMode::Implied, 0x0a);
    add_instruction(&mut instruction_set, Mnemonic::ASL, AddressMode::Zeropage, 0x06);
    add_instruction(&mut instruction_set, Mnemonic::ASL, AddressMode::ZeropageX, 0x16);
    add_instruction(&mut instruction_set, Mnemonic::ASL, AddressMode::Absolute, 0x0e);
    add_instruction(&mut instruction_set, Mnemonic::ASL, AddressMode::AbsoluteX, 0x1e);
    add_instruction(&mut instruction_set, Mnemonic::BBR0, AddressMode::Relative, 0x0f);
    add_instruction(&mut instruction_set, Mnemonic::BBR1, AddressMode::Relative, 0x1f);
    add_instruction(&mut instruction_set, Mnemonic::BBR2, AddressMode::Relative, 0x2f);
    add_instruction(&mut instruction_set, Mnemonic::BBR3, AddressMode::Relative, 0x3f);
    add_instruction(&mut instruction_set, Mnemonic::BBR4, AddressMode::Relative, 0x4f);
    add_instruction(&mut instruction_set, Mnemonic::BBR5, AddressMode::Relative, 0x5f);
    add_instruction(&mut instruction_set, Mnemonic::BBR6, AddressMode::Relative, 0x6f);
    add_instruction(&mut instruction_set, Mnemonic::BBR7, AddressMode::Relative, 0x7f);
    add_instruction(&mut instruction_set, Mnemonic::BBS0, AddressMode::Relative, 0x8f);
    add_instruction(&mut instruction_set, Mnemonic::BBS1, AddressMode::Relative, 0x9f);
    add_instruction(&mut instruction_set, Mnemonic::BBS2, AddressMode::Relative, 0xaf);
    add_instruction(&mut instruction_set, Mnemonic::BBS3, AddressMode::Relative, 0xbf);
    add_instruction(&mut instruction_set, Mnemonic::BBS4, AddressMode::Relative, 0xcf);
    add_instruction(&mut instruction_set, Mnemonic::BBS5, AddressMode::Relative, 0xdf);
    add_instruction(&mut instruction_set, Mnemonic::BBS6, AddressMode::Relative, 0xef);
    add_instruction(&mut instruction_set, Mnemonic::BBS7, AddressMode::Relative, 0xff);
    add_instruction(&mut instruction_set, Mnemonic::BCC, AddressMode::Relative, 0x90);
    add_instruction(&mut instruction_set, Mnemonic::BCS, AddressMode::Relative, 0xb0);
    add_instruction(&mut instruction_set, Mnemonic::BEQ, AddressMode::Relative, 0xf0);
    add_instruction(&mut instruction_set, Mnemonic::BIT, AddressMode::Absolute, 0x2c);
    add_instruction(&mut instruction_set, Mnemonic::BIT, AddressMode::Zeropage, 0x24);
    add_instruction(&mut instruction_set, Mnemonic::BMI, AddressMode::Relative, 0x30);
    add_instruction(&mut instruction_set, Mnemonic::BNE, AddressMode::Relative, 0xd0);
    add_instruction(&mut instruction_set, Mnemonic::BPL, AddressMode::Relative, 0x10);
    add_instruction(&mut instruction_set, Mnemonic::BRA, AddressMode::Relative, 0x80);
    add_instruction(&mut instruction_set, Mnemonic::BRK, AddressMode::Implied, 0x00);
    add_instruction(&mut instruction_set, Mnemonic::BVC, AddressMode::Relative, 0x50);
    add_instruction(&mut instruction_set, Mnemonic::BVS, AddressMode::Relative, 0x70);
    add_instruction(&mut instruction_set, Mnemonic::CLC, AddressMode::Implied, 0x18);
    add_instruction(&mut instruction_set, Mnemonic::CLD, AddressMode::Implied, 0xd8);
    add_instruction(&mut instruction_set, Mnemonic::CLI, AddressMode::Implied, 0x58);
    add_instruction(&mut instruction_set, Mnemonic::CLV, AddressMode::Implied, 0xb8);
    add_instruction(&mut instruction_set, Mnemonic::CMP, AddressMode::Immediate, 0xc9);
    add_instruction(&mut instruction_set, Mnemonic::CMP, AddressMode::Zeropage, 0xc5);
    add_instruction(&mut instruction_set, Mnemonic::CMP, AddressMode::ZeropageX, 0xd5);
    add_instruction(&mut instruction_set, Mnemonic::CMP, AddressMode::Absolute, 0xcd);
    add_instruction(&mut instruction_set, Mnemonic::CMP, AddressMode::AbsoluteX, 0xdd);
    add_instruction(&mut instruction_set, Mnemonic::CMP, AddressMode::AbsoluteY, 0xd9);
    add_instruction(&mut instruction_set, Mnemonic::CMP, AddressMode::IndirectX, 0xc1);
    add_instruction(&mut instruction_set, Mnemonic::CMP, AddressMode::IndirectY, 0xd1);
    add_instruction(&mut instruction_set, Mnemonic::CPX, AddressMode::Implied, 0xe0);
    add_instruction(&mut instruction_set, Mnemonic::CPX, AddressMode::Zeropage, 0xe4);
    add_instruction(&mut instruction_set, Mnemonic::CPX, AddressMode::Absolute, 0xec);
    add_instruction(&mut instruction_set, Mnemonic::CPY, AddressMode::Implied, 0xc0);
    add_instruction(&mut instruction_set, Mnemonic::CPY, AddressMode::Zeropage, 0xc4);
    add_instruction(&mut instruction_set, Mnemonic::CPY, AddressMode::Absolute, 0xcc);
    add_instruction(&mut instruction_set, Mnemonic::DEC, AddressMode::Zeropage, 0xc6);
    add_instruction(&mut instruction_set, Mnemonic::DEC, AddressMode::ZeropageX, 0xd6);
    add_instruction(&mut instruction_set, Mnemonic::DEC, AddressMode::Absolute, 0xce);
    add_instruction(&mut instruction_set, Mnemonic::DEC, AddressMode::AbsoluteX, 0xde);
    add_instruction(&mut instruction_set, Mnemonic::DEX, AddressMode::Implied, 0xca);
    add_instruction(&mut instruction_set, Mnemonic::DEY, AddressMode::Implied, 0x88);
    add_instruction(&mut instruction_set, Mnemonic::EOR, AddressMode::Immediate, 0x49);
    add_instruction(&mut instruction_set, Mnemonic::EOR, AddressMode::Zeropage, 0x45);
    add_instruction(&mut instruction_set, Mnemonic::EOR, AddressMode::ZeropageX, 0x45);
    add_instruction(&mut instruction_set, Mnemonic::EOR, AddressMode::Absolute, 0x4d);
    add_instruction(&mut instruction_set, Mnemonic::EOR, AddressMode::AbsoluteX, 0x5d);
    add_instruction(&mut instruction_set, Mnemonic::EOR, AddressMode::AbsoluteY, 0x59);
    add_instruction(&mut instruction_set, Mnemonic::EOR, AddressMode::IndirectX, 0x41);
    add_instruction(&mut instruction_set, Mnemonic::EOR, AddressMode::IndirectY, 0x51);
    add_instruction(&mut instruction_set, Mnemonic::INC, AddressMode::Implied, 0x1a);
    add_instruction(&mut instruction_set, Mnemonic::INC, AddressMode::Zeropage, 0xe6);
    add_instruction(&mut instruction_set, Mnemonic::INC, AddressMode::ZeropageX, 0xf6);
    add_instruction(&mut instruction_set, Mnemonic::INC, AddressMode::Absolute, 0xee);
    add_instruction(&mut instruction_set, Mnemonic::INC, AddressMode::AbsoluteX, 0xfe);
    add_instruction(&mut instruction_set, Mnemonic::INX, AddressMode::Implied, 0xe8);
    add_instruction(&mut instruction_set, Mnemonic::INY, AddressMode::Implied, 0xc8);
    add_instruction(&mut instruction_set, Mnemonic::JMP, AddressMode::Absolute, 0x4c);
    add_instruction(&mut instruction_set, Mnemonic::JMP, AddressMode::Indirect, 0x6c);
    add_instruction(&mut instruction_set, Mnemonic::JSR, AddressMode::Absolute, 0x20);
    add_instruction(&mut instruction_set, Mnemonic::LDA, AddressMode::Immediate, 0xa9);
    add_instruction(&mut instruction_set, Mnemonic::LDA, AddressMode::Zeropage, 0xa5);
    add_instruction(&mut instruction_set, Mnemonic::LDA, AddressMode::ZeropageX, 0xb5);
    add_instruction(&mut instruction_set, Mnemonic::LDA, AddressMode::Absolute, 0xad);
    add_instruction(&mut instruction_set, Mnemonic::LDA, AddressMode::AbsoluteX, 0xbd);
    add_instruction(&mut instruction_set, Mnemonic::LDA, AddressMode::AbsoluteY, 0xb9);
    add_instruction(&mut instruction_set, Mnemonic::LDA, AddressMode::IndirectX, 0xa1);
    add_instruction(&mut instruction_set, Mnemonic::LDA, AddressMode::IndirectY, 0xb1);
    add_instruction(&mut instruction_set, Mnemonic::LDX, AddressMode::Immediate, 0xa2);
    add_instruction(&mut instruction_set, Mnemonic::LDX, AddressMode::Zeropage, 0xa6);
    add_instruction(&mut instruction_set, Mnemonic::LDX, AddressMode::ZeropageX, 0xb6);
    add_instruction(&mut instruction_set, Mnemonic::LDX, AddressMode::Absolute, 0xae);
    add_instruction(&mut instruction_set, Mnemonic::LDX, AddressMode::AbsoluteX, 0xbe);
    add_instruction(&mut instruction_set, Mnemonic::LDY, AddressMode::Immediate, 0xa0);
    add_instruction(&mut instruction_set, Mnemonic::LDY, AddressMode::Zeropage, 0xa4);
    add_instruction(&mut instruction_set, Mnemonic::LDY, AddressMode::ZeropageX, 0xb4);
    add_instruction(&mut instruction_set, Mnemonic::LDY, AddressMode::Absolute, 0xac);
    add_instruction(&mut instruction_set, Mnemonic::LDY, AddressMode::AbsoluteX, 0xbc);
    add_instruction(&mut instruction_set, Mnemonic::LSR, AddressMode::Immediate, 0x4a);
    add_instruction(&mut instruction_set, Mnemonic::LSR, AddressMode::Zeropage, 0x46);
    add_instruction(&mut instruction_set, Mnemonic::LSR, AddressMode::ZeropageX, 0x56);
    add_instruction(&mut instruction_set, Mnemonic::LSR, AddressMode::Absolute, 0x4e);
    add_instruction(&mut instruction_set, Mnemonic::LSR, AddressMode::AbsoluteX, 0x5e);
    add_instruction(&mut instruction_set, Mnemonic::NOP, AddressMode::Implied, 0xea);
    add_instruction(&mut instruction_set, Mnemonic::ORA, AddressMode::Immediate, 0x09);
    add_instruction(&mut instruction_set, Mnemonic::ORA, AddressMode::Zeropage, 0x05);
    add_instruction(&mut instruction_set, Mnemonic::ORA, AddressMode::ZeropageX, 0x15);
    add_instruction(&mut instruction_set, Mnemonic::ORA, AddressMode::Absolute, 0x0d);
    add_instruction(&mut instruction_set, Mnemonic::ORA, AddressMode::AbsoluteX, 0x1d);
    add_instruction(&mut instruction_set, Mnemonic::ORA, AddressMode::AbsoluteY, 0x19);
    add_instruction(&mut instruction_set, Mnemonic::ORA, AddressMode::IndirectX, 0x01);
    add_instruction(&mut instruction_set, Mnemonic::ORA, AddressMode::IndirectY, 0x11);
    add_instruction(&mut instruction_set, Mnemonic::PHA, AddressMode::Implied, 0x48);
    add_instruction(&mut instruction_set, Mnemonic::PHP, AddressMode::Implied, 0x08);
    add_instruction(&mut instruction_set, Mnemonic::PHX, AddressMode::Implied, 0xda);
    add_instruction(&mut instruction_set, Mnemonic::PHY, AddressMode::Implied, 0x5a);
    add_instruction(&mut instruction_set, Mnemonic::PLA, AddressMode::Implied, 0x68);
    add_instruction(&mut instruction_set, Mnemonic::PLP, AddressMode::Implied, 0x28);
    add_instruction(&mut instruction_set, Mnemonic::PLX, AddressMode::Relative, 0xfa);
    add_instruction(&mut instruction_set, Mnemonic::PLY, AddressMode::Relative, 0x7a);
    add_instruction(&mut instruction_set, Mnemonic::RMB0, AddressMode::Zeropage, 0x07);
    add_instruction(&mut instruction_set, Mnemonic::RMB1, AddressMode::Zeropage, 0x17);
    add_instruction(&mut instruction_set, Mnemonic::RMB2, AddressMode::Zeropage, 0x27);
    add_instruction(&mut instruction_set, Mnemonic::RMB3, AddressMode::Zeropage, 0x37);
    add_instruction(&mut instruction_set, Mnemonic::RMB4, AddressMode::Zeropage, 0x47);
    add_instruction(&mut instruction_set, Mnemonic::RMB5, AddressMode::Zeropage, 0x57);
    add_instruction(&mut instruction_set, Mnemonic::RMB6, AddressMode::Zeropage, 0x67);
    add_instruction(&mut instruction_set, Mnemonic::RMB7, AddressMode::Zeropage, 0x77);
    add_instruction(&mut instruction_set, Mnemonic::ROL, AddressMode::Implied, 0x2a);
    add_instruction(&mut instruction_set, Mnemonic::ROL, AddressMode::Zeropage, 0x26);
    add_instruction(&mut instruction_set, Mnemonic::ROL, AddressMode::ZeropageX, 0x36);
    add_instruction(&mut instruction_set, Mnemonic::ROL, AddressMode::Absolute, 0x2e);
    add_instruction(&mut instruction_set, Mnemonic::ROL, AddressMode::AbsoluteX, 0x3e);
    add_instruction(&mut instruction_set, Mnemonic::ROR, AddressMode::Implied, 0x6a);
    add_instruction(&mut instruction_set, Mnemonic::ROR, AddressMode::Zeropage, 0x66);
    add_instruction(&mut instruction_set, Mnemonic::ROR, AddressMode::ZeropageX, 0x76);
    add_instruction(&mut instruction_set, Mnemonic::ROR, AddressMode::Absolute, 0x6e);
    add_instruction(&mut instruction_set, Mnemonic::ROR, AddressMode::AbsoluteX, 0x7e);
    add_instruction(&mut instruction_set, Mnemonic::RTI, AddressMode::Implied, 0x40);
    add_instruction(&mut instruction_set, Mnemonic::RTS, AddressMode::Implied, 0x60);
    add_instruction(&mut instruction_set, Mnemonic::SBC, AddressMode::Immediate, 0xe9);
    add_instruction(&mut instruction_set, Mnemonic::SBC, AddressMode::Zeropage, 0xe5);
    add_instruction(&mut instruction_set, Mnemonic::SBC, AddressMode::ZeropageX, 0xf5);
    add_instruction(&mut instruction_set, Mnemonic::SBC, AddressMode::Absolute, 0xed);
    add_instruction(&mut instruction_set, Mnemonic::SBC, AddressMode::AbsoluteX, 0xfd);
    add_instruction(&mut instruction_set, Mnemonic::SBC, AddressMode::AbsoluteY, 0xf9);
    add_instruction(&mut instruction_set, Mnemonic::SBC, AddressMode::IndirectX, 0xe1);
    add_instruction(&mut instruction_set, Mnemonic::SBC, AddressMode::IndirectY, 0xf1);
    add_instruction(&mut instruction_set, Mnemonic::SEC, AddressMode::Implied, 0x38);
    add_instruction(&mut instruction_set, Mnemonic::SED, AddressMode::Implied, 0xf8);
    add_instruction(&mut instruction_set, Mnemonic::SEI, AddressMode::Implied, 0x78);
    add_instruction(&mut instruction_set, Mnemonic::SMB0, AddressMode::Zeropage, 0x87);
    add_instruction(&mut instruction_set, Mnemonic::SMB1, AddressMode::Zeropage, 0x97);
    add_instruction(&mut instruction_set, Mnemonic::SMB2, AddressMode::Zeropage, 0xa7);
    add_instruction(&mut instruction_set, Mnemonic::SMB3, AddressMode::Zeropage, 0xb7);
    add_instruction(&mut instruction_set, Mnemonic::SMB4, AddressMode::Zeropage, 0xc7);
    add_instruction(&mut instruction_set, Mnemonic::SMB5, AddressMode::Zeropage, 0xd7);
    add_instruction(&mut instruction_set, Mnemonic::SMB6, AddressMode::Zeropage, 0xe7);
    add_instruction(&mut instruction_set, Mnemonic::SMB7, AddressMode::Zeropage, 0xf7);
    add_instruction(&mut instruction_set, Mnemonic::STA, AddressMode::Zeropage, 0x85);
    add_instruction(&mut instruction_set, Mnemonic::STA, AddressMode::ZeropageX, 0x95);
    add_instruction(&mut instruction_set, Mnemonic::STA, AddressMode::Absolute, 0x8d);
    add_instruction(&mut instruction_set, Mnemonic::STA, AddressMode::AbsoluteX, 0x9d);
    add_instruction(&mut instruction_set, Mnemonic::STA, AddressMode::AbsoluteY, 0x99);
    add_instruction(&mut instruction_set, Mnemonic::STA, AddressMode::IndirectX, 0x81);
    add_instruction(&mut instruction_set, Mnemonic::STA, AddressMode::IndirectY, 0x91);
    add_instruction(&mut instruction_set, Mnemonic::STP, AddressMode::Implied, 0xdb);
    add_instruction(&mut instruction_set, Mnemonic::STX, AddressMode::Zeropage, 0x85);
    add_instruction(&mut instruction_set, Mnemonic::STX, AddressMode::ZeropageY, 0x95);
    add_instruction(&mut instruction_set, Mnemonic::STX, AddressMode::Absolute, 0x8d);
    add_instruction(&mut instruction_set, Mnemonic::STY, AddressMode::Zeropage, 0x85);
    add_instruction(&mut instruction_set, Mnemonic::STY, AddressMode::ZeropageX, 0x95);
    add_instruction(&mut instruction_set, Mnemonic::STY, AddressMode::Absolute, 0x8d);
    add_instruction(&mut instruction_set, Mnemonic::STZ, AddressMode::Zeropage, 0x64);
    add_instruction(&mut instruction_set, Mnemonic::STZ, AddressMode::ZeropageX, 0x94);
    add_instruction(&mut instruction_set, Mnemonic::STZ, AddressMode::Absolute, 0x9c);
    add_instruction(&mut instruction_set, Mnemonic::TAX, AddressMode::Implied, 0xaa);
    add_instruction(&mut instruction_set, Mnemonic::TAY, AddressMode::Implied, 0xa8);
    add_instruction(&mut instruction_set, Mnemonic::TRB, AddressMode::Absolute, 0x1c);
    add_instruction(&mut instruction_set, Mnemonic::TRB, AddressMode::Zeropage, 0x14);
    add_instruction(&mut instruction_set, Mnemonic::TSB, AddressMode::Absolute, 0x0c);
    add_instruction(&mut instruction_set, Mnemonic::TSB, AddressMode::Zeropage, 0x04);
    add_instruction(&mut instruction_set, Mnemonic::TSX, AddressMode::Implied, 0xba);
    add_instruction(&mut instruction_set, Mnemonic::TXA, AddressMode::Implied, 0x8a);
    add_instruction(&mut instruction_set, Mnemonic::TXS, AddressMode::Implied, 0x9a);
    add_instruction(&mut instruction_set, Mnemonic::TYA, AddressMode::Implied, 0x98);
    add_instruction(&mut instruction_set, Mnemonic::WAI, AddressMode::Relative, 0xcb);
    instruction_set
}

pub enum Value {
    String(String),
    U8(u8),
    U16(u16),
    Null,
}

pub fn str_to_instruction (instruction: String, operand_type: AddressMode, value: Value) -> (Mnemonic, AddressMode, Value) {
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