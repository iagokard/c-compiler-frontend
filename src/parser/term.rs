use super::{ParseError, Parser};
use crate::ast::{AstNode, BinOp};
use crate::token::{Span, TokenKind};

impl<'a> Parser<'a> {
    pub(super) fn parse_term(&mut self) -> Result<AstNode, ParseError> {
        let mut left = self.parse_factor()?;
        loop {
            let op = match self.peek() {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_factor()?;
            let span = Span {
                start: left.span().start,
                end: right.span().end,
            };
            left = AstNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }
}
