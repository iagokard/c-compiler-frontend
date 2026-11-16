mod assignment;
mod block_item;
mod declaration;
mod error;
mod expression;
mod factor;
mod function;
mod term;
mod top_level;

use crate::ast::AstNode;
use crate::linemap::LineMap;
use crate::symbol_table::DataType;
use crate::token::{Span, Token, TokenKind};

pub use error::ParseError;

pub struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
    linemap: &'a LineMap,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], linemap: &'a LineMap) -> Self {
        Self {
            tokens,
            position: 0,
            linemap,
        }
    }

    fn peek(&self) -> &TokenKind {
        &self.tokens[self.position].kind
    }

    fn peek_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn peek_n(&self, n: usize) -> Option<&TokenKind> {
        self.tokens.get(self.position + n).map(|t| &t.kind)
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.position];
        self.position += 1;
        token
    }

    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(self.peek()) == std::mem::discriminant(kind)
    }

    fn accept(&mut self, kind: &TokenKind) -> Option<&Token> {
        if self.check(kind) {
            Some(self.advance())
        } else {
            None
        }
    }

    fn expect(&mut self, kind: &TokenKind, what: &str) -> Result<&Token, ParseError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            let token = self.peek_token();
            Err(self.error_at(
                format!(
                    "esperado {}, encontrado {:?} ('{}')",
                    what, token.kind, token.lexeme
                ),
                token.span,
            ))
        }
    }

    pub fn parse_program(&mut self) -> Result<AstNode, ParseError> {
        let mut items = Vec::new();
        while !matches!(self.peek(), TokenKind::EOF) {
            items.push(self.parse_top_level()?);
        }
        Ok(AstNode::Program(items))
    }

    fn error_at(&self, message: impl Into<String>, span: Span) -> ParseError {
        ParseError::new(message, span, self.linemap)
    }
}

fn token_to_type(kind: &TokenKind) -> Option<DataType> {
    Some(match kind {
        TokenKind::Int => DataType::Int,
        TokenKind::Float => DataType::Float,
        TokenKind::Double => DataType::Double,
        TokenKind::Char => DataType::Char,
        TokenKind::Void => DataType::Void,
        TokenKind::Short => DataType::Short,
        TokenKind::Bool => DataType::Bool,
        _ => return None,
    })
}
