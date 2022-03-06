use span::{span::Span, symbol::Symbol};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BinOpToken {
    Plus,
    Minus,
    Star,
    Slash,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DelimToken {
    Paren,
    Brace,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
    BinOp(BinOpToken),

    OpenDelim(DelimToken),
    CloseDelim(DelimToken),

    Ident(Symbol),

    Eof,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    #[inline]
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}
