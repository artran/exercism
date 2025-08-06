pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    data: Vec<Value>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth { data: Vec::new() }
    }

    pub fn stack(&self) -> &[Value] {
        &self.data
    }

    pub fn eval(&mut self, input: &str) -> Result {
        input
            .split_whitespace()
            .try_for_each(|token| self.evaluate_token(token))
    }

    fn evaluate_token(&mut self, token: &str) -> Result {
        match token.parse::<i32>() {
            Ok(c) => self.data.push(c),
            Err(_) => match token {
                "+" => return self.add(),
                _ => return Err(Error::InvalidWord),
            },
        }

        Ok(())
    }

    fn add(&mut self) -> Result {
        if let Some(first) = self.data.pop() {
            if let Some(second) = self.data.pop() {
                self.data.push(first + second);
                return Ok(());
            }
        }
        Err(Error::StackUnderflow)
    }
}
