use super::{ParseError, Parser};
use crate::ast::AstNode;
use crate::token::{Span, TokenKind};

impl<'a> Parser<'a> {
    pub(super) fn parse_assignment(&mut self) -> Result<AstNode, ParseError> {
        let id_token = self.advance();
        let start = id_token.span.start;

        let target = match &id_token.kind {
            TokenKind::Identifier(idx) => *idx,
            _ => unreachable!(),
        };

        self.expect(&TokenKind::Assign, "'='")?;

        let value = Box::new(self.parse_expr()?);
        let semicolon = self.expect(&TokenKind::Semicolon, "';'")?;
        Ok(AstNode::Assignment {
            target,
            value,
            span: Span {
                start,
                end: semicolon.span.end,
            },
        })
    }
}
