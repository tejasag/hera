#[macro_use]
extern crate lazy_static;

pub mod lexer;
pub mod token;

#[cfg(test)]
pub mod lexer_test;

fn main() {
    println!("hello world!");
}
