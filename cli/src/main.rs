use lib::*;

mod lexer;

struct Parser {
    state: ParserState,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Parser {
            state: ParserState {
                input: input.to_string(),
                pos: 0,
            },
        }
    }

    pub fn parse(&mut self) -> Option<String> {
        let ident = lit("var");
        let eq = lit("=");
        let number = or(lit("0"), lit("1"));
        let rule = and(and(ident, eq), number);

        rule.parse(&mut self.state)
    }
}

fn main() {
    let input = "var = 1";

    let mut parser = Parser::new(input);

    println!("{:?}", parser.parse());
}
