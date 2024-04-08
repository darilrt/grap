use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Value,     // String | Number
    Ident,     // Identifier
    Operation, // example: +, -, *, /
    Keyword,   // example: let, if, else
    Symbol,    // example: (, ), {, }, [, ], ;
    Unknown,
    EOF,
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({:?})", self.kind, self.literal)
    }
}
