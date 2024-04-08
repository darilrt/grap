use std::fmt::Debug;

use lib::token::Token;

pub enum ExprKind {
    Number(i32),
    Ident(String),
    String(String),
    Binary(Token, Box<ExprKind>, Box<ExprKind>),
}

impl Debug for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprKind::Number(n) => write!(f, "Number({})", n),
            ExprKind::Ident(s) => write!(f, "Ident({})", s),
            ExprKind::String(s) => write!(f, "String({})", s),
            ExprKind::Binary(token, lhs, rhs) => {
                write!(f, "Binary({:?}, {:?}, {:?})", token.literal, lhs, rhs)
            }
        }
    }
}

#[derive(Debug)]
pub enum StmtKind {
    Expr(Box<ExprKind>),
    Decl(Token, Token, Box<ExprKind>),
    FuncDecl(Token, Box<ExprKind>),
}

#[derive(Debug)]
pub struct Ast {
    pub stmts: Vec<StmtKind>,
}
