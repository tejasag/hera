#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod ast;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod repl;
pub mod token;

use eval::{env::Env, object::Object, Eval};
use lexer::Lexer;
use parser::Parser;
use std::{cell::RefCell, env, fs, rc::Rc};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 1 && args[1].as_str() == "run" {
        let filename = &args[2].split('.').collect::<Vec<_>>();
        if filename[filename.len() - 1] != "hera" {
            println!("Invalid File name. File must have the `hera` extension.");
            return;
        }
        let content = fs::read_to_string(&args[2]).expect("Could not read the file.");

        let env = Env::new();
        let mut evaluator = Eval {
            env: Rc::new(RefCell::new(env)),
        };
        let lexer = Lexer::new(content);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        //println!("{:#?}", program);
        if !parser.errors.is_empty() {
            for e in parser.errors.iter() {
                println!("\t{}", e);
            }
            return;
        }
        let res = evaluator.eval(program);

        if let Some(o) = res {
            match o {
                Object::Null => (),
                _ => println!("{}", o),
            }
        }
        return;
    }

    let user = match env::var("USER") {
        Ok(i) => i,
        Err(_e) => "there".to_string(),
    };

    println!(
        "Hey {}! This is the Hera programming language. Type in a command to run!",
        user
    );
    repl::start();
}
