pub struct ParserState {
    pub input: String,
    pub pos: usize,
    pub stack: Vec<usize>,
}

impl ParserState {
    pub fn new(input: &str) -> Self {
        ParserState {
            input: input.to_string(),
            pos: 0,
            stack: Vec::new(),
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
        }
        c
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

pub trait Rule {
    fn parse(&self, state: &mut ParserState) -> Option<String>;
}

struct Lit {
    value: String,
}

impl Rule for Lit {
    fn parse(&self, state: &mut ParserState) -> Option<String> {
        state.ignore_whitespace();
        let pos = state.pos;
        for c in self.value.chars() {
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
        Some(self.value.clone())
    }
}

pub fn exec<T: Rule>(rule: T, state: &mut ParserState) -> Option<String> {
    rule.parse(state)
}

pub fn lit(value: &str) -> impl Rule {
    Lit {
        value: value.to_string(),
    }
}

pub fn or<T: Rule, U: Rule>(a: T, b: U) -> impl Rule {
    struct Or<T, U>(T, U);

    impl<T: Rule, U: Rule> Rule for Or<T, U> {
        fn parse(&self, state: &mut ParserState) -> Option<String> {
            state.ignore_whitespace();
            let pos = state.pos;
            if let Some(res) = self.0.parse(state) {
                state.ignore_whitespace();
                return Some(res);
            }
            state.pos = pos;
            self.1.parse(state)
        }
    }

    Or(a, b)
}

pub fn and<T: Rule, U: Rule>(a: T, b: U) -> impl Rule {
    struct And<T, U>(T, U);

    impl<T: Rule, U: Rule> Rule for And<T, U> {
        fn parse(&self, state: &mut ParserState) -> Option<String> {
            state.ignore_whitespace();
            let pos = state.pos;
            let res_a = self.0.parse(state);
            if let Some(res_a) = res_a {
                state.ignore_whitespace();
                let res_b = self.1.parse(state);
                if let Some(res_b) = res_b {
                    return Some(res_a + &res_b);
                }
            }
            state.pos = pos;
            None
        }
    }

    And(a, b)
}
