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

    Identifier(usize),
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

    fn render_row(&self, index: usize) -> String {
        let value = self.kind.display_value();
        let shown = if value.is_empty() {
            format!("{:?}", self.lexeme) // ex.: "\"+\"" vira "+"
        } else {
            value
        };
        let category = self.kind.category();

        format!(
            "│ {:<3} │ {:<20} │ {:<26} │ {:<18} │ {:>4}..{:<4} │",
            index,
            category,
            shown,
            token_name(&self.kind),
            self.span.start,
            self.span.end,
        )
    }
}

impl TokenKind {
    pub fn display_value(&self) -> String {
        match self {
            TokenKind::Identifier(idx) => format!("[{idx}]"),
            TokenKind::IntLiteral(n) => format!("{n}"),
            TokenKind::FloatLiteral(n) => format!("{n:?}"),
            TokenKind::CharLiteral(c) => format!("{c:?}"),
            TokenKind::StringLiteral(s) => format!("{s:?}"),
            TokenKind::EOF => "<EOF>".to_string(),
            _ => String::new(), // o lexema cobre
        }
    }

    pub fn category(&self) -> &'static str {
        use TokenKind::*;
        match self {
            // ── palavras reservadas (C89/C99/C11/C23) ──
            Auto | Break | Case | Char | Const | Continue | Default | Do | Double | Else | Enum
            | Extern | Float | For | Goto | If | Inline | Int | Long | Register | Restrict
            | Return | Short | Signed | Sizeof | Static | Struct | Switch | Typedef | Union
            | Unsigned | Void | Volatile | While | Alignas | Alignof | Atomic | Bool | Complex
            | Generic | Imaginary | Noreturn | StaticAssert | ThreadLocal => "Palavra Reservada",

            // ── literais e identificador ──
            Identifier(_) => "Identificador",
            IntLiteral(_) => "Literal Inteiro",
            FloatLiteral(_) => "Literal Float",
            CharLiteral(_) => "Literal Char",
            StringLiteral(_) => "Literal String",

            // ── operadores aritméticos ──
            Plus | Minus | Star | Slash | Percent | Increment | Decrement => "Operador Aritmético",

            // ── relacionais ──
            Equal | NotEqual | Less | Greater | LessEqual | GreaterEqual => "Operador Relacional",

            // ── lógicos ──
            LogicalAnd | LogicalOr | LogicalNot => "Operador Lógico",

            // ── bitwise ──
            BitwiseAnd | BitwiseOr | BitwiseXor | BitwiseNot | ShiftLeft | ShiftRight => {
                "Operador Bitwise"
            }

            // ── atribuição ──
            Assign | PlusAssign | MinusAssign | StarAssign | SlashAssign | PercentAssign
            | ShiftLeftAssign | ShiftRightAssign | BitwiseAndAssign | BitwiseOrAssign
            | BitwiseXorAssign => "Operador Atribuição",

            // ── pontuação (inclui Hash, Arrow, Dot e os demais) ──
            Hash | Arrow | Dot | Question | Colon | Comma | Ellipsis | Semicolon | LParen
            | RParen | LBracket | RBracket | LBrace | RBrace => "Pontuação",

            // ── fim ──
            EOF => "Fim de Arquivo",
        }
    }
}

fn token_name(kind: &TokenKind) -> String {
    let dbg = format!("{kind:?}");
    match dbg.find('(') {
        Some(i) => dbg[..i].to_string(),
        None => dbg,
    }
}

pub fn display_tokens(tokens: &[Token]) {
    // ── larguras de conteúdo ──────────────────────────────
    const NUM_W: usize = 3;
    const CAT_W: usize = 20;
    const VAL_W: usize = 26;
    const TOK_W: usize = 18;
    const SPAN_W: usize = 10; // "1234..5678"

    // larguras *entre pipes* = conteúdo + 2 (padding)
    const NW: usize = NUM_W + 2; // 5
    const CW: usize = CAT_W + 2; // 22
    const VW: usize = VAL_W + 2; // 28
    const TW: usize = TOK_W + 2; // 20
    const SW: usize = SPAN_W + 2; // 12

    const CELLS: [usize; 5] = [NW, CW, VW, TW, SW];
    const TOTAL: usize = NW + CW + VW + TW + SW + 4; // +4 pipes internos

    // gera qualquer linha de borda (topo, meio, base)
    let border = |l: &str, m: &str, r: &str| -> String {
        let mut s = String::from(l);
        for (i, w) in CELLS.iter().enumerate() {
            if i > 0 {
                s.push_str(m);
            }
            s.push_str(&"═".repeat(*w));
        }
        s.push_str(r);
        s
    };

    // ── topo ──
    println!("{}", border("╔", "╦", "╗"));

    // ── título centralizado ──
    let title = "LISTA DE TOKENS";
    let pad = TOTAL.saturating_sub(title.chars().count());
    let lp = pad / 2;
    let rp = pad - lp;
    println!("║{}{}{}║", " ".repeat(lp), title, " ".repeat(rp));

    // ── separador ──
    println!("{}", border("╠", "╬", "╣"));

    // ── cabeçalho ──
    println!(
        "║ {:<nw$} ║ {:<cw$} ║ {:<vw$} ║ {:<tw$} ║ {:^sw$} ║",
        "#",
        "Categoria",
        "Valor / Lexema",
        "Token",
        "Span",
        nw = NUM_W,
        cw = CAT_W,
        vw = VAL_W,
        tw = TOK_W,
        sw = SPAN_W
    );

    // ── separador ──
    println!("{}", border("╠", "╬", "╣"));

    // ── linhas ──
    for (i, tok) in tokens.iter().enumerate() {
        let raw = tok.kind.display_value();
        let shown = if raw.is_empty() {
            format!("{:?}", tok.lexeme)
        } else {
            raw
        };
        let cat = truncate(tok.kind.category(), CAT_W);
        let name = truncate(&token_name(&tok.kind), TOK_W);
        let val = truncate(&shown, VAL_W);

        println!(
            "║ {:<nw$} ║ {:<cw$} ║ {:<vw$} ║ {:<tw$} ║ {:>4}..{:<4} ║",
            i,
            cat,
            val,
            name,
            tok.span.start,
            tok.span.end,
            nw = NUM_W,
            cw = CAT_W,
            vw = VAL_W,
            tw = TOK_W
        );
    }

    // ── base ──
    println!("{}", border("╚", "╩", "╝"));
    println!("Total: {} tokens.", tokens.len());
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
        out.push('…');
        out
    }
}
