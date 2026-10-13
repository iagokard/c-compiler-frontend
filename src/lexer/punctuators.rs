use super::{LexError, Lexer};
use crate::token::{Span, Token, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn lex_punctuator(&mut self) -> Result<Token, LexError> {
        let start = self.position;
        let start_line = self.line;
        let start_column = self.column;

        let first = self
            .bump()
            .expect("lex_punctuator chamada sem caractere disponível");

        let kind = match first {
            '.' => {
                if self.peek() == Some('.') && self.peek_next() == Some('.') {
                    self.bump_two();
                    TokenKind::Ellipsis
                } else {
                    TokenKind::Dot
                }
            }
            '<' => {
                if self.peek() == Some('<') {
                    self.bump();
                    if self.eat('=') {
                        TokenKind::ShiftLeftAssign
                    } else {
                        TokenKind::ShiftLeft
                    }
                } else if self.eat('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '>' => {
                if self.peek() == Some('>') {
                    self.bump();
                    if self.eat('=') {
                        TokenKind::ShiftRightAssign
                    } else {
                        TokenKind::ShiftRight
                    }
                } else if self.eat('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }

            '+' => {
                if self.eat('+') {
                    TokenKind::Increment
                } else if self.eat('=') {
                    TokenKind::PlusAssign
                } else {
                    TokenKind::Plus
                }
            }
            '-' => {
                if self.eat('-') {
                    TokenKind::Decrement
                } else if self.eat('>') {
                    TokenKind::Arrow
                } else if self.eat('=') {
                    TokenKind::MinusAssign
                } else {
                    TokenKind::Minus
                }
            }
            '*' => {
                if self.eat('=') {
                    TokenKind::StarAssign
                } else {
                    TokenKind::Star
                }
            }
            '/' => {
                if self.eat('=') {
                    TokenKind::SlashAssign
                } else {
                    TokenKind::Slash
                }
            }
            '%' => {
                if self.eat('=') {
                    TokenKind::PercentAssign
                } else {
                    TokenKind::Percent
                }
            }
            '=' => {
                if self.eat('=') {
                    TokenKind::Equal
                } else {
                    TokenKind::Assign
                }
            }
            '!' => {
                if self.eat('=') {
                    TokenKind::NotEqual
                } else {
                    TokenKind::LogicalNot
                }
            }
            '&' => {
                if self.eat('&') {
                    TokenKind::LogicalAnd
                } else if self.eat('=') {
                    TokenKind::BitwiseAndAssign
                } else {
                    TokenKind::BitwiseAnd
                }
            }
            '|' => {
                if self.eat('|') {
                    TokenKind::LogicalOr
                } else if self.eat('=') {
                    TokenKind::BitwiseOrAssign
                } else {
                    TokenKind::BitwiseOr
                }
            }
            '^' => {
                if self.eat('=') {
                    TokenKind::BitwiseXorAssign
                } else {
                    TokenKind::BitwiseXor
                }
            }

            '~' => TokenKind::BitwiseNot,
            '#' => TokenKind::Hash,
            '?' => TokenKind::Question,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,

            other => {
                return Err(LexError::new(
                    format!("caractere inesperado: '{other}'"),
                    start_line,
                    start_column,
                ));
            }
        };

        let end = self.position;
        let lexeme = &self.src[start..end];

        Ok(Token::new(kind, lexeme, Span { start, end }))
    }
}
