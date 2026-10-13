mod chars;
mod error;
mod identifiers;
mod numbers;
mod strings;

use crate::token::Token;
pub use error::LexError;

pub fn lex(src: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(src);
    lexer.tokenize()
}

struct Lexer<'a> {
    src: &'a str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Lexer<'a> {
        Self {
            src,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.src[self.position..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.src[self.position..].chars().nth(1)
    }

    fn bump(&mut self) -> Option<char> {
        let character = self.peek()?;

        self.position += character.len_utf8();

        if character == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(character)
    }

    fn bump_two(&mut self) -> Option<char> {
        self.bump();
        self.bump()
    }

    fn eat(&mut self, character: char) -> bool {
        if self.peek() == Some(character) {
            self.bump();
            return true;
        }

        false
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        loop {
            self.skip_trivia()?;

            let c = match self.peek() {
                Some(value) => value,
                None => break,
            };

            let next_c = self.peek_next();

            let token = match c {
                c if c.is_ascii_alphabetic() || c == '_' => self.lex_identifier()?,
                c if c.is_ascii_digit() => self.lex_number()?,
                '.' if next_c.is_some_and(|next| next.is_ascii_digit()) => self.lex_number()?,
                '"' => self.lex_string()?,
                '\'' => self.lex_char()?,
                _ => {
                    return Err(LexError::new("Caracter inválido", self.line, self.column));
                }
            };

            tokens.push(token);
        }

        Ok(tokens)
    }

    fn skip_trivia(&mut self) -> Result<(), LexError> {
        loop {
            self.consume_while(|c| matches!(c, ' ' | '\t' | '\r' | '\n'));

            match (self.peek(), self.peek_next()) {
                (Some('/'), Some('/')) => self.skip_line_comment(),
                (Some('/'), Some('*')) => self.skip_block_comment()?,
                _ => break,
            }
        }

        Ok(())
    }

    fn skip_line_comment(&mut self) {
        self.consume_while(|c| c != '\n');
    }

    fn skip_block_comment(&mut self) -> Result<(), LexError> {
        let start_line = self.line;
        let start_column = self.column;

        loop {
            self.consume_while(|c| c != '*');

            if self.peek().is_none() {
                return Err(LexError::new(
                    "comentário de bloco não terminado",
                    start_line,
                    start_column,
                ));
            }

            self.bump();

            if self.eat('/') {
                return Ok(());
            }
        }
    }

    fn consume_while<F: Fn(char) -> bool>(&mut self, func: F) {
        while let Some(c) = self.peek() {
            if !func(c) {
                break;
            }
            self.bump();
        }
    }
}
