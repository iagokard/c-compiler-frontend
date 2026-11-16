use super::{ParseError, Parser};
use crate::ast::AstNode;
use crate::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn parse_factor(&mut self) -> Result<AstNode, ParseError> {
        let token = self.peek_token().clone();
        match token.kind {
            TokenKind::IntLiteral(n) => {
                self.advance();
                Ok(AstNode::IntLiteral(n, token.span))
            }
            TokenKind::FloatLiteral(n) => {
                self.advance();
                Ok(AstNode::FloatLiteral(n, token.span))
            }
            TokenKind::StringLiteral(s) => {
                self.advance();
                Ok(AstNode::StringLiteral(s, token.span))
            }
            TokenKind::Identifier(idx) => {
                self.advance();
                Ok(AstNode::Identifier(idx, token.span))
            }
            TokenKind::LParen => {
                self.advance();
                let e = self.parse_expr()?;
                self.expect(&TokenKind::RParen, "')'")?;
                Ok(e)
            }
            _ => Err(self.error_at(
                format!("esperado expressão, encontrado {:?}", token.kind),
                token.span,
            )),
        }
    }
}
