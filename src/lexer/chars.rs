use super::{LexError, Lexer};
use crate::token::{Span, Token, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn lex_char(&mut self) -> Result<Token, LexError> {
        let start = self.position;

        self.bump();

        let value = match self.peek() {
            None => return Err(LexError::new("char não terminado", self.line, self.column)),

            Some('\'') => {
                return Err(LexError::new("char literal vazio", self.line, self.column));
            }

            Some('\n') => {
                return Err(LexError::new(
                    "quebra de linha em char literal",
                    self.line,
                    self.column,
                ));
            }

            Some('\\') => self.lex_escape()?,

            Some(other) => {
                self.bump();
                other
            }
        };

        if self.peek() != Some('\'') {
            return Err(LexError::new(
                "esperado `'` de fechamento",
                self.line,
                self.column,
            ));
        }
        self.bump();

        let end = self.position;
        let lexeme = &self.src[start..end];
        let span = Span { start, end };

        Ok(Token::new(TokenKind::CharLiteral(value), lexeme, span))
    }

    fn lex_escape(&mut self) -> Result<char, LexError> {
        self.bump();

        match self.peek() {
            None => Err(LexError::new(
                "escape não terminado",
                self.line,
                self.column,
            )),
            Some('n') => {
                self.bump();
                Ok('\n')
            }
            Some('t') => {
                self.bump();
                Ok('\t')
            }
            Some('r') => {
                self.bump();
                Ok('\r')
            }
            Some('0') => {
                self.bump();
                Ok('\0')
            }
            Some('\\') => {
                self.bump();
                Ok('\\')
            }
            Some('\'') => {
                self.bump();
                Ok('\'')
            }
            Some('"') => {
                self.bump();
                Ok('"')
            }
            Some('a') => {
                self.bump();
                Ok('\x07')
            }
            Some('b') => {
                self.bump();
                Ok('\x08')
            }
            Some('f') => {
                self.bump();
                Ok('\x0C')
            }
            Some('v') => {
                self.bump();
                Ok('\x0B')
            }
            Some(other) => Err(LexError::new(
                format!("escape desconhecido: '\\{other}'"),
                self.line,
                self.column,
            )),
        }
    }
}
