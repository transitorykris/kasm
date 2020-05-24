pub use crate::instructions::AddressMode;
pub use crate::instructions::Mnemonic;
pub use crate::instructions::Value;
pub use crate::pass1;

// This isn't right, I'll come back to it
type CodeTable = Vec<(Mnemonic, AddressMode, pass1::Value)>;

pub struct Program {
    pub code: CodeTable,
}

pub fn pass2(pass1_program: pass1::Program) -> Program {
    // Determine the location of each of the labels
    // Update each reference to the symbol
    // For now we're not worrying about labels
    let mut program = Program {
        code: CodeTable::new(),
    };

    for line in pass1_program.code {
        program.code.push((line.0, line.1, line.2));
    }

    program
}
