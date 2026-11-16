use super::{ParseError, Parser};
use crate::ast::AstNode;
use crate::symbol_table::DataType;
use crate::token::{Span, TokenKind};

impl<'a> Parser<'a> {
    pub(super) fn parse_declaration(&mut self, data_type: DataType) -> Result<AstNode, ParseError> {
        let type_token = self.advance();
        let start = type_token.span.start;

        let id_token = self.advance();
        let symbol_index = match &id_token.kind {
            TokenKind::Identifier(idx) => *idx,
            _ => {
                let span = id_token.span; // ← copia e encerra o empréstimo
                return Err(self.error_at("esperado identificador após tipo", span));
            }
        };

        let init = if self.accept(&TokenKind::Assign).is_some() {
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };

        let semicolon = self.expect(&TokenKind::Semicolon, "';'")?;
        Ok(AstNode::Declaration {
            var: symbol_index,
            data_type,
            init,
            span: Span {
                start,
                end: semicolon.span.end,
            },
        })
    }
}
