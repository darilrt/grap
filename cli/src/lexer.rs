use lib::*;

struct Ident;

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

struct Number;

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
