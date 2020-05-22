pub use crate::instructions::Mnemonic;
pub use crate::instructions::AddressMode;

enum Instruction {
    Absolute(Counter, Mnemonic, u16),
    AbsoluteX(Counter, Mnemonic, u16),
    AbsoluteY(Counter, Mnemonic, u16),
    Immediate(Counter, Mnemonic, u8),
    Implied(Counter, Mnemonic),
    Indirect(Counter, Mnemonic, u8),
    IndirectX(Counter, Mnemonic, u8),
    IndirectY(Counter, Mnemonic, u8),
    Relative(Counter, Mnemonic, u8),
    Zeropage(Counter, Mnemonic, u8),
    ZeropageX(Counter, Mnemonic, u8),
    ZeropageY(Counter, Mnemonic, u8),
    Unresolved(Counter, Mnemonic),  // Uses a label we haven't resolved
}

type SymbolTable = Vec<(String, Address, Line)>;
type Address = u16;
type Counter = u16;
type Line = u16;
pub type CodeTable = Vec<(Mnemonic, AddressMode, Value)>;

pub enum Value {
    String(String),
    U8(u8),
    U16(u16),
    Null,
}

pub struct Program {
    symbol_table: SymbolTable,
    pub code: CodeTable,
}

pub fn pass1(source: &str) -> Program {
    let mut program = Program {
        symbol_table: SymbolTable::new(),
        code: CodeTable::new(),
    };

    // Iterate over each line of source file
    for raw_line in source.lines() {
        let line = raw_line.trim();     // Trim off leading and trailing whitespace
        if line.len() == 0 {
            // We got a blank line
        } else if line.chars().next().unwrap() == ';' {
            // We got a comment
        } else if line.chars().next().unwrap() == '.' {
            // We've got a directive
            handle_directive(&mut program, line);
        } else if line.chars().next().unwrap().is_ascii_alphabetic() {
            // We got an instruction
            println!("Instruction found: {}", line);
            handle_instruction(&mut program, line);
        } else {
            // this isn't good!
            panic!("Unknown syntax: {}", line);
        }
    }
    // Store labels in a label table to have their addresses determined later
    // Determine length in bytes the symbol requires
    program
}

// TODO: implement directives!
fn handle_directive(program: &mut Program, line: &str) {

}

