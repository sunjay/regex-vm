pub mod ast;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ParserInput<'a>(&'a [u8]);

impl<'a> ParserInput<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self(input)
    }

    pub fn next(&mut self) -> Option<u8> {
        let &next_byte = self.0.get(0)?;
        self.0 = &self.0[1..];
        Some(next_byte)
    }

    pub fn peek(&mut self) -> Option<u8> {
        self.0.get(0).copied()
    }
}

pub fn parse_pattern(pattern: &[u8]) -> ast::Expr {
    let input = ParserInput::new(pattern);
    expr(input)
}

fn expr(input: ParserInput) -> ast::Expr {
    todo!()
}
