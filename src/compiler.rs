mod parser;

use std::fmt;

use crate::vm;

#[derive(Clone, PartialEq, Eq)]
pub struct Program {
    bytes: Box<[u8]>,
}

impl PartialEq<[u8]> for Program {
    fn eq(&self, other: &[u8]) -> bool {
        (&*self.bytes).eq(other)
    }
}

impl<const N: usize> PartialEq<[u8; N]> for Program {
    fn eq(&self, other: &[u8; N]) -> bool {
        (&*self.bytes).eq(other)
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dbg_list = f.debug_list();

        let mut program = self.iter();
        while let Some(byte) = program.next() {
            match byte {
                vm::OP_SUCCESS => {
                    dbg_list.entry(&format_args!("SUCCESS()"));
                },

                vm::OP_MATCH_BYTE => {
                    let arg = program.expect_u8();
                    dbg_list.entry(&format_args!("MATCH_BYTE(b'{}')", arg as char));
                },

                _ => unreachable!("bug: unknown op code '{}'", byte),
            }
        }

        dbg_list.finish()
    }
}

impl Program {
    pub fn compile(pattern: &[u8]) -> Self {
        let mut program = Vec::new();

        for &pattern_byte in pattern {
            match pattern_byte {
                // Default to just matching the byte exactly as is
                _ => {
                    program.push(vm::OP_MATCH_BYTE);
                    program.push(pattern_byte);
                }
            }
        }

        program.push(vm::OP_SUCCESS);

        program.shrink_to_fit();
        let bytes = program.into();

        Program {bytes}
    }

    pub fn iter(&self) -> ProgramIter {
        ProgramIter {
            bytes: &self.bytes,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProgramIter<'a> {
    bytes: &'a [u8],
}

impl<'a> Iterator for ProgramIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let &next_byte = self.bytes.get(0)?;
        self.bytes = &self.bytes[1..];
        Some(next_byte)
    }
}

impl<'a> ProgramIter<'a> {
    pub fn expect_u8(&mut self) -> u8 {
        self.next().expect("bug: expected at least one more byte")
    }
}
