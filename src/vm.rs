use crate::compiler::Program;

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

    pub fn next(&mut self) -> Option<u8> {
        let &next_byte = self.bytes.get(0)?;
        self.bytes = &self.bytes[1..];
        Some(next_byte)
    }
}


#[derive(Debug, Default)]
pub struct RegexVM {}

impl RegexVM {
    pub fn execute(&mut self, program: &Program, input: &[u8]) -> ExecuteStatus {
        fn arg_u8(mut program_bytes: impl Iterator<Item=u8>) -> u8 {
            program_bytes.next().expect("bug: miscompiled program did not have expected argument")
        }

        let mut program_bytes = program.bytes().iter().copied();
        let mut input = Input::new(input);
        while let Some(program_byte) = program_bytes.next() {
            match program_byte {
                OP_SUCCESS => return ExecuteStatus::Success,

                OP_MATCH_BYTE => {
                    let target_byte = arg_u8(&mut program_bytes);

                    if input.next() != Some(target_byte) {
                        return ExecuteStatus::Aborted;
                    }
                },

                _ => unreachable!("bug: miscompiled program with unknown op code '{}'", program_byte),
            }
        }

        unreachable!("bug: program ended without success or abort instruction")
    }
}
