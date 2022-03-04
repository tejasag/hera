use std::fmt::{self, Debug, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Int(i32),
    Str(String),
    Bool(bool),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Percent,

    Lt,
    Gt,
    LtEq,
    GtEq,
    Equal,
    NotEq,

    Comma,
    SemiColon,

    RParen,
    LParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Function,
    Let,
    If,
    Else,
    Return,
    Import,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
