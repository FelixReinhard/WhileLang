use std::str::FromStr;

use crate::charstream::CharStream;

// problem with iterator because we cant peek it without advancing it fucking sucks

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    TkVariable(u32),
    TkWhile,
    TkFor,
    TkConstant(i32),
    TkDo,
    TkOd,
    TkNewline,
    TkPlus,
    TkMinus,
    TkNeq,
    TkEq,
    TkOut,
    TkIn,
}

pub fn lex(chars: &mut CharStream) -> Option<Vec<Token>> {
    let mut current_char = chars.next();

    let mut ret: Vec<Token> = Vec::new();

    loop {
        if let Some(c) = current_char {
            match c {
                'x' => {
                    if let Some(t) = variable(chars) {
                        ret.push(t);
                    } else {
                        return None;
                    }
                }
                'w' => {
                    if let Some(t) = wwhile(chars) {
                        ret.push(t);
                    } else {
                        return None;
                    }
                }
                'f' => {
                    if let Some(t) = ffor(chars) {
                        ret.push(t);
                    } else {
                        return None;
                    }
                }
                '=' => ret.push(Token::TkEq),
                '+' => ret.push(Token::TkPlus),
                '-' => ret.push(Token::TkMinus),
                '!' => {
                    ret.push(is_next_char('=', chars, Token::TkNeq)?);
                }
                'o' | 'O' => {
                    if chars.peek()? == 'u' {
                        chars.next();
                        ret.push(is_next_char('t', chars, Token::TkOut)?);
                    } else {
                        ret.push(is_next_char('d', chars, Token::TkOd)?)
                    }
                }
                'd' | 'D' => ret.push(is_next_char('o', chars, Token::TkDo)?),
                ' ' | '\n' | '\r' => {
                    current_char = chars.next();
                    continue;
                }
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    let cc = number_with(c, chars);
                    ret.push(Token::TkConstant(cc?));
                }
                'i' | 'I' => ret.push(is_next_char('n', chars, Token::TkIn)?),
                _ => {
                    return None;
                }
            }
        } else {
            break;
        }
        current_char = chars.next();
    }
    Some(ret)
}

fn is_next_char(c: char, chars: &mut CharStream, tk: Token) -> Option<Token> {
    if let Some(cc) = chars.next() {
        if cc == c {
            return Some(tk);
        }
    }
    None
}

fn wwhile(chars: &mut CharStream) -> Option<Token> {
    for c in ['h', 'i', 'l', 'e'] {
        let ci = chars.next()?;
        if c != ci {
            return None;
        }
    }

    Some(Token::TkWhile)
}

fn ffor(chars: &mut CharStream) -> Option<Token> {
    for c in ['o', 'r'] {
        let ci = chars.next()?;
        if c != ci {
            return None;
        }
    }

    Some(Token::TkFor)
}

fn variable(chars: &mut CharStream) -> Option<Token> {
    let x = chars.next()?;
    if x != '_' {
        return None;
    }
    Some(Token::TkVariable(number(chars)?))
}

fn number(chars: &mut CharStream) -> Option<u32> {
    let mut number_str = String::new();
    loop {
        if !chars.is_empty() && chars.peek()?.is_digit(10) {
            number_str.push(chars.next()?);
        } else {
            break;
        }
    }

    match u32::from_str(&number_str) {
        Ok(b) => Some(b),
        Err(_) => None,
    }
}

fn number_with(c: char, chars: &mut CharStream) -> Option<i32> {
    let mut number_str = String::new();
    number_str.push(c);
    loop {
        if !chars.is_empty() && chars.peek()?.is_digit(10) {
            number_str.push(chars.next()?);
        } else {
            break;
        }
    }

    match i32::from_str(&number_str) {
        Ok(b) => Some(b),
        Err(_) => None,
    }
}
