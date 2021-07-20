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

                vm::OP_MATCH_CHAR => {
                    let arg = program.expect_char();
                    dbg_list.entry(&format_args!("MATCH_CHAR('{}')", arg));
                },

                _ => unreachable!("bug: unknown op code '{}'", byte),
            }
        }

        dbg_list.finish()
    }
}

impl Program {
    pub(super) fn from_bytes(bytes: Box<[u8]>) -> Self {
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
    pub fn expect_char(&mut self) -> char {
        char::from_u32(u32::from_le_bytes([
            self.expect_u8(),
            self.expect_u8(),
            self.expect_u8(),
            self.expect_u8(),
        ])).expect("bug: invalid char in program")
    }

    fn expect_u8(&mut self) -> u8 {
        self.next().expect("bug: expected at least one more byte")
    }
}
