use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

const INST: [&str; 8] = ["+", "-", "*", "/", "swap", "over", "dup", "drop"];

pub struct Forth {
    stack: Vec<Value>,
    custom_inst: HashMap<String, Vec<String>>,
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
        Self {
            stack: Vec::new(),
            custom_inst: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.to_ascii_lowercase();
        let mut words = input.split_whitespace();

        while let Some(word) = words.next() {
            match word {
                w if w == String::from(":") => {
                    let name = words.next().unwrap();
                    let mut def = vec![];
                    while let Some(w) = words.next() {
                        if w == ";" {
                            break;
                        }
                        def.push(w.to_string());
                    }
                    self.custom_inst.insert(name.to_string(), def.clone());
                },
                w if self.custom_inst.contains_key(w) => {
                    
                },
                w if INST.contains(&w) => self.exec_inst(w)?,
                w if w.parse::<Value>().is_ok() => self.stack.push(w.parse::<Value>().unwrap()),
                _ => return Err(Error::UnknownWord),
            }
        }

        Ok(())
    }

    pub fn exec_inst(&mut self, inst: &str) -> Result {
        let len = self.stack.len();
        if len < INST.contains(&inst) as usize + 1 {
            return Err(Error::StackUnderflow);
        }

        let elem = self.stack.pop().unwrap();
        match inst {
            "+" => self.stack[len - 2] += elem,
            "-" => self.stack[len - 2] -= elem,
            "*" => self.stack[len - 2] *= elem,
            "/" => match elem { 0 => return Err(Error::DivisionByZero), _ => self.stack[len - 2] /= elem },
            "dup" => self.stack.append(&mut vec![elem, elem]),
            "swap" => self.stack.insert(len - 2, elem),
            "over" => self.stack.append(&mut vec![elem, self.stack[len - 2]]),
            _ => (),
        }

        Ok(())
    }

    // pub fn create_custom_inst(&mut self, words: &mut impl Iterator) -> Result {
    //     while let Some(word) = words.next() {
    //         println!("{word:?}");
    //     }
    //     Ok(())
    // }
}
