use super::{ParseError, Parser};
use crate::ast::{AstNode, BinOp};
use crate::token::{Span, TokenKind};

impl<'a> Parser<'a> {
    pub(super) fn parse_expr(&mut self) -> Result<AstNode, ParseError> {
        let mut left = self.parse_term()?;
        loop {
            let operation = match self.peek() {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => break,
            };

            self.advance();
            let right = self.parse_term()?;
            let span = Span {
                start: left.span().start,
                end: right.span().end,
            };

            left = AstNode::BinaryOp {
                op: operation,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }
}
