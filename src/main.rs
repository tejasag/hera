#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

pub mod lexer;
pub mod repl;
pub mod token;

#[cfg(test)]
pub mod lexer_test;

use std::env;

fn main() {
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
