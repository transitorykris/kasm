use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

#[derive(PartialEq, Eq, Hash)]
enum AddressMode {
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
}

#[derive(PartialEq, Eq, Hash)]
enum Mnemonic {
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
struct InstructionKey{
    mnemonic: Mnemonic,
    address_mode: AddressMode,
}

type Opcode = u8;
type InstructionMap = HashMap<InstructionKey, Opcode>;

const OUTSIZE: usize = 16384;       // We're generating binaries for a 16KB EEPROM
const OUTFILE: &str = "a.out";      // A typical default
const INFILE: &str = "example.s";   // Hardcode our source filename for the time being

fn main() {
    println!("kasm");

    // Read our source file
    let source = read_source(INFILE);
    println!("{}", source);

    // Get our instruction set
    let mut instruction_set = generate_instruction_set();

    // Pass 1
    pass1(&source);

    // Pass 2
    pass2();

    // Assemble
    let output = assemble();

    // Write output file
    write_out(OUTFILE, output);
}

fn pass1(source: &str) {
    // Iterate over each line of source file
    // Identify directives, labels, and symbols
    // Strip comments -- Careful with string literals
    // Store labels in a label table to have their addresses determined later
    // Determine length in bytes the symbol requires
}

fn pass2() {
    // Determine the location of each of the labels
    // Update each reference to the symbol
}

fn assemble() -> Vec<u8> {
    let mut output = vec![0; OUTSIZE];
    output
}

fn read_source(file: &str) -> String {
    let path = Path::new(file);
    let display = path.display();
    let mut f = match File::open(path) {
        Err(why) => {
            panic!("Couldn't open {}: {}", display, why.to_string())
        },
        Ok(f) => f,
    };

    let mut source = String::new();
    let _ = match f.read_to_string(&mut source) {
        Err(why) => {
            panic!("Couldn't read {}: {}", display, why.to_string())
        },
        Ok(source) => source,
    };
    source
}

fn write_out(filename: &str, output: Vec<u8>) {
    let path = Path::new(filename);
    let display = path.display();
    let mut f = match File::create(&path) {
        Err(why) => {
            panic!("Couldn't create {}: {}", display, why.to_string())
        },
        Ok(f) => f,
    };

    let _ = match f.write_all(&output) {
        Err(why) => {
            panic!("Couldn't write {}: {}", display, why.to_string())
        },
        Ok(f) => f,
    };
}

// returns the length in bytes of instruction for the given the AddressMode
fn address_mode_length(address_mode: AddressMode) -> u8 {
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
    }
}

fn add_instruction(instruction_set: &mut InstructionMap, mnemonic: Mnemonic,
        address_mode: AddressMode, opcode: u8) {
    instruction_set.insert(InstructionKey{mnemonic, address_mode}, opcode);
}

