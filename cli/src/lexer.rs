use lib::{
    rules::Rule,
    token::{Token, TokenKind},
    tokenizer::Tokenizer,
};

pub struct Ident;

impl Rule for Ident {
    fn parse(&self, state: &mut Tokenizer) -> Option<Token> {
        state.push();
        state.ignore_whitespace();

        let peek = state.peek()?;

        if !peek.is_alphabetic() && peek != '_' {
            state.pop();
            return None;
        }

        let ident = state.consume_while(|c| c.is_alphanumeric() || c == '_');
        if ident.is_empty() {
            state.pop();
            None
        } else {
            state.commit();
            Some(Token {
                kind: if state.is_keyword(&ident) {
                    TokenKind::Keyword
                } else {
                    TokenKind::Ident
                },
                literal: ident,
                location: state.location.clone(),
            })
        }
    }
}

pub struct Number;

impl Rule for Number {
    fn parse(&self, state: &mut Tokenizer) -> Option<Token> {
        state.push();
        state.ignore_whitespace();

        let number = state.consume_while(|c| c.is_numeric());
        if number.is_empty() {
            state.pop();
            None
        } else {
            state.commit();
            Some(Token {
                kind: TokenKind::Value,
                literal: number,
                location: state.location.clone(),
            })
        }
    }
}

pub struct Str;

impl Rule for Str {
    fn parse(&self, state: &mut Tokenizer) -> Option<Token> {
        state.push();
        state.ignore_whitespace();

        let peek = state.peek();

        if peek.is_none() {
            state.pop();
            return None;
        }

        let c = peek.unwrap();

        if c != '"' {
            state.pop();
            return None;
        }

        state.consume();

        let string = state.consume_while(|c| c != '"');

        if state.consume() != Some('"') {
            state.pop();
            return None;
        }

        state.commit();
        Some(Token {
            kind: TokenKind::Value,
            literal: string,
            location: state.location.clone(),
        })
    }
}
