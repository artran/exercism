use std::collections::HashMap;
use std::rc::Rc;

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
    Word(Rc<[Op]>),
}

pub struct Forth {
    data: Vec<Value>,
    definitions: HashMap<String, Rc<[Op]>>,
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
            if token == ":" {
                self.parse_definition(&mut iter)?;
            } else {
                let op = self.token_to_op(token)?;
                self.execute_op(&op)?;
            }
        }

        Ok(())
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
                    self.definitions
                        .insert(name.to_lowercase(), definition_ops.into());
                    return Ok(());
                }
                Some(token) => {
                    definition_ops.push(self.token_to_op(token)?);
                }
                None => return Err(Error::InvalidWord),
            }
        }
    }

    fn token_to_op(&self, token: &str) -> std::result::Result<Op, Error> {
        let lower_token = token.to_lowercase();
        if let Some(def) = self.definitions.get(&lower_token) {
            return Ok(Op::Word(def.clone()));
        }

        match lower_token.as_str() {
            "+" => Ok(Op::Add),
            "-" => Ok(Op::Sub),
            "*" => Ok(Op::Mul),
            "/" => Ok(Op::Div),
            "dup" => Ok(Op::Dup),
            "drop" => Ok(Op::Drop),
            "swap" => Ok(Op::Swap),
            "over" => Ok(Op::Over),
            _ => {
                if let Ok(val) = token.parse::<Value>() {
                    Ok(Op::Push(val))
                } else {
                    Err(Error::UnknownWord)
                }
            }
        }
    }

    fn execute_op(&mut self, op: &Op) -> Result {
        match op {
            Op::Add => self.calculate(Value::checked_add),
            Op::Sub => self.calculate(Value::checked_sub),
            Op::Mul => self.calculate(Value::checked_mul),
            Op::Div => self.calculate(Value::checked_div),
            Op::Dup => self.dup(),
            Op::Drop => self.drop(),
            Op::Swap => self.swap_over(false),
            Op::Over => self.swap_over(true),
            Op::Push(val) => {
                self.data.push(*val);
                Ok(())
            }
            Op::Word(def) => {
                for inner_op in def.iter() {
                    self.execute_op(inner_op)?;
                }
                Ok(())
            }
        }
    }

    fn calculate<F>(&mut self, operation: F) -> Result
    where
        F: Fn(Value, Value) -> Option<Value>,
    {
        if let Some(b) = self.data.pop() {
            if let Some(a) = self.data.pop() {
                if let Some(result) = operation(a, b) {
                    self.data.push(result);
                    return Ok(());
                } else {
                    self.data.push(a);
                    self.data.push(b);
                    return Err(Error::DivisionByZero);
                }
            } else {
                self.data.push(b);
                return Err(Error::StackUnderflow);
            }
        }
        Err(Error::StackUnderflow)
    }

    fn dup(&mut self) -> Result {
        if let Some(val) = self.data.last() {
            self.data.push(*val);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn drop(&mut self) -> Result {
        if self.data.pop().is_some() {
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn swap_over(&mut self, over: bool) -> Result {
        if let Some(last) = self.data.pop() {
            if let Some(second_to_last) = self.data.pop() {
                if over {
                    // Put second_to_last back
                    self.data.push(second_to_last);
                }
                // Put last and second_to_last back in swapped order
                self.data.push(last);
                self.data.push(second_to_last);
                return Ok(());
            }
        }
        Err(Error::StackUnderflow)
    }
}
