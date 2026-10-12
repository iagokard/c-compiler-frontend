mod lexer;
mod token;

use std::env;
use std::error::Error;
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    // TODO: rever tratamento de erro
    let path = env::args().nth(1).expect("usage: compiler <file>");

    let src = match fs::read_to_string(&path) {
        Ok(value) => value,
        Err(err) => return exit_with_error("error reading '{path}'", err),
    };

    let token_list = match lexer::lex(&src) {
        Ok(value) => value,
        Err(err) => return exit_with_error("", err),
    };

    for token in token_list {
        println!("{:?}", token);
    }

    ExitCode::SUCCESS
}

fn exit_with_error(msg: &str, err: impl Error) -> ExitCode {
    eprintln!("{msg}: {err}");
    ExitCode::FAILURE
}
