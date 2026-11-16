use super::{ParseError, Parser, token_to_type};
use crate::ast::AstNode;
use crate::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn parse_top_level(&mut self) -> Result<AstNode, ParseError> {
        if let Some(data_type) = token_to_type(self.peek()) {
            let is_function = matches!(self.peek_n(1), Some(TokenKind::Identifier(_)))
                && matches!(self.peek_n(2), Some(TokenKind::LParen));
            return if is_function {
                self.parse_function(data_type)
            } else {
                self.parse_declaration(data_type)
            };
        }

        if matches!(self.peek(), TokenKind::Identifier(_)) {
            return self.parse_assignment();
        }

        let token = self.peek_token();
        Err(self.error_at(
            format!("token inesperado no topo: {:?}", token.kind),
            token.span,
        ))
    }
}
