use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Drop,
    Swap,
    Over,
    Push(Value),
}

pub struct Forth {
    data: Vec<Value>,
    definitions: HashMap<String, Vec<Op>>,
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
        if token == ":" {
            self.parse_definition(iter)
        } else {
            self.execute_token(token)
        }
    }

    fn parse_definition<'a>(&mut self, iter: &mut impl Iterator<Item = &'a str>) -> Result {
        let name = iter.next().ok_or(Error::InvalidWord)?;
        if name.parse::<Value>().is_ok() {
            return Err(Error::InvalidWord);
        }

        let mut definition_ops = Vec::new();
        loop {
            match iter.next() {
                Some(";") => {
                    self.definitions.insert(name.to_lowercase(), definition_ops);
                    return Ok(());
                }
                Some(token) => {
                    let ops = self.resolve_token(token)?;
                    definition_ops.extend(ops);
                }
                None => return Err(Error::InvalidWord),
            }
        }
    }

    fn resolve_token(&self, token: &str) -> std::result::Result<Vec<Op>, Error> {
        let lower_token = token.to_lowercase();

        if let Some(def) = self.definitions.get(&lower_token) {
            return Ok(def.clone());
        }

        match lower_token.as_str() {
            "+" => return Ok(vec![Op::Add]),
            "-" => return Ok(vec![Op::Sub]),
            "*" => return Ok(vec![Op::Mul]),
            "/" => return Ok(vec![Op::Div]),
            "dup" => return Ok(vec![Op::Dup]),
            "drop" => return Ok(vec![Op::Drop]),
            "swap" => return Ok(vec![Op::Swap]),
            "over" => return Ok(vec![Op::Over]),
            _ => {}
        }

        if let Ok(val) = token.parse::<Value>() {
            return Ok(vec![Op::Push(val)]);
        }

        Err(Error::UnknownWord)
    }

    fn execute_token(&mut self, token: &str) -> Result {
        let ops = self.resolve_token(token)?;
        for op in ops {
            self.execute_op(&op)?;
        }
        Ok(())
    }

    fn execute_op(&mut self, op: &Op) -> Result {
        match op {
            Op::Add => self.calculate(i32::checked_add),
            Op::Sub => self.calculate(i32::checked_sub),
            Op::Mul => self.calculate(i32::checked_mul),
            Op::Div => self.calculate(i32::checked_div),
            Op::Dup => self.dup(),
            Op::Drop => self.drop(),
            Op::Swap => self.swap_over(false),
            Op::Over => self.swap_over(true),
            Op::Push(val) => {
                self.data.push(*val);
                Ok(())
            }
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
}
