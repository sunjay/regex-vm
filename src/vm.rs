use crate::compiler::{Program, ProgramIter};

pub const OP_SUCCESS: u8 = 0;

pub const OP_MATCH_CHAR: u8 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecuteStatus {
    /// Evaluated the program to completion with no errors
    Success,

    /// Aborted the execution of the program due to a mismatch
    Aborted,
}

#[derive(Debug, Clone)]
struct Input<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> Input<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {chars: input.chars()}
    }
}

impl<'a> Iterator for Input<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next()
    }
}


#[derive(Debug)]
pub struct RegexVM<'a> {
    program: ProgramIter<'a>,
    input: Input<'a>,
}

impl<'a> RegexVM<'a> {
    pub fn new(program: &'a Program, input: &'a str) -> Self {
        let program = program.iter();
        let input = Input::new(input);

        Self {program, input}
    }

    pub fn execute(&mut self) -> ExecuteStatus {
        while let Some(program_byte) = self.program.next() {
            match program_byte {
                OP_SUCCESS => return ExecuteStatus::Success,

                OP_MATCH_CHAR => {
                    let target_char = self.program.expect_char();

                    if self.input.next() != Some(target_char) {
                        return ExecuteStatus::Aborted;
                    }
                },

                _ => unreachable!("bug: miscompiled program with unknown op code '{}'", program_byte),
            }
        }

        unreachable!("bug: program ended without success or abort instruction")
    }
}
