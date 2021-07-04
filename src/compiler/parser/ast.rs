/// A span of text
///
/// Invariants:
///
/// * Span must be of size >= 0, i.e. `end >= start`
/// * The `start` and `end` indexes MUST remain within the boundaries of a single file. That is, you
///   can never span two files at the same time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    /// The start index of the span (inclusive)
    pub start: usize,
    /// The end index of the span (exclusive)
    pub end: usize,
}

impl Span {
    /// Creates a span from the start of `self` to the end of `other`
    pub fn to(self, other: Self) -> Self {
        assert!(other.end >= self.start, "bug: span should have at least zero size");

        Self {
            start: self.start,
            end: other.end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    /// An empty regex that matches everything.
    Empty(Span),
    /// The "any character" class.
    Dot(Span),
    /// A single character class. This includes all forms of character classes except for `.`
    /// e.g., `\d`, `\pN`, `[a-z]` and `[[:alpha:]]`.
    CharClass(CharClass<'a>),
    Bytes(&'a [u8]),
    Branch(Box<Branch<'a>>),
}

/// Regular expressions separated by `|`
#[derive(Debug, Clone, PartialEq)]
pub struct Branch<'a> {
    pub left: Expr<'a>,
    pub right: Expr<'a>,
}

/// A single character class expression.
#[derive(Debug, Clone, PartialEq)]
pub enum CharClass<'a> {
    /// A bracketed character class set, which may contain zero or more character ranges and/or zero
    /// or more nested classes. e.g., `[a-zA-Z\d]`.
    Bracketed(CharClassBracketed<'a>),
    //TODO: A perl character class, e.g., \d or \W.
    //TODO: Perl(CharClassPerl),
}

/// A bracketed character class set
#[derive(Debug, Clone, PartialEq)]
pub struct CharClassBracketed<'a> {
    /// Whether this class is negated or not. e.g., `[a]` is not negated but `[^a]` is.
    pub negated: bool,
    pub span: Span,
}
