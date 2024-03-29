#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// Any whitespace characters sequence.
    Whitespace,

    /// "ident" or "keyword"
    /// At this step keywords are considered identifiers.
    Ident,

    /// Literal.
    Literal {
        kind: LiteralKind,
    },

    /// `;`
    Semi,

    /// `:`
    Colon,

    // `.`
    Dot,

    // `,`
    Comma,

    /// `(`
    OpenParen,

    /// `)`
    CloseParen,

    /// `{`
    OpenBrace,

    /// `}`
    CloseBrace,

    /// `->`
    Arrow,

    /// `=`
    Eq,

    /// `!`
    Bang,

    /// `<`
    Lt,

    /// `<=`
    Le,

    /// `==`
    EqEq,

    /// `!=`
    Ne,

    /// `>=`
    Ge,

    /// `>`
    Gt,

    /// `-`
    Minus,

    /// `+`
    Plus,

    /// `*`
    Star,

    /// `/`
    Slash,

    /// Unknown token, not expected by the lexer.
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int,
}
