use super::{LexError, Lexer};
use crate::token::{Span, Token, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn lex_number(&mut self) -> Result<Token, LexError> {
        let start = self.position;

        let mut saw_dot = false;

        loop {
            match self.peek() {
                Some(c) if c.is_ascii_digit() => {
                    self.bump();
                }
                Some('.') if !saw_dot => {
                    saw_dot = true;
                    self.bump();
                }
                Some('.') => {
                    let message = format!(
                        "número inválido '{}': mais de um ponto decimal",
                        &self.src[start..self.position]
                    );

                    return Err(LexError::new(message, self.line, self.column));
                }
                _ => break,
            }
        }

        let lexeme = &self.src[start..self.position];
        let kind = if saw_dot {
            TokenKind::FloatLiteral(lexeme.parse().unwrap_or(0.0))
        } else {
            TokenKind::IntLiteral(lexeme.parse().unwrap_or(0))
        };

        Ok(Token::new(
            kind,
            lexeme,
            Span {
                start,
                end: self.position,
            },
        ))
    }
}
