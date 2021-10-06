use crate::{lexer::Lexer, parser::Parser};
use std::io::{stdin, stdout, Write};

pub fn start() {
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
        if parser.errors.len() != 0 {
            print_parse_errors(parser.errors);
            continue;
        }

        println!("{:#?}", program);

        // let mut token = lexer.next_token();

        /* while token != Token::Eof {
             println!("Token: {:?}", token);

             token = lexer.next_token();
         }
        */
    }
}

fn print_parse_errors(errors: Vec<String>) {
    for e in errors.iter() {
        println!("\t{}", e);
    }
}
