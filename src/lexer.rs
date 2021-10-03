use super::token::Token;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token> = {
        let mut keywords = HashMap::new();
        keywords.insert("fn", Token::Function);
        keywords.insert("let", Token::Let);
        keywords.insert("true", Token::Bool(true));
        keywords.insert("false", Token::Bool(false));
        keywords.insert("if", Token::If);
        keywords.insert("else", Token::Else);
        keywords.insert("return", Token::Return);
        keywords
    };
}

pub fn lookup_indentifier(i: &str) -> Option<&Token> {
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
            ch: input.chars().next().unwrap(),
            input,
            position: 0,
            read_position: 1,
        }
    }

    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            0 as char
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok: Token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            ';' => Token::SemiColon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            }
            '\u{0}' => Token::Eof,
            _ => {
                if is_letter(self.ch) {
                    let i: String = self.read_identifier();
                    return match lookup_indentifier(i.as_str()) {
                        Some(a) => a.to_owned(),
                        _ => Token::Ident(i),
                    };
                } else if self.ch.is_numeric() {
                    let i: i32 = self.read_number();
                    return Token::Int(i);
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let pos: usize = self.position;
        while is_letter(self.ch) {
            self.read_char()
        }
        self.input[pos..self.position].to_string()
    }

    fn read_number(&mut self) -> i32 {
        let pos: usize = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        self.input[pos..self.position].parse::<i32>().unwrap()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return 0 as char;
        }
        self.input.chars().nth(self.read_position).unwrap()
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}
