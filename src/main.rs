mod ast;
mod exporter;
mod lexer;
mod linemap;
mod parser;
mod semantic;
mod symbol_table;
mod token;

use std::env;
use std::error::Error;
use std::fs;
use std::process::ExitCode;

use crate::parser::Parser;
use crate::symbol_table::SymbolTable;

fn main() -> ExitCode {
    // TODO: rever tratamento de erro
    let path = env::args().nth(1).expect("usage: compiler <file>");

    let src = match fs::read_to_string(&path) {
        Ok(value) => value,
        Err(err) => return exit_with_error("error reading '{path}'", err),
    };

    let mut symbol_table = SymbolTable::new();

    let token_list = match lexer::lex(&src, &mut symbol_table) {
        Ok(value) => value,
        Err(err) => return exit_with_error("", err),
    };

    let linemapping = linemap::LineMap::new(&src);

    let ast = match Parser::new(&token_list, &linemapping).parse_program() {
        Ok(value) => value,
        Err(err) => {
            println!("{err}");
            return ExitCode::FAILURE;
        }
    };
    let errors = semantic::analyze(&ast, &mut symbol_table, &linemapping);

    if errors.is_empty() {
        println!("\n[OK] análise semântica passou sem erros.");
    } else {
        for e in &errors {
            eprintln!("{e}");
        }
        return ExitCode::FAILURE;
    }

    token::display_tokens(&token_list);
    ast.print(&symbol_table);
    symbol_table.display();

    exporter::export_html(
        "result.html",
        &token_list,
        &ast,
        &symbol_table,
        &linemapping,
    )
    .unwrap();

    exporter::export_txt("result.txt", &token_list, &ast, &symbol_table, &linemapping).unwrap();

    ExitCode::SUCCESS
}

fn exit_with_error(msg: &str, err: impl Error) -> ExitCode {
    eprintln!("{msg}: {err}");
    ExitCode::FAILURE
}
