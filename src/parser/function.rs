use super::{ParseError, Parser};
use crate::ast::AstNode;
use crate::symbol_table::DataType;
use crate::token::{Span, TokenKind};

impl<'a> Parser<'a> {
    pub(super) fn parse_function(&mut self, return_type: DataType) -> Result<AstNode, ParseError> {
        let type_token = self.advance();
        let start = type_token.span.start;

        let id_token = self.advance();
        let name = match &id_token.kind {
            TokenKind::Identifier(idx) => *idx,
            _ => unreachable!("parse_top_level já garantiu Identifier via lookahead"),
        };

        self.expect(&TokenKind::LParen, "'('")?;
        self.expect(&TokenKind::RParen, "')'")?; // `int main()`

        self.expect(&TokenKind::LBrace, "'{'")?;
        let mut body = Vec::new();
        while !matches!(self.peek(), TokenKind::RBrace | TokenKind::EOF) {
            body.push(self.parse_block_item()?);
        }
        let rbrace = self.expect(&TokenKind::RBrace, "'}'")?;

        Ok(AstNode::FunctionDef {
            name,
            return_type,
            body,
            span: Span {
                start,
                end: rbrace.span.end,
            },
        })
    }
}
