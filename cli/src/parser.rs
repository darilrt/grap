use std::borrow::Borrow;

use lib::{
    rules::*,
    token::{Token, TokenKind},
    tokenizer::Tokenizer,
};

use crate::ast::*;
use crate::lexer::{Ident, Number, Str};

pub struct Parser {
    pub state: Tokenizer,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        Parser {
            state: Tokenizer::new(
                input,
                vec![
                    "if", "else", "while", "for", "return", "break", "continue", "type", "fn",
                ],
            ),
        }
    }

    pub fn parse(&mut self) -> Ast {
        let mut stmts = Vec::new();
        while self.state.pos < self.state.input.len() {
            let stmt = self.parse_stmt();

            if stmt.is_none() {
                break;
            }

            stmts.push(stmt.unwrap());

            if let Some(c) = self.state.input.chars().nth(self.state.pos) {
                if c != '\n' {
                    eprintln!("Expected newline");
                    break;
                }
            }
        }
        Ast { stmts }
    }

    // stmt = keyword_stmt | ident_stmt
    fn parse_stmt(&mut self) -> Option<StmtKind> {
        let token = (Ident {}).parse(&mut self.state);

        if token.is_none() {
            return self.parse_expr_stmt();
        }

        let token = token.unwrap();

        match token.kind {
            TokenKind::Keyword => self.parse_keyword_stmt(token),
            TokenKind::Ident => self.parse_ident_stmt(token),
            _ => Some(StmtKind::Expr(Box::new(ExprKind::Ident(token.literal)))),
        }
    }

    fn parse_keyword_stmt(&mut self, token: Token) -> Option<StmtKind> {
        None
    }

    // ident_stmt = ident, (':', decl_stmt | '::', def_stmt)
    fn parse_ident_stmt(&mut self, token: Token) -> Option<StmtKind> {
        if let Some(_) = lex!(":").parse(&mut self.state) {
            return self.parse_decl_stmt(token);
        }

        if let Some(_) = lex!("::").parse(&mut self.state) {
            return self.parse_def_stmt(token);
        }

        Some(StmtKind::Expr(Box::new(ExprKind::Ident(token.literal))))
    }

    // def_stmt = ident, '::', ('fn', fn_decl | 'type', type_decl)
    fn parse_def_stmt(&mut self, token: Token) -> Option<StmtKind> {
        let keyword = (Ident {}).parse(&mut self.state).unwrap();

        match keyword.literal.as_str() {
            // "fn" => self.parse_fn_decl(token),
            // "type" => self.parse_type_decl(token),
            _ => None,
        }
    }

    // decl_stmt = ident, ':', type, ['=', expr]
    fn parse_decl_stmt(&mut self, token: Token) -> Option<StmtKind> {
        let type_ = (Ident {}).parse(&mut self.state).unwrap();

        let expr = if let Some(_) = lex!("=").parse(&mut self.state) {
            self.parse_expr()
        } else {
            None
        };

        Some(StmtKind::Decl(token, type_, Box::new(expr?)))
    }

    // expr_stmt = expr
    fn parse_expr_stmt(&mut self) -> Option<StmtKind> {
        let expr = self.parse_expr();

        if expr.is_none() {
            return None;
        }

        Some(StmtKind::Expr(Box::new(expr?)))
    }

    // expr = add_expr
    fn parse_expr(&mut self) -> Option<ExprKind> {
        self.parse_add_expr()
    }

    // add_expr = mul_expr, ('+', add_expr | '-', add_expr)
    fn parse_add_expr(&mut self) -> Option<ExprKind> {
        let mut lhs = self.parse_mul_expr()?;

        while let Some(token) = (lex!("+" | "-")).parse(&mut self.state) {
            let rhs = self.parse_mul_expr()?;
            lhs = ExprKind::Binary(token, Box::new(lhs), Box::new(rhs));
        }

        Some(lhs)
    }

    // mul_expr = primary_expr, ('*', mul_expr | '/', mul_expr)
    fn parse_mul_expr(&mut self) -> Option<ExprKind> {
        let mut lhs = self.parse_primary_expr()?;

        while let Some(token) = (lex!("*" | "/")).parse(&mut self.state) {
            let rhs = self.parse_primary_expr()?;
            lhs = ExprKind::Binary(token, Box::new(lhs), Box::new(rhs));
        }

        Some(lhs)
    }

    // primary_expr = number | ident | '(', expr, ')'
    fn parse_primary_expr(&mut self) -> Option<ExprKind> {
        if let Some(token) = (Number {}).parse(&mut self.state) {
            return Some(ExprKind::Number(token.literal.parse().unwrap()));
        }

        if let Some(token) = (Ident {}).parse(&mut self.state) {
            return Some(ExprKind::Ident(token.literal));
        }

        if let Some(token) = (Str {}).parse(&mut self.state) {
            return Some(ExprKind::String(token.literal));
        }

        if let Some(_) = lex!("(").parse(&mut self.state) {
            let expr = self.parse_expr()?;
            if let Some(_) = lex!(")").parse(&mut self.state) {
                return Some(expr);
            }
        }

        None
    }
}
