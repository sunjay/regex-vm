use std::fmt;

use crate::vm::OP_MATCH_BYTE;

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

        fn expect_arg(prog: &mut std::slice::Iter<u8>) -> u8 {
            *prog.next().expect("bug: expected an argument while disassembling")
        }

        let mut program = self.bytes.iter();
        while let Some(&byte) = program.next() {
            match byte {
                OP_MATCH_BYTE => {
                    let arg = expect_arg(&mut program);
                    dbg_list.entry(&format_args!("MATCH_BYTE(b'{}')", arg as char));
                },

                _ => unreachable!(),
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
                    program.push(OP_MATCH_BYTE);
                    program.push(pattern_byte);
                }
            }
        }

        program.shrink_to_fit();
        let bytes = program.into();
        Program {bytes}
    }
}
