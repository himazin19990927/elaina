use span::{
    span::Span,
    symbol::{Symbol, SymbolMap},
};

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

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LitKind {
    Integer,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Lit {
    pub kind: LitKind,
    pub symbol: Symbol,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TokenKind {
    /// `=`
    Eq,

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

    /// Binary operator: `+`, `-`, `*`, `/`
    BinOp(BinOpToken),

    /// `;`
    Semi,

    /// `:`
    Colon,

    /// Open delimiter: `(`, `{`
    OpenDelim(DelimToken),

    /// Close delimiter: `)`, `}`
    CloseDelim(DelimToken),

    /// Literal: `bool`, `integer`
    Literal(Lit),

    /// Identifier (include keyword)
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

pub struct Tokens<'a> {
    pub tokens: Vec<Token>,
    pub map: SymbolMap<'a>,
}
