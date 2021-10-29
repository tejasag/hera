use crate::{
    eval::{env::Env, object::Object, Eval},
    lexer::Lexer,
    parser::Parser,
    token::Token,
};
use std::io::{stdin, stdout, Write};

pub fn start() {
    let mut action = "eval";
    let env = Env::new();
    loop {
        print!(">>> ");
        let _ = stdout().flush();
        let mut input_string = String::new();
        stdin().read_line(&mut input_string).ok();

        if input_string.is_empty() {
            println!("\nPlease enter valid code.");
            continue;
        }

        match input_string.as_str() {
            "eval()\n" => {
                action = "eval";
                continue;
            }
            "parse()\n" => {
                action = "parse";
                continue;
            }
            "lex()\n" => {
                action = "lex";
                continue;
            }
            _ => (),
        };

        let mut lexer = Lexer::new(input_string);

        match action {
            "parse" => {
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                if !parser.errors.is_empty() {
                    print_parse_errors(parser.errors);
                    continue;
                }
                println!("{:#?}", program);
            }
            "eval" => {
                let mut parser = Parser::new(lexer);
                let program = parser.parse_program();
                if !parser.errors.is_empty() {
                    print_parse_errors(parser.errors);
                    continue;
                }

                let eval = (Eval::new()).eval(program, env.clone());
                println!("{}", eval.unwrap_or(Object::Null));
            }
            "lex" => {
                let mut token = lexer.next_token();
                while token != Token::Eof {
                    println!("Token: {:?}", token);

                    token = lexer.next_token();
                }
            }
            _ => continue,
        }
    }
}

fn print_parse_errors(errors: Vec<String>) {
    for e in errors.iter() {
        println!("\t{}", e);
    }
}