fn handle_instruction(program: &mut Program, line: &str) {
    let mut parts = line.split_ascii_whitespace();
    let instruction = parts.next().unwrap().to_lowercase();
    println!("About to handle instruction {}", instruction);
    let operand_part = parts.next();
    let value: Value;
    let operand_type: AddressMode;
    if operand_part.is_none() {
        println!("is_none!!!!");
        operand_type = AddressMode::Implied;
        value = Value::Null;
    } else {
        println!("unwrap!!!!");
        let (operand_type_tmp, value_tmp) = get_operand_type(operand_part.unwrap());
        operand_type = operand_type_tmp;
        value = value_tmp;
    }

    match instruction.as_str() {
        "adc" => program.code.push((Mnemonic::ADC, operand_type, value)),
        "and" => program.code.push((Mnemonic::AND, operand_type, value)),
        "asl" => program.code.push((Mnemonic::ASL, operand_type, value)),
        "bbr0" => program.code.push((Mnemonic::BBR0, operand_type, value)),
        "bbr1" => program.code.push((Mnemonic::BBR1, operand_type, value)),
        "bbr2" => program.code.push((Mnemonic::BBR2, operand_type, value)),
        "bbr3" => program.code.push((Mnemonic::BBR3, operand_type, value)),
        "bbr4" => program.code.push((Mnemonic::BBR4, operand_type, value)),
        "bbr5" => program.code.push((Mnemonic::BBR5, operand_type, value)),
        "bbr6" => program.code.push((Mnemonic::BBR6, operand_type, value)),
        "bbr7" => program.code.push((Mnemonic::BBR7, operand_type, value)),
        "bbs0" => program.code.push((Mnemonic::BBS0, operand_type, value)),
        "bbs1" => program.code.push((Mnemonic::BBS1, operand_type, value)),
        "bbs2" => program.code.push((Mnemonic::BBS2, operand_type, value)),
        "bbs3" => program.code.push((Mnemonic::BBS3, operand_type, value)),
        "bbs4" => program.code.push((Mnemonic::BBS4, operand_type, value)),
        "bbs5" => program.code.push((Mnemonic::BBS5, operand_type, value)),
        "bbs6" => program.code.push((Mnemonic::BBS6, operand_type, value)),
        "bbs7" => program.code.push((Mnemonic::BBS7, operand_type, value)),
        "bcc" => program.code.push((Mnemonic::BCC, operand_type, value)),
        "bcs" => program.code.push((Mnemonic::BCS, operand_type, value)),
        "beq" => program.code.push((Mnemonic::BEQ, operand_type, value)),
        "bit" => program.code.push((Mnemonic::BIT, operand_type, value)),
        "bmi" => program.code.push((Mnemonic::BMI, operand_type, value)),
        "bne" => program.code.push((Mnemonic::BNE, operand_type, value)),
        "bpl" => program.code.push((Mnemonic::BPL, operand_type, value)),
        "bra" => program.code.push((Mnemonic::BRA, operand_type, value)),
        "brk" => program.code.push((Mnemonic::BRK, operand_type, value)),
        "bvc" => program.code.push((Mnemonic::BVC, operand_type, value)),
        "bvs" => program.code.push((Mnemonic::BVS, operand_type, value)),
        "clc" => program.code.push((Mnemonic::CLC, operand_type, value)),
        "cld" => program.code.push((Mnemonic::CLD, operand_type, value)),
        "cli" => program.code.push((Mnemonic::CLI, operand_type, value)),
        "clv" => program.code.push((Mnemonic::CLV, operand_type, value)),
        "cmp" => program.code.push((Mnemonic::CMP, operand_type, value)),
        "cpx" => program.code.push((Mnemonic::CPX, operand_type, value)),
        "cpy" => program.code.push((Mnemonic::CPY, operand_type, value)),
        "dec" => program.code.push((Mnemonic::DEC, operand_type, value)),
        "dex" => program.code.push((Mnemonic::DEX, operand_type, value)),
        "dey" => program.code.push((Mnemonic::DEY, operand_type, value)),
        "eor" => program.code.push((Mnemonic::EOR, operand_type, value)),
        "inc" => program.code.push((Mnemonic::INC, operand_type, value)),
        "inx" => program.code.push((Mnemonic::INX, operand_type, value)),
        "iny" => program.code.push((Mnemonic::INY, operand_type, value)),
        "jmp" => program.code.push((Mnemonic::JMP, operand_type, value)),
        "jsr" => program.code.push((Mnemonic::JSR, operand_type, value)),
        "lda" => program.code.push((Mnemonic::LDA, operand_type, value)),
        "ldx" => program.code.push((Mnemonic::LDX, operand_type, value)),
        "ldy" => program.code.push((Mnemonic::LDY, operand_type, value)),
        "lsr" => program.code.push((Mnemonic::LSR, operand_type, value)),
        "nop" => program.code.push((Mnemonic::NOP, operand_type, value)),
        "ora" => program.code.push((Mnemonic::ORA, operand_type, value)),
        "pha" => program.code.push((Mnemonic::PHA, operand_type, value)),
        "php" => program.code.push((Mnemonic::PHP, operand_type, value)),
        "phx" => program.code.push((Mnemonic::PHX, operand_type, value)),
        "phy" => program.code.push((Mnemonic::PHY, operand_type, value)),
        "pla" => program.code.push((Mnemonic::PLA, operand_type, value)),
        "plp" => program.code.push((Mnemonic::PLP, operand_type, value)),
        "plx" => program.code.push((Mnemonic::PLX, operand_type, value)),
        "ply" => program.code.push((Mnemonic::PLY, operand_type, value)),
        "rmb0" => program.code.push((Mnemonic::RMB0, operand_type, value)),
        "rmb1" => program.code.push((Mnemonic::RMB1, operand_type, value)),
        "rmb2" => program.code.push((Mnemonic::RMB2, operand_type, value)),
        "rmb3" => program.code.push((Mnemonic::RMB3, operand_type, value)),
        "rmb4" => program.code.push((Mnemonic::RMB4, operand_type, value)),
        "rmb5" => program.code.push((Mnemonic::RMB5, operand_type, value)),
        "rmb6" => program.code.push((Mnemonic::RMB6, operand_type, value)),
        "rmb7" => program.code.push((Mnemonic::RMB7, operand_type, value)),
        "rol" => program.code.push((Mnemonic::ROL, operand_type, value)),
        "ror" => program.code.push((Mnemonic::ROR, operand_type, value)),
        "rts" => program.code.push((Mnemonic::RTS, operand_type, value)),
        "sbc" => program.code.push((Mnemonic::SBC, operand_type, value)),
        "sec" => program.code.push((Mnemonic::SEC, operand_type, value)),
        "sed" => program.code.push((Mnemonic::SED, operand_type, value)),
        "sei" => program.code.push((Mnemonic::SEI, operand_type, value)),
        "smb0" => program.code.push((Mnemonic::SMB0, operand_type, value)),
        "smb1" => program.code.push((Mnemonic::SMB1, operand_type, value)),
        "smb2" => program.code.push((Mnemonic::SMB2, operand_type, value)),
        "smb3" => program.code.push((Mnemonic::SMB3, operand_type, value)),
        "smb4" => program.code.push((Mnemonic::SMB4, operand_type, value)),
        "smb5" => program.code.push((Mnemonic::SMB5, operand_type, value)),
        "smb6" => program.code.push((Mnemonic::SMB6, operand_type, value)),
        "smb7" => program.code.push((Mnemonic::SMB7, operand_type, value)),
        "sta" => program.code.push((Mnemonic::STA, operand_type, value)),
        "stp" => program.code.push((Mnemonic::STP, operand_type, value)),
        "stx" => program.code.push((Mnemonic::STX, operand_type, value)),
        "sty" => program.code.push((Mnemonic::STY, operand_type, value)),
        "stz" => program.code.push((Mnemonic::STZ, operand_type, value)),
        "tax" => program.code.push((Mnemonic::TAX, operand_type, value)),
        "tay" => program.code.push((Mnemonic::TAY, operand_type, value)),
        "trb" => program.code.push((Mnemonic::TRB, operand_type, value)),
        "tsb" => program.code.push((Mnemonic::TSB, operand_type, value)),
        "tsx" => program.code.push((Mnemonic::TSX, operand_type, value)),
        "txa" => program.code.push((Mnemonic::TXA, operand_type, value)),
        "txs" => program.code.push((Mnemonic::TXS, operand_type, value)),
        "tya" => program.code.push((Mnemonic::TYA, operand_type, value)),
        "wai" => program.code.push((Mnemonic::WAI, operand_type, value)),
        _ => panic!("Unknown instruction: {}", instruction),
    }
}

