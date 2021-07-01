pub mod vm;
pub mod compiler;

use vm::{ExecuteStatus, RegexVM};
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
        let mut vm = RegexVM::default();
        vm.execute(&self.program, bytes) == ExecuteStatus::Success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::vm;

    #[test]
    fn empty_pattern() {
        let pattern = Regex::compile(b"");

        assert_eq!(pattern.program(), &[
            vm::OP_SUCCESS,
        ]);

        // empty string is in all strings, so everything should match
        assert!(pattern.is_match(b""));
        assert!(pattern.is_match(b"a"));
        assert!(pattern.is_match(b"abc"));
    }

    #[test]
    fn basic_pattern() {
        let pattern = Regex::compile(b"abcd");

        assert_eq!(pattern.program(), &[
            vm::OP_MATCH_BYTE,
            b'a',
            vm::OP_MATCH_BYTE,
            b'b',
            vm::OP_MATCH_BYTE,
            b'c',
            vm::OP_MATCH_BYTE,
            b'd',
            vm::OP_SUCCESS,
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
