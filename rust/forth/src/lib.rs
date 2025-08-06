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

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
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
        match token {
            "+" => self.calculate(Operation::Add),
            "-" => self.calculate(Operation::Subtract),
            "*" => self.calculate(Operation::Multiply),
            "/" => self.calculate(Operation::Divide),
            "dup" => self.dup(),
            "drop" => self.drop(),
            _ => self.try_numeric(token),
        }
    }

    fn calculate(&mut self, operation: Operation) -> Result {
        if let Some(first) = self.data.pop() {
            if let Some(second) = self.data.pop() {
                match operation {
                    Operation::Add => {
                        self.data.push(second + first);
                        return Ok(());
                    }
                    Operation::Subtract => {
                        self.data.push(second - first);
                        return Ok(());
                    }
                    Operation::Multiply => {
                        self.data.push(second * first);
                        return Ok(());
                    }
                    Operation::Divide => {
                        if first != 0 {
                            self.data.push(second / first);
                            return Ok(());
                        } else {
                            return Err(Error::DivisionByZero);
                        }
                    }
                }
            }
        }
        Err(Error::StackUnderflow)
    }

    fn dup(&mut self) -> Result {
        if let Some(first) = self.data.pop() {
            self.data.push(first);
            self.data.push(first);
            return Ok(());
        }
        Err(Error::StackUnderflow)
    }

    fn drop(&mut self) -> Result {
        if self.data.pop().is_some() {
            return Ok(());
        }
        Err(Error::StackUnderflow)
    }

    fn try_numeric(&mut self, token: &str) -> Result {
        match token.parse::<i32>() {
            Ok(c) => {
                self.data.push(c);
                return Ok(());
            }
            Err(_) => return Err(Error::InvalidWord),
        }
    }
}