fn get_operand_type(operand: &str) -> (AddressMode, Value) {
    let mut x_indexed = false;
    let mut y_indexed = false;
    let mut raw_operand = String::from(operand);

    println!("Checking operand {} length {}", raw_operand, raw_operand.len());
    if raw_operand == "" {
        println!("implied");
        return (AddressMode::Implied, Value::Null);
    }

    if raw_operand.starts_with(";") {
        // That's a comment there!
        println!("implied");
        return (AddressMode::Implied, Value::Null);
    }
    
    // Check if this is X or Y indexed
    if raw_operand.ends_with(",x") {
        println!("X indexed");
        x_indexed = true;
    } else if raw_operand.ends_with(",y") {
        println!("Y indexed");
        y_indexed = true;
    }

    if raw_operand.starts_with("$") {
        if raw_operand.len() == 3 && x_indexed {
            println!("zeropage");
            let val = u8::from_str_radix(raw_operand.trim_start_matches("$"), 16);
            return (AddressMode::ZeropageX, Value::U8(val.unwrap()));
        } else if raw_operand.len() == 3 && y_indexed {
            println!("zeropage");
            let val = u8::from_str_radix(raw_operand.trim_start_matches("$"), 16);
            return (AddressMode::ZeropageY, Value::U8(val.unwrap()));
        } else if raw_operand.len() == 3 {
            println!("absolute zeropage");
            let val = u8::from_str_radix(raw_operand.trim_start_matches("$"), 16);
            return (AddressMode::Absolute, Value::U8(val.unwrap()));
        } else {
            println!("absolute");
            let val = u16::from_str_radix(raw_operand.trim_start_matches("$"), 16);
            return (AddressMode::Absolute, Value::U16(val.unwrap()));
        }
    } else if raw_operand.starts_with("#$") {
        println!("immediate");
        let val = u8::from_str_radix(raw_operand.trim_start_matches("#$"), 16);
        return (AddressMode::Immediate, Value::U8(val.unwrap()));
    } else if raw_operand.starts_with("($") && operand.ends_with(")") {
        println!("indirect");
        let operand_trimmed = raw_operand.trim_start_matches("($").trim_end_matches(")");
        let val = u16::from_str_radix(operand_trimmed, 16);
        return (AddressMode::Indirect, Value::U16(val.unwrap()));
    } else if raw_operand.starts_with("($") && x_indexed {
        println!("indirectX");
        println!("{}", raw_operand);
        let operand_trimmed = raw_operand.trim_start_matches("($").trim_end_matches(",x");
        println!("{}", operand_trimmed);
        let val = u16::from_str_radix(operand_trimmed, 16);
        return (AddressMode::IndirectX, Value::U16(val.unwrap()));
    } else if raw_operand.starts_with("($") && y_indexed {
        println!("indirectY");
        let operand_trimmed = raw_operand.trim_start_matches("($").trim_end_matches("),y");
        let val = u16::from_str_radix(operand_trimmed, 16);
        return (AddressMode::IndirectY, Value::U16(val.unwrap()));
    } else {
        // Otherwise must be a label
        println!("unknown");
        (AddressMode::Unknown, Value::String(raw_operand))
    }
}