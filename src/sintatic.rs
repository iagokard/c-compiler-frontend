use crate::ast::{AstNode, BinOp};
use crate::linemap::LineMap;
use crate::symbol_table::DataType;
use crate::token::{Span, Token, TokenKind};

use std::fmt;

pub struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
    linemap: &'a LineMap,
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
    pub line: usize,
    pub column: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], linemap: &'a LineMap) -> Self {
        Self {
            tokens,
            position: 0,
            linemap,
        }
    }

    fn peek(&self) -> &TokenKind {
        &self.tokens[self.position].kind
    }

    fn peek_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn peek_n(&self, n: usize) -> Option<&TokenKind> {
        self.tokens.get(self.position + n).map(|t| &t.kind)
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.position];
        self.position += 1;
        token
    }

    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(self.peek()) == std::mem::discriminant(kind)
    }

    fn accept(&mut self, kind: &TokenKind) -> Option<&Token> {
        if self.check(kind) {
            Some(self.advance())
        } else {
            None
        }
    }

    fn expect(&mut self, kind: &TokenKind, what: &str) -> Result<&Token, ParseError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            let token = self.peek_token();
            Err(self.error_at(
                format!(
                    "esperado {}, encontrado {:?} ('{}')",
                    what, token.kind, token.lexeme
                ),
                token.span,
            ))
        }
    }

    pub fn parse_program(&mut self) -> Result<AstNode, ParseError> {
        let mut items = Vec::new();
        while !matches!(self.peek(), TokenKind::EOF) {
            items.push(self.parse_top_level()?);
        }
        Ok(AstNode::Program(items))
    }

    fn parse_top_level(&mut self) -> Result<AstNode, ParseError> {
        if let Some(data_type) = token_to_type(self.peek()) {
            let is_function = matches!(self.peek_n(1), Some(TokenKind::Identifier(_)))
                && matches!(self.peek_n(2), Some(TokenKind::LParen));
            return if is_function {
                self.parse_function(data_type)
            } else {
                self.parse_declaration(data_type)
            };
        }

        if matches!(self.peek(), TokenKind::Identifier(_)) {
            return self.parse_assignment();
        }

        let token = self.peek_token();
        Err(self.error_at(
            format!("token inesperado no topo: {:?}", token.kind),
            token.span,
        ))
    }

    fn parse_function(&mut self, return_type: DataType) -> Result<AstNode, ParseError> {
        let type_token = self.advance();
        let start = type_token.span.start;

        let id_token = self.advance();
        let name = match &id_token.kind {
            TokenKind::Identifier(idx) => *idx,
            _ => unreachable!("parse_top_level já garantiu Identifier via lookahead"),
        };

        self.expect(&TokenKind::LParen, "'('")?;
        self.expect(&TokenKind::RParen, "')'")?; // `int main()`

        self.expect(&TokenKind::LBrace, "'{'")?;
        let mut body = Vec::new();
        while !matches!(self.peek(), TokenKind::RBrace | TokenKind::EOF) {
            body.push(self.parse_block_item()?);
        }
        let rbrace = self.expect(&TokenKind::RBrace, "'}'")?;

        Ok(AstNode::FunctionDef {
            name,
            return_type,
            body,
            span: Span {
                start,
                end: rbrace.span.end,
            },
        })
    }

    fn parse_block_item(&mut self) -> Result<AstNode, ParseError> {
        if let Some(data_type) = token_to_type(self.peek()) {
            return self.parse_declaration(data_type);
        }
        if matches!(self.peek(), TokenKind::Identifier(_)) {
            return self.parse_assignment();
        }
        let token = self.peek_token();
        Err(self.error_at(
            format!(
                "comando inesperado dentro de bloco: {:?} ('{}')",
                token.kind, token.lexeme
            ),
            token.span,
        ))
    }

    fn parse_declaration(&mut self, data_type: DataType) -> Result<AstNode, ParseError> {
        let type_token = self.advance();
        let start = type_token.span.start;

        let id_token = self.advance();
        let symbol_index = match &id_token.kind {
            TokenKind::Identifier(idx) => *idx,
            _ => {
                let span = id_token.span; // ← copia e encerra o empréstimo
                return Err(self.error_at("esperado identificador após tipo", span));
            }
        };

        let init = if self.accept(&TokenKind::Assign).is_some() {
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };

        let semicolon = self.expect(&TokenKind::Semicolon, "';'")?;
        Ok(AstNode::Declaration {
            var: symbol_index,
            data_type,
            init,
            span: Span {
                start,
                end: semicolon.span.end,
            },
        })
    }

    fn parse_assignment(&mut self) -> Result<AstNode, ParseError> {
        let id_token = self.advance();
        let start = id_token.span.start;

        let target = match &id_token.kind {
            TokenKind::Identifier(idx) => *idx,
            _ => unreachable!(),
        };

        self.expect(&TokenKind::Assign, "'='")?;

        let value = Box::new(self.parse_expr()?);
        let semicolon = self.expect(&TokenKind::Semicolon, "';'")?;
        Ok(AstNode::Assignment {
            target,
            value,
            span: Span {
                start,
                end: semicolon.span.end,
            },
        })
    }

    fn parse_expr(&mut self) -> Result<AstNode, ParseError> {
        let mut left = self.parse_term()?;
        loop {
            let operation = match self.peek() {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => break,
            };

            self.advance();
            let right = self.parse_term()?;
            let span = Span {
                start: node_start(&left),
                end: node_end(&right),
            };

            left = AstNode::BinaryOp {
                op: operation,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<AstNode, ParseError> {
        let mut left = self.parse_factor()?;
        loop {
            let op = match self.peek() {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                _ => break,
            };
            self.advance();
            let right = self.parse_factor()?;
            let span = Span {
                start: node_start(&left),
                end: node_end(&right),
            };
            left = AstNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<AstNode, ParseError> {
        let token = self.peek_token().clone();
        match token.kind {
            TokenKind::IntLiteral(n) => {
                self.advance();
                Ok(AstNode::IntLiteral(n, token.span))
            }
            TokenKind::FloatLiteral(n) => {
                self.advance();
                Ok(AstNode::FloatLiteral(n, token.span))
            }
            TokenKind::StringLiteral(s) => {
                self.advance();
                Ok(AstNode::StringLiteral(s, token.span))
            }
            TokenKind::Identifier(idx) => {
                self.advance();
                Ok(AstNode::Identifier(idx, token.span))
            }
            TokenKind::LParen => {
                self.advance();
                let e = self.parse_expr()?;
                self.expect(&TokenKind::RParen, "')'")?;
                Ok(e)
            }
            _ => Err(self.error_at(
                format!("esperado expressão, encontrado {:?}", token.kind),
                token.span,
            )),
        }
    }

    fn error_at(&self, message: impl Into<String>, span: Span) -> ParseError {
        ParseError::new(message, span, self.linemap)
    }
}

fn token_to_type(kind: &TokenKind) -> Option<DataType> {
    Some(match kind {
        TokenKind::Int => DataType::Int,
        TokenKind::Float => DataType::Float,
        TokenKind::Double => DataType::Double,
        TokenKind::Char => DataType::Char,
        TokenKind::Void => DataType::Void,
        _ => return None,
    })
}

fn node_start(n: &AstNode) -> usize {
    match n {
        AstNode::IntLiteral(_, s)
        | AstNode::FloatLiteral(_, s)
        | AstNode::StringLiteral(_, s)
        | AstNode::Identifier(_, s) => s.start,
        AstNode::BinaryOp { span, .. }
        | AstNode::Assignment { span, .. }
        | AstNode::Declaration { span, .. }
        | AstNode::FunctionDef { span, .. } => span.start,
        AstNode::Program(_) => 0,
    }
}

impl ParseError {
    pub fn new(message: impl Into<String>, span: Span, linemap: &LineMap) -> Self {
        let (line, column) = linemap.location(span.start);
        Self {
            message: message.into(),
            span,
            line,
            column,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ERRO SINTÁTICO] {} (linha {}, coluna {})",
            self.message, self.line, self.column
        )
    }
}

impl std::error::Error for ParseError {}

fn node_end(n: &AstNode) -> usize {
    match n {
        AstNode::IntLiteral(_, s)
        | AstNode::FloatLiteral(_, s)
        | AstNode::StringLiteral(_, s)
        | AstNode::Identifier(_, s) => s.end,
        AstNode::BinaryOp { span, .. }
        | AstNode::Assignment { span, .. }
        | AstNode::Declaration { span, .. }
        | AstNode::FunctionDef { span, .. } => span.end,
        AstNode::Program(_) => 0,
    }
}
