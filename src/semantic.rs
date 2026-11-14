use std::fmt;

use crate::ast::{AstNode, BinOp};
use crate::linemap::LineMap;
use crate::symbol_table::{DataType, SymbolTable};
use crate::token::Span;

#[derive(Debug)]
pub struct SemanticError {
    pub message: String,
    pub span: Span,
    pub line: usize,
    pub column: usize,
}

impl SemanticError {
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

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[ERRO SEMÂNTICO] {} (linha {}, coluna {})",
            self.message, self.line, self.column
        )
    }
}

impl std::error::Error for SemanticError {}

pub fn analyze(ast: &AstNode, symbols: &mut SymbolTable, linemap: &LineMap) -> Vec<SemanticError> {
    let mut errors = Vec::new();
    check_node(ast, symbols, linemap, &mut errors);
    errors
}

fn check_node(
    node: &AstNode,
    symbols: &mut SymbolTable,
    linemap: &LineMap,
    errors: &mut Vec<SemanticError>,
) {
    match node {
        AstNode::Program(items) => {
            for item in items {
                check_node(item, symbols, linemap, errors);
            }
        }

        AstNode::FunctionDef {
            name,
            return_type,
            body,
            ..
        } => {
            symbols.declare_at(*name, *return_type);
            for item in body {
                check_node(item, symbols, linemap, errors);
            }
        }

        AstNode::Declaration {
            var,
            data_type,
            init,
            span,
        } => {
            if let Some(expr) = init {
                check_node(expr, symbols, linemap, errors);

                if let Some(rhs) = infer_type(expr, symbols) {
                    if !compatible(*data_type, rhs) {
                        errors.push(SemanticError::new(
                            format!(
                                "incompatibilidade de tipos: '{}' declarado como {} mas recebe {}",
                                symbols.name_of(*var),
                                type_name(*data_type),
                                type_name(rhs),
                            ),
                            *span,
                            linemap,
                        ));
                    }
                }
            }
            symbols.declare_at(*var, *data_type);
        }

        AstNode::Assignment {
            target,
            value,
            span,
        } => {
            check_node(value, symbols, linemap, errors);

            if !symbols.is_declared_at(*target) {
                errors.push(SemanticError::new(
                    format!(
                        "variável '{}' utilizada sem declaração prévia",
                        symbols.name_of(*target)
                    ),
                    *span,
                    linemap,
                ));
                return;
            }

            if let (Some(lhs), Some(rhs)) =
                (symbols.data_type_at(*target), infer_type(value, symbols))
            {
                if !compatible(lhs, rhs) {
                    errors.push(SemanticError::new(
                        format!(
                            "não é possível atribuir {} a '{}' (tipo {})",
                            type_name(rhs),
                            symbols.name_of(*target),
                            type_name(lhs),
                        ),
                        *span,
                        linemap,
                    ));
                }
            }
        }

        AstNode::BinaryOp { left, right, .. } => {
            check_node(left, symbols, linemap, errors);
            check_node(right, symbols, linemap, errors);
        }

        AstNode::Identifier(idx, span) => {
            if !symbols.is_declared_at(*idx) {
                errors.push(SemanticError::new(
                    format!(
                        "variável '{}' utilizada sem declaração prévia",
                        symbols.name_of(*idx)
                    ),
                    *span,
                    linemap,
                ));
            }
        }

        AstNode::IntLiteral(..) | AstNode::FloatLiteral(..) | AstNode::StringLiteral(..) => {}
    }
}

fn infer_type(node: &AstNode, symbols: &SymbolTable) -> Option<DataType> {
    match node {
        AstNode::IntLiteral(..) => Some(DataType::Int),
        AstNode::FloatLiteral(..) => Some(DataType::Float),
        AstNode::StringLiteral(..) => Some(DataType::String),
        AstNode::Identifier(idx, _) => symbols.data_type_at(*idx),
        AstNode::BinaryOp {
            op, left, right, ..
        } => {
            let l = infer_type(left, symbols)?;
            let r = infer_type(right, symbols)?;
            Some(promote(l, r, *op))
        }
        // Declaração, atribuição, função não são expressões.
        _ => None,
    }
}

fn promote(a: DataType, b: DataType, _op: BinOp) -> DataType {
    use DataType::*;
    match (a, b) {
        (Double, _) | (_, Double) => Double,
        (Float, _) | (_, Float) => Float,
        (Long, _) | (_, Long) => Long,
        (String, String) => String,
        _ => Int,
    }
}

fn compatible(lhs: DataType, rhs: DataType) -> bool {
    use DataType::*;
    if lhs == rhs {
        return true;
    }
    matches!(
        (lhs, rhs),
        (Float, Int) | (Double, Int) | (Double, Float) | (Long, Int) | (Long, Short) | (Long, Char)
    )
}

fn type_name(t: DataType) -> &'static str {
    match t {
        DataType::Int => "int",
        DataType::Bool => "bool",
        DataType::Float => "float",
        DataType::Double => "double",
        DataType::Char => "char",
        DataType::Void => "void",
        DataType::Long => "long",
        DataType::Short => "short",
        DataType::Signed => "signed",
        DataType::Unsigned => "unsigned",
        DataType::String => "String",
        DataType::Unknown => "?",
    }
}
