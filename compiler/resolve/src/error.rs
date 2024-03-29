use span::Span;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum NameResolutionError {
    #[error("unresolved name `{name}` at ({span:?}) was used")]
    UnresolvedNameUsed { name: String, span: Span },
    #[error("the name `{name}` is defined multiple times")]
    DefinedMultipleTimes { name: String },
}
