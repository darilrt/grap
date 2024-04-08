use crate::token::Location;

pub struct Tokenizer {
    pub input: String,
    pub pos: usize,
    pub stack: Vec<usize>,
    pub location: Location,
    pub keywords: Vec<&'static str>,
}

impl Tokenizer {
    pub fn new(input: &str, keywords: Vec<&'static str>) -> Self {
        Tokenizer {
            input: input.to_string(),
            pos: 0,
            stack: Vec::new(),
            location: Location { line: 1, column: 1 },
            keywords,
        }
    }

    pub fn consume_while(&mut self, pred: impl Fn(char) -> bool) -> String {
        let mut res = String::new();
        while self.pos < self.input.len() {
            let c = self.input.chars().nth(self.pos).unwrap();
            if !pred(c) {
                break;
            }
            res.push(c);
            self.pos += 1;
            if c == '\n' {
                self.location.line += 1;
                self.location.column = 1;
            } else {
                self.location.column += 1;
            }
        }
        res
    }

    pub fn ignore_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    pub fn consume(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() {
            self.pos += 1;

            if c.unwrap() == '\n' {
                self.location.line += 1;
                self.location.column = 1;
            } else {
                self.location.column += 1;
            }
        }
        c
    }

    pub fn is_keyword(&self, s: &str) -> bool {
        self.keywords.contains(&s)
    }

    pub fn push(&mut self) {
        self.stack.push(self.pos);
    }

    pub fn pop(&mut self) {
        self.pos = self.stack.pop().unwrap();
    }

    pub fn commit(&mut self) {
        self.stack.pop();
    }
}
