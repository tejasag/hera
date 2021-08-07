use std::collections::HashMap;

use super::token::{Token, TokenType};

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut keywords = HashMap::new();
        keywords.insert("fn", TokenType::FUNCTION);
        keywords.insert("let", TokenType::LET);
        keywords
    };
}

pub fn lookup_indentifier(i: &str) -> Option<&'static TokenType> {
    KEYWORDS.get(i)
}

pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

impl Lexer {
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        self.skip_whitespace();

        match self.ch {
            '=' => tok = self.new_token(TokenType::ASSIGN, self.ch),
            ';' => {
                tok = self.new_token(TokenType::SEMICOLON, self.ch);
            }
            '(' => tok = self.new_token(TokenType::LPAREN, self.ch),
            ')' => tok = self.new_token(TokenType::RPAREN, self.ch),
            '{' => tok = self.new_token(TokenType::LBRACE, self.ch),
            '}' => tok = self.new_token(TokenType::RBRACE, self.ch),
            ',' => tok = self.new_token(TokenType::COMMA, self.ch),
            '+' => tok = self.new_token(TokenType::PLUS, self.ch),
            '\u{0}' => {
                tok = Token {
                    token_type: TokenType::EOF,
                    literal: "",
                }
            }
            _ => {
                if is_letter(self.ch) {
                    let i = self.read_identifier();
                    let t_type = match lookup_indentifier(i.as_str()) {
                        Some(a) => a.to_owned(),
                        _ => TokenType::IDENT,
                    };
                    tok = Token {
                        token_type: t_type,
                        literal: Box::leak(i.into_boxed_str()),
                    };
                    return tok;
                } else if is_numeric(self.ch) {
                    let i = self.read_number();
                    tok = Token {
                        token_type: TokenType::INT,
                        literal: Box::leak(i.into_boxed_str()),
                    };
                    return tok;
                } else {
                    tok = self.new_token(TokenType::ILLEGAL, self.ch)
                }
            }
        };

        &self.read_char();
        tok
    }

    pub fn new_token<'a>(&self, token_type: TokenType, ch: char) -> Token<'a> {
        let ch_string: String = ch.to_string();
        Token {
            token_type,
            literal: Box::leak(ch_string.into_boxed_str()),
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }
        self.input.get(pos..self.position).unwrap().to_owned()
    }

    pub fn read_number(&mut self) -> String {
        let pos = self.position;
        while is_numeric(self.ch) {
            self.read_char();
        }
        self.input.get(pos..self.position).unwrap().to_owned()
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }
}

pub fn New(input: String) -> Lexer {
    Lexer {
        ch: input.chars().nth(0).unwrap(),
        input,
        position: 0,
        read_position: 1,
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_numeric(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}
