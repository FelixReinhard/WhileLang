use std::collections::HashMap;
use std::io;

use crate::parser::Statement;

pub struct Interpreter {
    map : HashMap<u32, i32>,
    is_while: bool
}

impl Interpreter {
    pub fn new(is_while: bool) -> Interpreter {
        Interpreter{map : HashMap::new(), is_while: is_while}
    }

    pub fn interpret(&mut self, statement: Statement) -> Option<i32> {

        self.interpret_statements(&statement);

        Some(*self.map.get(&0).unwrap_or(&0))
    }

    pub fn print_map(&self) {
        for (key, value) in &self.map {
            println!("x_{} = {}", key, value);
        }
    }

    fn interpret_statements(&mut self, stmt : &Statement) {
        match stmt {
            Statement::None => {},
            Statement::Concat(stmt1, stmt2) => {
                self.interpret_statements(&stmt1);
                self.interpret_statements(&stmt2);
            },
            Statement::Constant(variable, value) => {
                self.map.insert(*variable, *value);
            }, 
            Statement::Loop(variable, stmt) => {
                if self.is_while {
                    while *self.map.get(variable).unwrap_or(&0) != 0 {
                        self.interpret_statements(stmt);
                    }
                } else {
                    let loop_times = *self.map.get(variable).unwrap_or(&0);
                    for _ in 0..loop_times {
                        self.interpret_statements(stmt);
                    }
                }
            },
            Statement::Minus(var1, var2, var3) => {
                let left = *self.map.get(var2).unwrap_or(&0);
                let right = *self.map.get(var3).unwrap_or(&0);
                self.map.insert(*var1, left - right);
            }, 
            Statement::Plus(var1, var2, var3) => {
                let left = *self.map.get(var2).unwrap_or(&0);
                let right = *self.map.get(var3).unwrap_or(&0);
                self.map.insert(*var1, left + right);
            },
            Statement::In(var) => {
                let mut input_line = String::new();
                io::stdin()
                    .read_line(&mut input_line)
                    .expect("Failed to read line");
                let x: i32 = input_line.trim().parse().expect("Input not an integer");
                self.map.insert(*var, x);
            },
            Statement::Out(var) => {
                println!("{}", self.map.get(var).unwrap_or(&0));
            }
        }
    }
}   
