use super::token::Token;
use std::io::{stdin, stdout, Write};

pub fn start() {
    loop {
        print!(">> ");
        stdout().flush();
        let mut input_string = String::new();
        stdin()
            .read_line(&mut input_string)
            .ok()
            .expect("Could not take in a string!");
        if input_string.is_empty() {
            println!("\nPlease enter valid code.");
            continue;
        }
        let mut l = super::lexer::New(input_string);
        let mut tok = l.next_token();
        while tok != Token::EOF {
            println!("Token: {:?}", tok);
            tok = l.next_token();
        }
    }
}
