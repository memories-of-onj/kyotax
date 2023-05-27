mod error;
mod lexer;
mod scanner;
mod token;

use lexer::Lexer;
use token::{Token, TokenKind};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("./examples/greet.kx")?;
    let mut lexer = Lexer::new(&input);
    let mut tokens: Vec<Token> = vec![];

    while tokens.len() == 0 || tokens.last().unwrap().kind != TokenKind::Eof {
        tokens.push(lexer.read()?);
    }

    println!("{:?}", tokens);

    Ok(())
}
