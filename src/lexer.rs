use crate::token::{self, Span, Token, TokenKind};

pub fn lex(src: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize()?;

    Ok(tokens)
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

    fn skip_trivia(&mut self) -> Result<(), LexError> {
        loop {
            match self.peek() {
                Some(' ' | '\t' | '\r' | '\n') => {
                    self.bump();
                }
                Some('/') if self.peek_next() == Some('/') => {
                    self.skip_line_comment();
                }
                Some('/') if self.peek_next() == Some('*') => {
                    self.skip_block_comment()?;
                }
                _ => break,
            }
        }

        Ok(())
    }

    fn skip_line_comment(&mut self) {
        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.bump();
        }
    }

    fn skip_block_comment(&mut self) -> Result<(), LexError> {
        let start_line = self.line;
        let start_col = self.column;

        loop {
            match self.peek() {
                None => {
                    return Err(LexError {
                        message: "comentário de bloco não terminado".into(),
                        line: start_line,
                        column: start_col,
                    });
                }
                Some('*') if self.peek_next() == Some('/') => {
                    self.bump_two();
                    return Ok(());
                }
                Some(_) => {
                    self.bump();
                }
            }
        }
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
                '.' if next_c.is_some_and(|next| next.is_ascii_digit()) => self.lex_number()?,
                c if c.is_ascii_digit() => self.lex_number()?,
                _ => {
                    return Err(LexError {
                        message: "Caracter inválido".into(),
                        line: self.line,
                        column: self.column,
                    });
                }
            };

            tokens.push(token);
        }

        Ok(tokens)
    }

    fn lex_identifier(&mut self) -> Result<Token, LexError> {
        let start = self.position;

        self.bump();

        while matches!(
            self.peek(),
            Some(c) if c.is_ascii_alphanumeric() || c == '_'
        ) {
            self.bump();
        }

        let lexeme = &self.src[start..self.position];
        let kind = token::get_identifier_kind(lexeme);

        Ok(Token::new(
            kind,
            lexeme,
            Span {
                start,
                end: self.position,
            },
        ))
    }

    fn lex_number(&mut self) -> Result<Token, LexError> {
        let start = self.position;

        let mut is_float = false;

        while matches!(
            self.peek(),
            Some(c) if c.is_ascii_digit() || c == '.'
        ) {
            if self.peek() == Some('.') {
                if is_float == true {
                    return Err(LexError {
                        message: "Número inválido".into(),
                        line: self.line,
                        column: self.column,
                    });
                }

                is_float = true;
            }

            self.bump();
        }

        let lexeme = &self.src[start..self.position];
        let kind = match is_float {
            true => TokenKind::FloatLiteral(lexeme.parse::<f64>().unwrap()),
            false => TokenKind::IntLiteral(lexeme.parse::<i64>().unwrap()),
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

#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ERRO LÉXICO] {} (linha {}, coluna {})",
            self.message, self.line, self.column
        )
    }
}

impl std::error::Error for LexError {}
