use crate::compiler::{Program, ProgramIter};

pub const OP_SUCCESS: u8 = 0;

pub const OP_MATCH_BYTE: u8 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecuteStatus {
    /// Evaluated the program to completion with no errors
    Success,

    /// Aborted the execution of the program due to a mismatch
    Aborted,
}

#[derive(Debug, Clone)]
struct Input<'a> {
    bytes: &'a [u8],
}

impl<'a> Input<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {bytes}
    }
}

impl<'a> Iterator for Input<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let &next_byte = self.bytes.get(0)?;
        self.bytes = &self.bytes[1..];
        Some(next_byte)
    }
}


#[derive(Debug)]
pub struct RegexVM<'a> {
    program: ProgramIter<'a>,
    input: Input<'a>,
}

impl<'a> RegexVM<'a> {
    pub fn new(program: &'a Program, input: &'a [u8]) -> Self {
        let program = program.iter();
        let input = Input::new(input);

        Self {program, input}
    }

    pub fn execute(&mut self) -> ExecuteStatus {
        while let Some(program_byte) = self.program.next() {
            match program_byte {
                OP_SUCCESS => return ExecuteStatus::Success,

                OP_MATCH_BYTE => {
                    let target_byte = self.program.expect_u8();

                    if self.input.next() != Some(target_byte) {
                        return ExecuteStatus::Aborted;
                    }
                },

                _ => unreachable!("bug: miscompiled program with unknown op code '{}'", program_byte),
            }
        }

        unreachable!("bug: program ended without success or abort instruction")
    }
}
