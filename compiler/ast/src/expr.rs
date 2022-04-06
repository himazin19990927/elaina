use crate::{block::*, lit::*, op::*};
use span::symbol::{Ident, Symbol};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// A binary operation: `a + b`, "a * b"
    Binary {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },

    /// A unary operation: `-x`
    Unary { op: UnOp, expr: Box<Expr> },

    /// If expression: `if a == b { 0 } else { 1 }`
    If {
        cond: Box<Expr>,
        then: Box<Block>,
        else_opt: Option<Box<Expr>>,
    },

    /// Loop expression: `loop { block }`
    Loop { block: Box<Block> },

    /// Break expression: `break;`, `break expr;`
    Break { expr: Option<Box<Expr>> },

    /// Continue expression: `continue;`, `continue expr;`
    Continue { expr: Option<Box<Expr>> },

    /// Block expression: `{ 0 }`, `{let a = 1; a}`
    Block { block: Box<Block> },

    /// Assign expression: `a = 1`
    Assign { lhs: Box<Expr>, rhs: Box<Expr> },

    /// A literal in place of an expression: `1`
    Lit { lit: Lit },

    /// A identifier such as variables, functions, etx: `foo`, `bar`
    /// This will be deleted in the near future.
    Ident { ident: Symbol },

    /// A path such as variables, functions, etx: `foo`, `bar`
    Path(Path),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub ident: Ident,
}