// Something will inevitably wrong in the below!
fn generate_instruction_set() -> InstructionMap {
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
    add_instruction(&mut instruction_set, Mnemonic::BBR0, AddressMode::Relative, 0x1f);
    add_instruction(&mut instruction_set, Mnemonic::BBR0, AddressMode::Relative, 0x2f);
    add_instruction(&mut instruction_set, Mnemonic::BBR0, AddressMode::Relative, 0x3f);
    add_instruction(&mut instruction_set, Mnemonic::BBR0, AddressMode::Relative, 0x4f);
    add_instruction(&mut instruction_set, Mnemonic::BBR0, AddressMode::Relative, 0x5f);
    add_instruction(&mut instruction_set, Mnemonic::BBR0, AddressMode::Relative, 0x6f);
    add_instruction(&mut instruction_set, Mnemonic::BBR0, AddressMode::Relative, 0x7f);
    add_instruction(&mut instruction_set, Mnemonic::BBS0, AddressMode::Relative, 0x8f);
    add_instruction(&mut instruction_set, Mnemonic::BBS0, AddressMode::Relative, 0x9f);
    add_instruction(&mut instruction_set, Mnemonic::BBS0, AddressMode::Relative, 0xaf);
    add_instruction(&mut instruction_set, Mnemonic::BBS0, AddressMode::Relative, 0xbf);
    add_instruction(&mut instruction_set, Mnemonic::BBS0, AddressMode::Relative, 0xcf);
    add_instruction(&mut instruction_set, Mnemonic::BBS0, AddressMode::Relative, 0xdf);
    add_instruction(&mut instruction_set, Mnemonic::BBS0, AddressMode::Relative, 0xef);
    add_instruction(&mut instruction_set, Mnemonic::BBS0, AddressMode::Relative, 0xff);
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
    add_instruction(&mut instruction_set, Mnemonic::INC, AddressMode::Zeropage, 0xe6);
    add_instruction(&mut instruction_set, Mnemonic::INC, AddressMode::ZeropageX, 0xf6);
    add_instruction(&mut instruction_set, Mnemonic::INC, AddressMode::Absolute, 0xee);
    add_instruction(&mut instruction_set, Mnemonic::INC, AddressMode::AbsoluteX, 0xfe);
    add_instruction(&mut instruction_set, Mnemonic::INX, AddressMode::Implied, 0xe8);
    add_instruction(&mut instruction_set, Mnemonic::INY, AddressMode::Implied, 0xc8);
    add_instruction(&mut instruction_set, Mnemonic::JMP, AddressMode::Absolute, 0x4c);
    add_instruction(&mut instruction_set, Mnemonic::JMP, AddressMode::Indirect, 0x6c);
    add_instruction(&mut instruction_set, Mnemonic::JSR, AddressMode::Absolute, 0x20);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::Immediate, 0xa9);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::Zeropage, 0xa5);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::ZeropageX, 0xb5);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::Absolute, 0xad);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::AbsoluteX, 0xbd);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::AbsoluteY, 0xb9);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::IndirectX, 0xa1);
    add_instruction(&mut instruction_set, Mnemonic::ADC, AddressMode::IndirectY, 0xb1);
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
    add_instruction(&mut instruction_set, Mnemonic::STP, AddressMode::Relative, 0xdb);
    add_instruction(&mut instruction_set, Mnemonic::STX, AddressMode::Zeropage, 0x85);
    add_instruction(&mut instruction_set, Mnemonic::STX, AddressMode::ZeropageY, 0x95);
    add_instruction(&mut instruction_set, Mnemonic::STX, AddressMode::Absolute, 0x8d);
    add_instruction(&mut instruction_set, Mnemonic::STY, AddressMode::Zeropage, 0x85);
    add_instruction(&mut instruction_set, Mnemonic::STY, AddressMode::ZeropageX, 0x95);
    add_instruction(&mut instruction_set, Mnemonic::STY, AddressMode::Absolute, 0x8d);
    add_instruction(&mut instruction_set, Mnemonic::TAX, AddressMode::Implied, 0xaa);
    add_instruction(&mut instruction_set, Mnemonic::TAY, AddressMode::Implied, 0xa8);
    add_instruction(&mut instruction_set, Mnemonic::TRB, AddressMode::Absolute, 0x1c);
    add_instruction(&mut instruction_set, Mnemonic::TRB, AddressMode::Zeropage, 0x14);
    add_instruction(&mut instruction_set, Mnemonic::TSB, AddressMode::Absolute, 0x0c);
    add_instruction(&mut instruction_set, Mnemonic::TSB, AddressMode::Zeropage, 0x04);
    add_instruction(&mut instruction_set, Mnemonic::TSX, AddressMode::Implied, 0xba);
    add_instruction(&mut instruction_set, Mnemonic::TXA, AddressMode::Implied, 0x8a);
    add_instruction(&mut instruction_set, Mnemonic::TYA, AddressMode::Implied, 0x98);
    add_instruction(&mut instruction_set, Mnemonic::WAI, AddressMode::Relative, 0xcb);
    instruction_set
}
