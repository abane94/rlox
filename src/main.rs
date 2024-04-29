#![feature(let_chains)]

use std::error::Error;
use std::{process::exit, path::PathBuf};
use std::{fs, env};
use std::io::{self, Write};

use scanner::Scanner;
use token_type::{TokenType, Token, Literal};
use types::expr::Expr;
use types::operations::Print;

mod token_type;
mod scanner;
mod test;
mod types;
mod parser;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64);
    } else if args.len() == 2 {
        let file_path = &args[1];
        println!("In file {}", file_path);
        let _ = run_file(file_path.into());
    } else {
        run_prompt();
    }
}

fn run_file(path: PathBuf) -> Result<(), Box<dyn Error>> {
    let script = fs::read_to_string(path)?;
    run(&script);
    Ok(())
}

fn run_prompt() {
    println!("Running Prompt mode");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();

        io::stdin().read_line(&mut line).expect("failed to readline");
        run(&line);
    }
}

fn run(source: &str) {
    // let tokens: Vec<char> = source.chars().collect();

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    for (i, token) in tokens.iter().enumerate() {
        println!("{}: {}", i, token);
    }

}

fn print_ast() {
    let expr = Expr::Binary { left:
        Box::new(Expr::Unary {
            operator: Token::new(TokenType::MINUS, "-".to_string(), Literal::None(), 1),
            right: Box::new(Expr::Literal { value: Literal::Number(123.0) })
        }),
        operator: Token::new(TokenType::STAR, "*".to_string(), Literal::None(), 1),
        right: Box::new(Expr::Grouping { expression:
            Box::new(Expr::Literal { value: Literal::Number(45.67) })
        }) };

    let representation = expr.print();
    println!("{}", representation);
}
