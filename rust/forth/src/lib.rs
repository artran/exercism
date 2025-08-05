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
        input.split_whitespace().for_each(|token| {
            self.data.push(token.parse().unwrap());
        });
        Ok(())
    }
}
