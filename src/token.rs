use std::fmt;

use fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    ILLEGAL,
    EOF,

    IDENT(String),
    INT(String),

    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    COMMA,
    SEMICOLON,

    RPAREN,
    LPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}

/*
#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a str,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] lit: '{}'", self.token_type, self.literal)
    }
}
*/
