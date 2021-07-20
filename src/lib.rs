pub mod vm;
pub mod compiler;

use vm::{ExecuteStatus, RegexVM};
use compiler::{Program, compile_regex};

#[derive(Debug)]
pub struct Regex {
    program: Program,
}

impl Regex {
    pub fn compile(pattern: &str) -> Self {
        Self {
            program: compile_regex(pattern),
        }
    }

    pub fn program(&self) -> &Program {
        &self.program
    }

    pub fn is_match(&self, input: &str) -> bool {
        let mut vm = RegexVM::new(&self.program, input);
        vm.execute() == ExecuteStatus::Success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::vm;

    #[test]
    fn empty_pattern() {
        let pattern = Regex::compile("");

        assert_eq!(pattern.program(), &[
            vm::OP_SUCCESS,
        ]);

        // empty string is in all strings, so everything should match
        assert!(pattern.is_match(""));
        assert!(pattern.is_match("a"));
        assert!(pattern.is_match("abc"));
    }

    #[test]
    fn basic_pattern() {
        let pattern = Regex::compile("abcd");

        assert_eq!(pattern.program(), &[
            vm::OP_MATCH_CHAR,
            b'a', 0, 0, 0,
            vm::OP_MATCH_CHAR,
            b'b', 0, 0, 0,
            vm::OP_MATCH_CHAR,
            b'c', 0, 0, 0,
            vm::OP_MATCH_CHAR,
            b'd', 0, 0, 0,
            vm::OP_SUCCESS,
        ]);

        assert!(!pattern.is_match(""));
        assert!(!pattern.is_match("a"));
        assert!(!pattern.is_match("ab"));
        assert!(!pattern.is_match("abc"));
        assert!(!pattern.is_match("abca"));

        assert!(!pattern.is_match("abbd"));

        assert!(pattern.is_match("abcd"));
        assert!(pattern.is_match("abcde"));
        assert!(pattern.is_match("abcdabcd"));
    }
}
