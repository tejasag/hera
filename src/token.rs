use std::fmt;

use fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    IDENT,
    INT,

    ASSIGN,
    PLUS,

    COMMA,
    SEMICOLON,

    RPAREN,
    LPAREN,
    LBRACE,
    RBRACE,

    FUNCTION,
    LET,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a str,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] lit: '{}'", self.token_type, self.literal)
    }
}
