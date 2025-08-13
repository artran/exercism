use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    data: Vec<Value>,
    definitions: HashMap<String, Vec<String>>,
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
        Forth {
            data: Vec::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.data
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut iter = input.split_whitespace();

        while let Some(token) = iter.next() {
            self.evaluate_token(token, &mut iter)?;
        }

        Ok(())
    }

    fn evaluate_token<'a>(
        &mut self,
        token: &'a str,
        iter: &mut impl Iterator<Item = &'a str>,
    ) -> Result {
        match token {
            ":" => self.parse_definition(iter),
            _ => self.execute_builtin(token),
        }
    }

    fn parse_definition<'a>(
        &mut self,
        iter: &mut impl Iterator<Item = &'a str>,
    ) -> std::result::Result<(), Error> {
        let Some(name) = iter.next() else {
            return Err(Error::InvalidWord);
        };

        if iter.last() != Some(";") {
            return Err(Error::InvalidWord);
        }

        let mut definition: Vec<String> = vec![];
        for token in iter {
            if token != ";" {
                definition.push(token.to_string());
            }
        }

        self.definitions.insert(name.to_string(), definition);

        Ok(())
    }

    fn execute_builtin(&mut self, token: &str) -> Result {
        match token {
            "+" => self.calculate(i32::checked_add),
            "-" => self.calculate(i32::checked_sub),
            "*" => self.calculate(i32::checked_mul),
            "/" => self.calculate(i32::checked_div),
            "dup" => self.dup(),
            "drop" => self.drop(),
            "swap" => self.swap_over(false),
            "over" => self.swap_over(true),
            _ => self.try_numeric(token),
        }
    }

    fn calculate<F>(&mut self, operation: F) -> Result
    where
        F: Fn(i32, i32) -> Option<i32>,
    {
        if let Some(first) = self.data.pop() {
            if let Some(second) = self.data.pop() {
                if let Some(result) = operation(second, first) {
                    self.data.push(result);
                    return Ok(());
                } else {
                    // Only handling DivisionByZero because that is all required
                    return Err(Error::DivisionByZero);
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

    fn swap_over(&mut self, over: bool) -> Result {
        if let Some(first) = self.data.pop() {
            if let Some(second) = self.data.pop() {
                if over {
                    self.data.push(second);
                }
                self.data.push(first);
                self.data.push(second);
                return Ok(());
            }
        }
        Err(Error::StackUnderflow)
    }

    fn try_numeric(&mut self, token: &str) -> Result {
        match token.parse::<i32>() {
            Ok(c) => {
                self.data.push(c);
                Ok(())
            }
            Err(_) => Err(Error::InvalidWord),
        }
    }
}
