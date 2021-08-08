use std::io::stdin;

use crate::token::Token;

pub fn start() {
    loop {
        println!(">> ");
        let mut input_string = String::new();
        stdin()
            .read_line(&mut input_string)
            //        .ok()
            .expect("Could not take in a string!");
        let mut l = super::lexer::New(input_string);
        // let mut tok = l.next_token();
        // while tok != super::token::Token::EOF {
        //    println!("Token: {}", tok);
        //   tok = l.next_token();
        // }
        loop {
            let tok = l.next_token();
            println!("Token: {}", tok);
            if tok == Token::EOF {
                break;
            }
        }
    }
}
