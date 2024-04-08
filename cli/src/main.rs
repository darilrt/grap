use crate::parser::Parser;

mod ast;
mod lexer;
mod parser;

fn main() {
    let input = "a : b = 10\n2";

    let mut parser = Parser::new(input);

    println!("{:?}", parser.parse());
}
