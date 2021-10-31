use crate::{
    eval::{env::Env, object::Object, Eval},
    lexer::Lexer,
    parser::Parser,
};
use std::{
    cell::RefCell,
    io::{stdin, stdout, Write},
    rc::Rc,
};

pub fn start() {
    let env = Env::new();
    let mut evaluator = Eval {
        env: Rc::new(RefCell::new(env)),
    };
    loop {
        print!(">>> ");
        let _ = stdout().flush();
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).ok();

        if input_string.is_empty() {
            println!("\nPlease enter valid code.");
            continue;
        }

        let lexer = Lexer::new(input_string);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        if !parser.errors.is_empty() {
            print_parse_errors(parser.errors);
            continue;
        }
        let res = evaluator.eval(program);
        println!("{}", res.unwrap_or(Object::Null));
    }
}

fn print_parse_errors(errors: Vec<String>) {
    for e in errors.iter() {
        println!("\t{}", e);
    }
}
