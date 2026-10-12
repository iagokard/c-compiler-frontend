#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Inline,
    Int,
    Long,
    Register,
    Restrict,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,

    Alignas,
    Alignof,
    Atomic,
    Bool,
    Complex,
    Generic,
    Imaginary,
    Noreturn,
    StaticAssert,
    ThreadLocal,

    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    CharLiteral(char),
    StringLiteral(String),

    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Increment,
    Decrement,
    Hash,

    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    LogicalAnd,
    LogicalOr,
    LogicalNot,

    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    ShiftLeft,
    ShiftRight,

    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    ShiftLeftAssign,
    ShiftRightAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,

    Arrow,
    Dot,
    Question,
    Colon,
    Comma,
    Ellipsis,

    Semicolon,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    EOF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: impl Into<String>, span: Span) -> Token {
        Self {
            kind,
            lexeme: lexeme.into(),
            span,
        }
    }
}

pub fn get_identifier_kind(lexeme: &str) -> TokenKind {
    match keyword_token(lexeme) {
        Some(value) => value,
        None => TokenKind::Identifier(lexeme.into()),
    }
}

pub fn keyword_token(lexeme: &str) -> Option<TokenKind> {
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
