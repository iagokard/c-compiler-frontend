use super::{ParseError, Parser, token_to_type};
use crate::ast::AstNode;
use crate::token::TokenKind;

impl<'a> Parser<'a> {
    pub(super) fn parse_block_item(&mut self) -> Result<AstNode, ParseError> {
        if let Some(data_type) = token_to_type(self.peek()) {
            return self.parse_declaration(data_type);
        }
        if matches!(self.peek(), TokenKind::Identifier(_)) {
            return self.parse_assignment();
        }
        let token = self.peek_token();
        Err(self.error_at(
            format!(
                "comando inesperado dentro de bloco: {:?} ('{}')",
                token.kind, token.lexeme
            ),
            token.span,
        ))
    }
}
