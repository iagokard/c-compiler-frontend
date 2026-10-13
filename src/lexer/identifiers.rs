use super::{LexError, Lexer};
use crate::token::{Span, Token, TokenKind};

impl<'a> Lexer<'a> {
    pub(super) fn lex_identifier(&mut self) -> Result<Token, LexError> {
        let start = self.position;

        self.bump();

        while matches!(
            self.peek(),
            Some(c) if c.is_ascii_alphanumeric() || c == '_'
        ) {
            self.bump();
        }

        let lexeme = &self.src[start..self.position];
        let kind = Self::get_identifier_kind(lexeme);

        Ok(Token::new(
            kind,
            lexeme,
            Span {
                start,
                end: self.position,
            },
        ))
    }

    fn get_identifier_kind(lexeme: &str) -> TokenKind {
        match Self::keyword_token(lexeme) {
            Some(value) => value,
            None => TokenKind::Identifier(lexeme.into()),
        }
    }

    fn keyword_token(lexeme: &str) -> Option<TokenKind> {
        use TokenKind::*;

        Some(match lexeme {
            "auto" => Auto,
            "break" => Break,
            "case" => Case,
            "char" => Char,
            "continue" => Continue,
            "default" => Default,
            "do" => Do,
            "double" => Double,
            "else" => Else,
            "enum" => Enum,
            "extern" => Extern,
            "float" => Float,
            "for" => For,
            "goto" => Goto,
            "if" => If,
            "inline" => Inline,
            "int" => Int,
            "long" => Long,
            "register" => Register,
            "restrict" => Restrict,
            "return" => Return,
            "short" => Short,
            "signed" => Signed,
            "sizeof" => Sizeof,
            "static" => Static,
            "struct" => Struct,
            "switch" => Switch,
            "typedef" => Typedef,
            "union" => Union,
            "unsigned" => Unsigned,
            "void" => Void,
            "volatile" => Volatile,
            "while" => While,

            "_Alignas" => Alignas,
            "_Alignof" => Alignof,
            "_Atomic" => Atomic,
            "_Bool" => Bool,
            "_Complex" => Complex,
            "_Generic" => Generic,
            "_Imaginary" => Imaginary,
            "_Noreturn" => Noreturn,
            "_Static_assert" => StaticAssert,
            "_Thread_local" => ThreadLocal,

            _ => return None,
        })
    }
}
