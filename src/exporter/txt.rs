use crate::ast::AstNode;
use crate::linemap::LineMap;
use crate::symbol_table::SymbolTable;
use crate::token::Token;

use std::io;
use std::path::Path;

use super::{collect_symbols, token_name, token_value, truncate, write_output};

pub fn render(tokens: &[Token], ast: &AstNode, symbols: &SymbolTable, linemap: &LineMap) -> String {
    let mut output = String::new();

    output.push_str(&render_tokens(tokens));
    output.push_str("\nÁRVORE SINTÁTICA ABSTRATA\n");
    output.push_str(&ast.render(symbols));
    output.push('\n');
    output.push_str(&render_symbol_table(tokens, symbols, linemap));

    output
}

/// Gera e salva o relatório TXT no caminho informado.
pub fn export(
    path: impl AsRef<Path>,
    tokens: &[Token],
    ast: &AstNode,
    symbols: &SymbolTable,
    linemap: &LineMap,
) -> io::Result<()> {
    write_output(path, &render(tokens, ast, symbols, linemap))
}

fn render_tokens(tokens: &[Token]) -> String {
    const NUM_W: usize = 3;
    const CAT_W: usize = 20;
    const VAL_W: usize = 26;
    const TOK_W: usize = 18;
    const SPAN_W: usize = 10;

    const WIDTHS: [usize; 5] = [NUM_W + 2, CAT_W + 2, VAL_W + 2, TOK_W + 2, SPAN_W + 2];
    let total: usize = WIDTHS.iter().sum::<usize>() + 4;

    let border = |left: &str, middle: &str, right: &str| {
        let mut result = String::from(left);
        for (index, width) in WIDTHS.iter().enumerate() {
            if index > 0 {
                result.push_str(middle);
            }
            result.push_str(&"═".repeat(*width));
        }
        result.push_str(right);
        result
    };

    let mut output = String::new();
    output.push_str(&border("╔", "╦", "╗"));
    output.push('\n');

    let title = "LISTA DE TOKENS";
    let padding = total.saturating_sub(title.chars().count());
    let left_padding = padding / 2;
    let right_padding = padding - left_padding;
    output.push_str(&format!(
        "║{}{}{}║\n",
        " ".repeat(left_padding),
        title,
        " ".repeat(right_padding)
    ));
    output.push_str(&border("╠", "╬", "╣"));
    output.push('\n');

    output.push_str(&format!(
        "║ {:<nw$} ║ {:<cw$} ║ {:<vw$} ║ {:<tw$} ║ {:^sw$} ║\n",
        "#",
        "Categoria",
        "Valor / Lexema",
        "Token",
        "Span",
        nw = NUM_W,
        cw = CAT_W,
        vw = VAL_W,
        tw = TOK_W,
        sw = SPAN_W,
    ));
    output.push_str(&border("╠", "╬", "╣"));
    output.push('\n');

    for (index, token) in tokens.iter().enumerate() {
        let category = truncate(token.kind.category(), CAT_W);
        let value = truncate(&token_value(token), VAL_W);
        let name = truncate(&token_name(&token.kind), TOK_W);
        output.push_str(&format!(
            "║ {:<nw$} ║ {:<cw$} ║ {:<vw$} ║ {:<tw$} ║ {:>4}..{:<4} ║\n",
            index,
            category,
            value,
            name,
            token.span.start,
            token.span.end,
            nw = NUM_W,
            cw = CAT_W,
            vw = VAL_W,
            tw = TOK_W,
        ));
    }

    output.push_str(&border("╚", "╩", "╝"));
    output.push_str(&format!("\nTotal: {} tokens.\n", tokens.len()));
    output
}

fn render_symbol_table(tokens: &[Token], symbols: &SymbolTable, linemap: &LineMap) -> String {
    const HEADERS: [&str; 5] = ["#", "Nome", "Tipo", "Linha", "Coluna"];
    const RIGHT_ALIGNED: [bool; 5] = [true, false, false, true, true];

    let rows = collect_symbols(tokens, symbols, linemap);
    if rows.is_empty() {
        return "(Tabela de símbolos vazia)\n".to_string();
    }

    let values: Vec<[String; 5]> = rows
        .iter()
        .map(|row| {
            [
                row.index.to_string(),
                row.name.clone(),
                row.data_type.clone(),
                row.line.to_string(),
                row.column.to_string(),
            ]
        })
        .collect();

    let mut widths = [0usize; 5];
    for (width, header) in widths.iter_mut().zip(HEADERS.iter()) {
        *width = header.chars().count();
    }
    for row in &values {
        for (width, cell) in widths.iter_mut().zip(row.iter()) {
            *width = (*width).max(cell.chars().count());
        }
    }

    let format_cell = |cell: &str, width: usize, right: bool| {
        if right {
            format!("{cell:>width$}")
        } else {
            format!("{cell:<width$}")
        }
    };
    let border = |left: &str, middle: &str, right: &str| {
        let mut result = String::from(left);
        for (index, width) in widths.iter().enumerate() {
            if index > 0 {
                result.push_str(middle);
            }
            result.push_str(&"═".repeat(*width + 2));
        }
        result.push_str(right);
        result
    };

    let total_width: usize = widths.iter().map(|width| *width + 2).sum::<usize>() + 4;
    let title = "TABELA DE SÍMBOLOS";
    let padding = total_width.saturating_sub(title.chars().count());
    let left_padding = padding / 2;
    let right_padding = padding - left_padding;

    let mut output = String::new();
    output.push_str(&border("╔", "╦", "╗"));
    output.push('\n');
    output.push_str(&format!(
        "║{}{}{}║\n",
        " ".repeat(left_padding),
        title,
        " ".repeat(right_padding)
    ));
    output.push_str(&border("╠", "╬", "╣"));
    output.push('\n');

    output.push('║');
    for (index, (width, header)) in widths.iter().zip(HEADERS.iter()).enumerate() {
        output.push_str(&format!(
            " {} ║",
            format_cell(header, *width, RIGHT_ALIGNED[index])
        ));
    }
    output.push('\n');
    output.push_str(&border("╠", "╬", "╣"));
    output.push('\n');

    for row in &values {
        output.push('║');
        for (index, (width, cell)) in widths.iter().zip(row.iter()).enumerate() {
            output.push_str(&format!(
                " {} ║",
                format_cell(cell, *width, RIGHT_ALIGNED[index])
            ));
        }
        output.push('\n');
    }

    output.push_str(&border("╚", "╩", "╝"));
    output.push_str(&format!("\nTotal: {} símbolo(s).\n", rows.len()));
    output
}
