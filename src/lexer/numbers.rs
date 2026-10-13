use super::{LexError, Lexer};
use crate::token::{Span, Token, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumberBase {
    Decimal,
    Octal,
    Hexadecimal,
    Binary,
}

impl NumberBase {
    fn valid_digit(self, c: char) -> bool {
        match self {
            NumberBase::Decimal => c.is_ascii_digit(),
            NumberBase::Octal => matches!(c, '0'..='7'),
            NumberBase::Hexadecimal => c.is_ascii_hexdigit(),
            NumberBase::Binary => matches!(c, '0' | '1'),
        }
    }
}

impl<'a> Lexer<'a> {
    pub(super) fn lex_number(&mut self) -> Result<Token, LexError> {
        let start = self.position;

        if self.peek() == Some('.') {
            return self.lex_float(NumberBase::Decimal, start);
        }

        let (base, prefix_len) = self.detect_number_base();

        for _ in 0..prefix_len {
            self.bump();
        }

        if self.number_has_float_marker(base) {
            let float_base = if base == NumberBase::Octal {
                NumberBase::Decimal
            } else {
                base
            };
            self.lex_float(float_base, start)
        } else {
            self.lex_integer(base, start)
        }
    }

    fn detect_number_base(&self) -> (NumberBase, usize) {
        if self.peek() != Some('0') {
            return (NumberBase::Decimal, 0);
        }

        match self.peek_next() {
            Some('x') | Some('X') => (NumberBase::Hexadecimal, 2),
            Some('b') | Some('B') => (NumberBase::Binary, 2),
            _ => (NumberBase::Octal, 0),
        }
    }

    /// Only decimal and hexadecimal floating constants exist in C.
    /// A leading `0` (Octal here) that turns out to have a float marker is
    /// scanned as decimal — it is really a decimal float like `0.5`/`0e1`.
    fn number_has_float_marker(&self, base: NumberBase) -> bool {
        let (scan_base, exponent_marker) = match base {
            NumberBase::Decimal => (NumberBase::Decimal, ['e', 'E']),
            NumberBase::Hexadecimal => (NumberBase::Hexadecimal, ['p', 'P']),
            NumberBase::Octal => (NumberBase::Decimal, ['e', 'E']),
            NumberBase::Binary => return false,
        };

        let mut offset = 0;
        loop {
            let c = match self.peek_ahead(offset) {
                Some(c) => c,
                None => return false,
            };

            match c {
                '.' => return true,
                c if exponent_marker.contains(&c) => return true,
                '\'' => offset += 1,
                c if scan_base.valid_digit(c) => offset += 1,
                _ => return false,
            }
        }
    }

    fn lex_integer(&mut self, base: NumberBase, start: usize) -> Result<Token, LexError> {
        self.consume_digits(base)?;

        let digits_end = self.position;

        self.lex_number_suffix(false)?;
        self.reject_bad_suffix_continuation()?;

        let end = self.position;
        let raw = &self.src[start..end];
        let cleaned: String = self.src[start..digits_end]
            .chars()
            .filter(|&c| c != '\'')
            .collect();

        let value = match base {
            NumberBase::Hexadecimal => i64::from_str_radix(
                cleaned.trim_start_matches("0x").trim_start_matches("0X"),
                16,
            ),
            NumberBase::Binary => {
                i64::from_str_radix(cleaned.trim_start_matches("0b").trim_start_matches("0B"), 2)
            }
            NumberBase::Octal => {
                let digits = &cleaned;
                // NOTE: keep the leading `0` in the string — this is also
                // what makes the literal `0` parse correctly.
                i64::from_str_radix(digits, 8)
            }
            NumberBase::Decimal => cleaned.parse::<i64>(),
        }
        .map_err(|_| {
            LexError::new(
                format!("literal inteiro inválido: '{raw}'"),
                self.line,
                self.column,
            )
        })?;

        Ok(Token::new(
            TokenKind::IntLiteral(value),
            raw,
            Span { start, end },
        ))
    }

    fn lex_float(&mut self, base: NumberBase, start: usize) -> Result<Token, LexError> {
        self.consume_digits_optional(base);
        self.lex_float_rest(base, start)
    }

    fn lex_float_rest(&mut self, base: NumberBase, start: usize) -> Result<Token, LexError> {
        if self.eat('.') {
            self.consume_digits_optional(base);
        }

        match base {
            NumberBase::Hexadecimal => {
                // Hexadecimal floating constants MUST have a binary exponent.
                if !(self.eat('p') || self.eat('P')) {
                    return Err(LexError::new(
                        "constante flutuante hexadecimal requer expoente 'p'",
                        self.line,
                        self.column,
                    ));
                }
                self.eat_any(&['+', '-']);
                self.consume_exponent_digits()?;
            }
            NumberBase::Decimal => {
                if self.eat('e') || self.eat('E') {
                    self.eat_any(&['+', '-']);
                    self.consume_exponent_digits()?;
                }
            }
            _ => unreachable!("apenas decimal/hexadecimal viram floats"),
        }

        let body_end = self.position;

        self.lex_number_suffix(true)?;
        self.reject_bad_suffix_continuation()?;

        let end = self.position;
        let raw = &self.src[start..end];
        let cleaned: String = self.src[start..body_end]
            .chars()
            .filter(|&c| c != '\'')
            .collect();

        let value = if base == NumberBase::Hexadecimal {
            Self::parse_hex_float(&cleaned).ok_or_else(|| {
                LexError::new(
                    format!("literal flutuante inválido: '{raw}'"),
                    self.line,
                    self.column,
                )
            })?
        } else {
            cleaned.parse::<f64>().map_err(|_| {
                LexError::new(
                    format!("literal flutuante inválido: '{raw}'"),
                    self.line,
                    self.column,
                )
            })?
        };

        Ok(Token::new(
            TokenKind::FloatLiteral(value),
            raw,
            Span { start, end },
        ))
    }

    fn parse_hex_float(cleaned: &str) -> Option<f64> {
        let body = cleaned
            .trim_start_matches("0x")
            .trim_start_matches("0X")
            .trim_end_matches(['f', 'F', 'l', 'L']);

        let (sig, exponent) = body
            .find(['p', 'P'])
            .map(|i| (&body[..i], &body[i + 1..]))?;
        let exponent: i32 = exponent.parse().ok()?;

        let mut value = 0.0f64;
        let mut frac = 1.0f64;
        let mut past_dot = false;

        for c in sig.chars() {
            if c == '.' {
                past_dot = true;
                continue;
            }
            let digit = c.to_digit(16)? as f64;
            if past_dot {
                frac /= 16.0;
                value += digit * frac;
            } else {
                value = value * 16.0 + digit;
            }
        }

        Some(value * 2f64.powi(exponent))
    }

    fn consume_exponent_digits(&mut self) -> Result<(), LexError> {
        if !self.peek().is_some_and(|c| c.is_ascii_digit()) {
            return Err(LexError::new(
                "esperado dígito no expoente",
                self.line,
                self.column,
            ));
        }
        self.consume_digits(NumberBase::Decimal)
    }

    fn consume_digits(&mut self, base: NumberBase) -> Result<(), LexError> {
        let first = self.peek();
        let valid = |c: char| base.valid_digit(c);

        let mut consumed_any = false;

        while let Some(c) = self.peek() {
            if valid(c) {
                self.bump();
                consumed_any = true;
            } else if c == '\'' && consumed_any && self.peek_next().is_some_and(valid) {
                self.bump();
            } else {
                break;
            }
        }

        if !consumed_any && first.is_some_and(|c| c != '.') {
            return Err(LexError::new(
                format!("esperado dígito na base {base:?}"),
                self.line,
                self.column,
            ));
        }

        Ok(())
    }
    fn consume_digits_optional(&mut self, base: NumberBase) -> bool {
        let valid = |c: char| base.valid_digit(c);
        let mut consumed_any = false;

        while let Some(c) = self.peek() {
            if valid(c) {
                self.bump();
                consumed_any = true;
            } else if c == '\'' && consumed_any && self.peek_next().is_some_and(valid) {
                self.bump();
            } else {
                break;
            }
        }

        consumed_any
    }
    /// C23 integer suffix grammar:
    ///   [u|U] (l|L|ll|LL|wb|WB)  or  (l|L|ll|LL|wb|WB) [u|U]  or  (u|U alone)
    fn lex_number_suffix(&mut self, is_float: bool) -> Result<(), LexError> {
        if is_float {
            self.lex_float_suffix()?;
        }

        let mut seen_unsigned = false;
        let mut seen_size = false;

        loop {
            match self.peek() {
                Some('u' | 'U') if !seen_unsigned => {
                    self.bump();
                    seen_unsigned = true;
                }
                Some(c @ ('l' | 'L')) if !seen_size => {
                    self.bump();

                    let twin = if c == 'l' { 'l' } else { 'L' };
                    if self.peek() == Some(twin) {
                        self.bump();
                    }
                    seen_size = true;
                }
                Some(c @ ('w' | 'W')) if !seen_size => {
                    self.bump();

                    match self.peek() {
                        Some(next_c @ ('b' | 'B')) => {
                            if (c == 'w') != (next_c == 'b') {
                                return Err(LexError::new(
                                    "sufixo 'w' deve ser seguido de 'b' de mesma caixa",
                                    self.line,
                                    self.column,
                                ));
                            }

                            self.bump();
                            seen_size = true;
                        }
                        _ => {
                            return Err(LexError::new(
                                "sufixo 'w' deve ser seguido de 'b' ou 'B'",
                                self.line,
                                self.column,
                            ));
                        }
                    }
                }
                _ => break,
            }
        }

        Ok(())
    }

    fn lex_float_suffix(&mut self) -> Result<(), LexError> {
        const SUFFIXES: &[&str] = &[
            "f128x", "f64x", "f32x", "f128", "f64", "f32", "f16", "df", "DF", "dd", "DD", "dl",
            "DL", "f", "F", "l", "L",
        ];

        for s in SUFFIXES {
            if self.matches_at_cursor(s) {
                for _ in 0..s.len() {
                    self.bump();
                }
                return Ok(());
            }
        }
        Ok(())
    }

    /// After a complete number, no identifier/number character may follow —
    /// otherwise the source was not a valid C constant (`1u5`, `1.5ff`,
    /// `123abc`, `0b1.1`). A `.` is also always invalid after a number
    fn reject_bad_suffix_continuation(&mut self) -> Result<(), LexError> {
        if let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' || c == '\'' || c == '.' {
                return Err(LexError::new(
                    format!("sufixo inválido na constante numérica (próximo a '{c}')"),
                    self.line,
                    self.column,
                ));
            }
        }
        Ok(())
    }
}
