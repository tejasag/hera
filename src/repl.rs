use crate::{lexer::Lexer, token::Token};
use std::io::{stdin, stdout, Write};

pub fn start() {
    loop {
        print!(">> ");

        let _ = stdout().flush();

        let mut input_string = String::new();
        stdin()
            .read_line(&mut input_string)
            .ok()
            .expect("Could not take in a string!");

        if input_string.is_empty() {
            println!("\nPlease enter valid code.");
            continue;
        }
        let mut lexer = Lexer::new(input_string);
        let mut token = lexer.next_token();

        while token != Token::EOF {
            println!("Token: {:?}", token);

            token = lexer.next_token();
        }
    }
}
