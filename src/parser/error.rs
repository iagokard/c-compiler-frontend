use crate::linemap::LineMap;
use crate::token::Span;

use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
    pub line: usize,
    pub column: usize,
}

impl ParseError {
    pub fn new(message: impl Into<String>, span: Span, linemap: &LineMap) -> Self {
        let (line, column) = linemap.location(span.start);

        Self {
            message: message.into(),
            span,
            line,
            column,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ERRO SINTÁTICO] {} (linha {}, coluna {}, span {}-{})",
            self.message, self.line, self.column, self.span.start, self.span.end
        )
    }
}

impl std::error::Error for ParseError {}
