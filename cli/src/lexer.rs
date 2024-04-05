use lib::*;

pub struct Ident;

impl Rule for Ident {
    fn parse(&self, state: &mut ParserState) -> Option<String> {
        let peek = state.peek();

        if peek.is_none() {
            return None;
        }

        let c = peek.unwrap();

        if !c.is_alphabetic() && c != '_' {
            return None;
        }

        let ident = state.consume_while(|c| c.is_alphanumeric() || c == '_');
        if ident.is_empty() {
            None
        } else {
            Some(ident)
        }
    }
}

pub struct Number;

impl Rule for Number {
    fn parse(&self, state: &mut ParserState) -> Option<String> {
        let number = state.consume_while(|c| c.is_numeric());
        if number.is_empty() {
            None
        } else {
            Some(number)
        }
    }
}

pub struct Str;

impl Rule for Str {
    fn parse(&self, state: &mut ParserState) -> Option<String> {
        let peek = state.peek();

        if peek.is_none() {
            return None;
        }

        let c = peek.unwrap();

        if c != '"' {
            return None;
        }

        state.consume();

        let string = state.consume_while(|c| c != '"');

        if state.consume() != Some('"') {
            return None;
        }

        Some(string)
    }
}
