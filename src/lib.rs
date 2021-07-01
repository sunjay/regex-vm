pub mod vm;
pub mod compiler;

use compiler::Program;

#[derive(Debug)]
pub struct Regex {
    program: Program,
}

impl Regex {
    pub fn compile(pattern: &[u8]) -> Self {
        Self {
            program: Program::compile(pattern),
        }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn is_match(&self, bytes: &[u8]) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::vm::OP_MATCH_BYTE;

    #[test]
    fn basic_pattern() {
        let pattern = Regex::compile(b"abcd");

        assert_eq!(pattern.program(), &[
            OP_MATCH_BYTE,
            b'a',
            OP_MATCH_BYTE,
            b'b',
            OP_MATCH_BYTE,
            b'c',
            OP_MATCH_BYTE,
            b'd',
        ]);

        assert!(!pattern.is_match(b""));
        assert!(!pattern.is_match(b"a"));
        assert!(!pattern.is_match(b"ab"));
        assert!(!pattern.is_match(b"abc"));
        assert!(!pattern.is_match(b"abca"));

        assert!(!pattern.is_match(b"abbd"));

        assert!(pattern.is_match(b"abcd"));
        assert!(pattern.is_match(b"abcde"));
        assert!(pattern.is_match(b"abcdabcd"));
    }
}
