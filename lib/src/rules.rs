use crate::token::{Token, TokenKind};
use crate::tokenizer::Tokenizer;

pub trait Rule {
    fn parse(&self, state: &mut Tokenizer) -> Option<Token>;
}

#[derive(Debug)]
pub struct Lit(pub &'static str);

impl Rule for Lit {
    fn parse(&self, state: &mut Tokenizer) -> Option<Token> {
        let pos = state.pos;
        state.ignore_whitespace();
        for c in self.0.chars() {
            if let Some(nc) = state.input.chars().nth(state.pos) {
                if c != nc {
                    state.pos = pos;
                    return None;
                }
            } else {
                state.pos = pos;
                return None;
            }
            state.pos += 1;
        }

        Some(Token {
            kind: if state.is_keyword(&self.0) {
                TokenKind::Keyword
            } else {
                TokenKind::Unknown
            },
            literal: self.0.to_owned(),
            location: state.location.clone(),
        })
    }
}

#[derive(Debug)]
pub struct Or<T, U>(pub T, pub U);

impl<T: Rule, U: Rule> Rule for Or<T, U> {
    fn parse(&self, state: &mut Tokenizer) -> Option<Token> {
        let pos = state.pos;
        state.ignore_whitespace();
        if let Some(res) = self.0.parse(state) {
            state.ignore_whitespace();
            return Some(res);
        }
        state.pos = pos;
        self.1.parse(state)
    }
}

#[derive(Debug)]
pub struct And<T, U>(pub T, pub U);

impl<T: Rule, U: Rule> Rule for And<T, U> {
    fn parse(&self, state: &mut Tokenizer) -> Option<Token> {
        let pos = state.pos;
        state.ignore_whitespace();
        let res_a = self.0.parse(state);
        if let Some(res_a) = res_a {
            state.ignore_whitespace();
            let res_b = self.1.parse(state);
            if let Some(res_b) = res_b {
                return Some(Token {
                    kind: TokenKind::Unknown,
                    literal: res_a.literal + &res_b.literal,
                    location: res_a.location.clone(),
                });
            }
        }
        state.pos = pos;
        None
    }
}

#[macro_export]
macro_rules! lex {
    ($token:ident) => {
        $token
    };
    ($token:literal) => {
        Lit($token)
    };
    ($lhs:tt | $($($rhs:tt)+)?) => {
        Or(lex!($lhs), lex!($($($rhs)+)?))
    };
    ($lhs:tt, $($($rhs:tt)+)?) => {
        And(lex!($lhs), lex!($($($rhs)+)?))
    };
    () => {
        compile_error!("Write some expression")
    };
}

pub use lex;
