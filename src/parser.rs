use std::{cmp::max, str::FromStr};

use crate::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum Statement {
    Plus(u32, u32, u32),
    Minus(u32, u32, u32),
    Constant(u32, i32),
    Loop(u32, Box<Statement>),
    Concat(Box<Statement>, Box<Statement>),
    None,
}

pub fn parse_while(tokens: Vec<Token>) -> Option<Statement> {
    let parser = Parser::new(tokens);
    parser.parse(true)
}

pub fn parse_for(tokens: Vec<Token>) -> Option<Statement> {
    let parser = Parser::new(tokens);
    parser.parse(false)
}

struct Parser {
    tokens: Vec<Token>,
    pointer: usize,
}

impl Parser {
    fn new(t: Vec<Token>) -> Parser {
        Parser {
            tokens: t,
            pointer: 0,
        }
    }

    fn parse(mut self, is_while: bool) -> Option<Statement> {
        let mut stmt = Statement::None;

        while !self.is_empty() {
            let tmp = self.statement(is_while)?;
            if stmt == Statement::None {
                stmt = tmp;
            } else {
                stmt = Statement::Concat(Box::new(stmt), Box::new(tmp));
            }
        }
        Some(stmt)
    }

    fn statement(&mut self, is_while: bool) -> Option<Statement> {
        match self.peek().unwrap() {
            Token::TkVariable(_) => self.simple_statement(),
            Token::TkWhile => {
                if is_while {
                    self.while_statement()
                } else {
                    print!("No While in for programms");
                    None
                }
            }
            Token::TkFor => {
                if !is_while {
                    self.for_statement()
                } else {
                    print!("No for in while programms");
                    None
                }
            }
            _ => {
                println!("statement must begin with variable");
                None
            }
        }
    }

    fn for_statement(&mut self) -> Option<Statement> {
        self.consume(Token::TkFor);

        let var1;
        if let Some(v) = self.variable() {
            var1 = v;
        } else {
            println!("No Variable here parsing while");
            return None;
        }

        if !self.consume(Token::TkDo) {
            println!("Couldnt consume !=");
            return None;
        }
        let mut stmt = self.statement(true)?;
        while !self.is_empty() && self.peek().unwrap() != &Token::TkOd {
            stmt = Statement::Concat(Box::new(stmt), Box::new(self.statement(true)?));
        }

        if !self.consume(Token::TkOd) {
            println!("Couldnt consume !=");
            return None;
        }

        Some(Statement::Loop(var1, Box::new(stmt)))
    }

    fn simple_statement(&mut self) -> Option<Statement> {
        let var1;

        if let Some(v) = self.variable() {
            var1 = v;
        } else {
            println!("No Variable here parsing simple statement");
            return None;
        }
        // consume =
        self.consume(Token::TkEq);

        if let Some(Token::TkConstant(val)) = self.next() {
            return Some(Statement::Constant(var1, *val));
        } else {
            self.back();
        }

        if let Some(Token::TkMinus) = self.next() {
            let value = match self.next().unwrap() {
                Token::TkConstant(v) => Some(*v),
                _ => {
                    println!("No Constant after minus in constant stmt");
                    None
                }
            };
            if let Some(v) = value {
                return Some(Statement::Constant(var1, -v));
            } else {
                return None;
            }
        } else {
            self.back()
        }

        let var2;
        if let Some(v) = self.variable() {
            var2 = v;
        } else {
            println!("No Variable here parsing simple statement");
            return None;
        }

        let is_plus;
        match self.next().unwrap_or(&Token::TkNewline) {
            Token::TkPlus => {
                is_plus = true;
            }
            Token::TkMinus => {
                is_plus = false;
            }
            _ => {
                println!("simple_expression needs eighter constant or +/- after first var");
                return None;
            }
        }

        let var3;
        if let Some(v) = self.variable() {
            var3 = v;
        } else {
            println!("No Variable here parsing simple statement");
            return None;
        }
        if is_plus {
            return Some(Statement::Plus(var1, var2, var3));
        } else {
            return Some(Statement::Minus(var1, var2, var3));
        }
    }

    fn while_statement(&mut self) -> Option<Statement> {
        self.consume(Token::TkWhile);

        let var1;
        if let Some(v) = self.variable() {
            var1 = v;
        } else {
            println!("No Variable here parsing while");
            return None;
        }

        if !self.consume(Token::TkNeq) {
            println!("Couldnt consume !=");
            return None;
        }

        if let Some(Token::TkConstant(val)) = self.next() {
            if *val != i32::from_str("0").unwrap() {
                println!("constant after != in while has to be 0");
                return None;
            }
        } else {
            println!("no constant after != in while");
            return None;
        }

        if !self.consume(Token::TkDo) {
            println!("Couldnt consume !=");
            return None;
        }
        let mut stmt = self.statement(true)?;
        while !self.is_empty() && self.peek().unwrap() != &Token::TkOd {
            stmt = Statement::Concat(Box::new(stmt), Box::new(self.statement(true)?));
        }

        if !self.consume(Token::TkOd) {
            println!("Couldnt consume !=");
            return None;
        }

        Some(Statement::Loop(var1, Box::new(stmt)))
    }

    fn back(&mut self) {
        self.pointer = max(0, self.pointer - 1);
    }

    fn consume(&mut self, tk: Token) -> bool {
        if let Some(peeked) = self.peek() {
            if *peeked == tk {
                self.pointer += 1;
                return true;
            } else {
                return false;
            }
        }
        false
    }

    fn variable(&mut self) -> Option<u32> {
        if let Some(Token::TkVariable(val)) = self.next() {
            Some(*val)
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.tokens.len() <= self.pointer
    }

    fn next(&mut self) -> Option<&Token> {
        if let Some(x) = self.tokens.get(self.pointer) {
            self.pointer += 1;
            Some(x)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&Token> {
        if let Some(x) = self.tokens.get(self.pointer) {
            Some(x)
        } else {
            None
        }
    }
}
