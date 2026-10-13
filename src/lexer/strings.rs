use super::{LexError, Lexer};
use crate::token::{Span, Token, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn lex_string(&mut self) -> Result<Token, LexError> {
        let start = self.position;
        self.bump();

        loop {
            match self.peek() {
                None => {
                    return Err(LexError::new(
                        "string não terminada",
                        self.line,
                        self.column,
                    ));
                }

                Some('"') => {
                    self.bump();
                    break;
                }

                Some('\\') => {
                    self.bump();
                    if self.peek().is_none() {
                        return Err(LexError::new(
                            "string não terminada após '\\'",
                            self.line,
                            self.column,
                        ));
                    }
                    self.bump();
                }

                Some('\n') => {
                    return Err(LexError::new(
                        "quebra de linha em string literal",
                        self.line,
                        self.column,
                    ));
                }

                Some(_) => {
                    self.bump();
                }
            }
        }

        let end = self.position;

        let inner_start = start + '"'.len_utf8();
        let inner_end = end - '"'.len_utf8();

        let inner = &self.src[inner_start..inner_end];
        let lexeme = &self.src[start..end];

        let kind = TokenKind::StringLiteral(inner.into());

        Ok(Token::new(kind, lexeme, Span { start, end }))
    }
}
