use super::token::Token;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token> = {
        let mut keywords = HashMap::new();
        keywords.insert("fn", Token::FUNCTION);
        keywords.insert("let", Token::LET);
        keywords.insert("true", Token::TRUE);
        keywords.insert("false", Token::FALSE);
        keywords.insert("if", Token::IF);
        keywords.insert("else", Token::ELSE);
        keywords.insert("return", Token::RETURN);
        keywords
    };
}

pub fn lookup_indentifier(i: &str) -> Option<&'static Token> {
    KEYWORDS.get(i)
}

pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            ch: input.chars().nth(0).unwrap(),
            input,
            position: 0,
            read_position: 1,
        }
    }

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
        self.skip_whitespace();

        let tok: Token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            }
            ';' => Token::SEMICOLON,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '{' => Token::LBRACE,
            '}' => Token::RBRACE,
            ',' => Token::COMMA,
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NOT_EQ
                } else {
                    Token::BANG
                }
            }
            '*' => Token::ASTERISK,
            '/' => Token::SLASH,
            '<' => Token::LT,
            '>' => Token::GT,
            '\u{0}' => Token::EOF,
            _ => {
                if is_letter(self.ch) {
                    let i = self.read_identifier();
                    match lookup_indentifier(i.as_str()) {
                        Some(a) => a.to_owned(),
                        _ => Token::IDENT(i),
                    }
                } else if self.ch.is_numeric() {
                    let i = self.read_number();
                    Token::INT(i)
                } else {
                    Token::ILLEGAL
                }
            }
        };

        self.read_char();
        tok
    }

    pub fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }
        self.input[pos..self.position].to_owned()
    }

    pub fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        self.input[pos..self.position].to_owned()
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            0 as char
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}
