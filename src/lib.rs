pub const OP_MATCH_BYTE: u8 = 0;

#[derive(Debug)]
pub struct Regex {
}

impl Regex {
    pub fn compile(pattern: &[u8]) -> Self {
        Self {
        }
    }

    pub fn program(&self) -> &[u8] {
        todo!()
    }

    pub fn is_match_bytes(&self, bytes: &[u8]) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert!(!pattern.is_match_bytes(b""));
        assert!(!pattern.is_match_bytes(b"a"));
        assert!(!pattern.is_match_bytes(b"ab"));
        assert!(!pattern.is_match_bytes(b"abc"));
        assert!(!pattern.is_match_bytes(b"abca"));

        assert!(!pattern.is_match_bytes(b"abbd"));

        assert!(pattern.is_match_bytes(b"abcd"));
        assert!(pattern.is_match_bytes(b"abcde"));
        assert!(pattern.is_match_bytes(b"abcdabcd"));
    }
}
