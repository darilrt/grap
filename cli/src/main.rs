use crate::parser::Parser;

mod lexer;
mod parser;

fn main() {
    let input = "\"Hello, \" + \"World!\"\na + b";

    let mut parser = Parser::new(input);

    println!("{:?}", parser.parse());
}
