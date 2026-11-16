use crate::linemap::LineMap;
use crate::symbol_table::SymbolTable;
use crate::token::{Token, TokenKind};

use std::io;
use std::path::Path;

pub mod html;
pub mod txt;

pub use html::{export as export_html, render as render_html};
pub use txt::{export as export_txt, render as render_txt};

#[derive(Debug, Clone)]
pub(super) struct SymbolRow {
    pub index: usize,
    pub name: String,
    pub data_type: String,
    pub line: usize,
    pub column: usize,
    pub declared: bool,
}

pub(super) fn collect_symbols(
    tokens: &[Token],
    symbols: &SymbolTable,
    linemap: &LineMap,
) -> Vec<SymbolRow> {
    let count = tokens
        .iter()
        .filter_map(|token| match &token.kind {
            TokenKind::Identifier(index) => Some(*index + 1),
            _ => None,
        })
        .max()
        .unwrap_or(0);

    (0..count)
        .map(|index| {
            let first_occurrence = tokens.iter().find(|token| {
                matches!(&token.kind, TokenKind::Identifier(token_index) if *token_index == index)
            });
            let (line, column) = first_occurrence
                .map(|token| linemap.location(token.span.end))
                .unwrap_or((0, 0));

            SymbolRow {
                index,
                name: symbols.name_of(index).to_string(),
                data_type: symbols
                    .data_type_at(index)
                    .map(|data_type| format!("{data_type:?}"))
                    .unwrap_or_else(|| "-".to_string()),
                line,
                column,
                declared: symbols.is_declared_at(index),
            }
        })
        .collect()
}

pub(super) fn token_name(kind: &TokenKind) -> String {
    format!("{kind:?}")
        .split('(')
        .next()
        .unwrap_or("Token")
        .to_string()
}

pub(super) fn token_value(token: &Token) -> String {
    let value = token.kind.display_value();
    if value.is_empty() {
        format!("{:?}", token.lexeme)
    } else {
        value
    }
}

pub(super) fn truncate(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        value.to_string()
    } else {
        let mut truncated: String = value.chars().take(max_chars.saturating_sub(1)).collect();
        truncated.push('…');
        truncated
    }
}

pub(super) fn html_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

/// Atalho para gravar uma string no caminho informado, mantendo o tratamento
/// de erro padrão de `std::fs`.
pub(super) fn write_output(path: impl AsRef<Path>, contents: &str) -> io::Result<()> {
    std::fs::write(path, contents)
}
