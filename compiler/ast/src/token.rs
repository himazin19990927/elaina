use span::*;

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

    // `.`
    Dot,

    // `,`
    Comma,

    /// `->`
    Arrow,

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

    pub fn can_begin_expr(&self) -> bool {
        match self.kind {
            TokenKind::BinOp(BinOpToken::Minus) // unary minus
            | TokenKind::OpenDelim(_) // parensized expr, block
            | TokenKind::Literal(_) => true, // literal
            TokenKind::Ident(name) => ident_can_begin_expr(&name), // identifier
            _ => false,
        }
    }
}

pub fn ident_can_begin_expr(name: &Symbol) -> bool {
    !name.is_keyword()
        | [
            Kw::Let,
            Kw::If,
            Kw::True,
            Kw::False,
            Kw::Loop,
            Kw::Break,
            Kw::Continue,
            Kw::Return,
        ]
        .map(|k| k.into())
        .contains(name)
}

pub struct Tokens<'a> {
    pub tokens: Vec<Token>,
    pub map: SymbolMap<'a>,
}
