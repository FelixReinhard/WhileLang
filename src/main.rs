mod charstream;
mod interpreter;
mod lexer;
mod parser;

use lexer::lex;
use parser::parse_while;
use std::{env, fs::File, io::Read};

use crate::{charstream::CharStream, interpreter::Interpreter, parser::parse_for};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        if args.len() == 2 && args[1] == "shellb" {
            shell(false);
            return;
        } else if args.len() == 2 && args[1] == "shellw" {
            shell(true);
            return;
        }
        println!("Use: run [filename.w | filename.f]");
        println!("     shellb");
        println!("     shellw");
        return;
    }
    if args[1] != "run" {
        println!("{} is no valid command!", args[1]);
        return;
    }

    let to_run = &args[2];
    if to_run.contains(".w") {
        if let Ok(val) = run(to_run, true) {
            println!("Programm exited with {}", val);
        } else {
            println!("Programm contained an error");
        }
    } else if to_run.contains(".f") {
        if let Ok(val) = run(to_run, false) {
            println!("Programm exited with {}", val);
        } else {
            println!("Programm contained an error");
        }
    } else {
        //run_direct(to_run);
    }
}

fn shell(is_while: bool) {
    let mut interpreter = Interpreter::new(is_while);
    use std::io::{stdin, stdout, Write};
    loop {

        let mut s = String::new();
        print!(">>: ");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        
        let mut stream = CharStream::new(s);
        let tokens = match lex(&mut stream) {
            Some(tkns) => tkns,
            None => {
                println!("Lexing Error");
                continue;
            }
        };

        let stmts = if is_while {
            if let Some(stmt) = parse_while(tokens) {
                stmt 
            } else {
                println!("Error parsing while!");
                continue;
            }
        } else {
            if let Some(stmt) = parse_for(tokens) {
                stmt
            } else {
                println!("Error parsing for!");
                continue;
            }
        };
        
        let result = interpreter.interpret(stmts);
        interpreter.print_map();
        if let Some(r) = result {
            println!("Result : {}", r);
        } else {
            println!("error interpreting stmt");
        }
    }
}

fn run(path: &String, is_while: bool) -> Result<i32, ()> {
    if let Ok(mut file) = File::open(path) {
        let mut content = String::new();
        let res = file.read_to_string(&mut content);
        if let Ok(_) = res {
            let mut charstream = CharStream::new(content);
            let ret = lex(&mut charstream);
            let tokens = ret.unwrap();
            let stmts = if is_while {
                parse_while(tokens).unwrap()
            } else {
                parse_for(tokens).unwrap()
            };

            let mut interpreter = Interpreter::new(is_while);
            let result = interpreter.interpret(stmts);
            interpreter.print_map();
            return Ok(result.unwrap_or(0));
        } else {
            return Err(());
        }
    } else {
        return Err(());
    }
}
