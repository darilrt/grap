use crate::lexer::*;
use lib::*;

#[derive(Debug)]
pub enum Ast {
    Unknown,
    Number(i32),
    String(String),
    Ident(String),
    Add(Box<Ast>, Box<Ast>),
    Sub(Box<Ast>, Box<Ast>),
    Mul(Box<Ast>, Box<Ast>),
    Div(Box<Ast>, Box<Ast>),
    Expr(Box<Ast>),
    Program(Vec<Ast>),
}

pub struct Parser {
    state: ParserState,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Parser {
            state: ParserState::new(input),
        }
    }

    pub fn parse(&mut self) -> Ast {
        let mut program = Vec::new();

        loop {
            self.state.ignore_whitespace();

            if self.state.eof() {
                break;
            }

            program.push(self.parse_stmt());
        }

        Ast::Program(program)
    }

    pub fn parse_stmt(&mut self) -> Ast {
        self.parse_expr()
    }

    pub fn parse_expr(&mut self) -> Ast {
        Ast::Expr(Box::new(self.parse_add()))
    }

    // add := mul (('+' | '-') add)
    pub fn parse_add(&mut self) -> Ast {
        let mut lhs = self.parse_mul();

        if let Some(op) = exec(or(lit("+"), lit("-")), &mut self.state) {
            let rhs = self.parse_add();

            if op == "+" {
                lhs = Ast::Add(Box::new(lhs), Box::new(rhs));
            } else {
                lhs = Ast::Sub(Box::new(lhs), Box::new(rhs));
            }
        }

        lhs
    }

    // term := factor (('*' | '/') mul)
    pub fn parse_mul(&mut self) -> Ast {
        let mut lhs = self.parse_factor();

        if let Some(op) = exec(or(lit("*"), lit("/")), &mut self.state) {
            let rhs = self.parse_mul();

            if op == "*" {
                lhs = Ast::Mul(Box::new(lhs), Box::new(rhs));
            } else {
                lhs = Ast::Div(Box::new(lhs), Box::new(rhs));
            }
        }

        lhs
    }

    // factor := number | ident
    pub fn parse_factor(&mut self) -> Ast {
        if let Some(literal) = exec(Number {}, &mut self.state) {
            return Ast::Number(literal.parse().unwrap());
        }

        if let Some(literal) = exec(Str {}, &mut self.state) {
            return Ast::String(literal);
        }

        if let Some(literal) = exec(Ident {}, &mut self.state) {
            return Ast::Ident(literal);
        }
        Ast::Unknown
    }
}
