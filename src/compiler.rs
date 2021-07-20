mod program;
mod program_builder;

pub use program::*;
pub use program_builder::*;

use regex_syntax::{Parser, hir::{self, Hir}};

use crate::vm;

pub fn compile_regex(pattern: &str) -> Program {
    let mut program = ProgramBuilder::default();

    for &pattern_byte in pattern.as_bytes() {
        match pattern_byte {
            // Default to just matching the byte exactly as is
            _ => {
                program.push_u8(vm::OP_MATCH_CHAR);
                program.push_char(pattern_byte as char);
            }
        }
    }

    program.push_u8(vm::OP_SUCCESS);

    Program::from_bytes(program.into_bytes())
}
