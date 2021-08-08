use super::token::Token;
use std::collections::HashMap;

lazy_static! {
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
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = Token::EQ;
                } else {
                    tok = Token::ASSIGN;
                }
            }
            ';' => tok = Token::SEMICOLON,
            '(' => tok = Token::LPAREN,
            ')' => tok = Token::RPAREN,
            '{' => tok = Token::LBRACE,
            '}' => tok = Token::RBRACE,
            ',' => tok = Token::COMMA,
            '+' => tok = Token::PLUS,
            '-' => tok = Token::MINUS,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = Token::NOT_EQ;
                } else {
                    tok = Token::BANG;
                }
            }
            '*' => tok = Token::ASTERISK,
            '/' => tok = Token::SLASH,
            '<' => tok = Token::LT,
            '>' => tok = Token::GT,
            '\u{0}' => tok = Token::EOF,
            _ => {
                if is_letter(self.ch) {
                    let i = self.read_identifier();
                    tok = match lookup_indentifier(i.as_str()) {
                        Some(a) => a.to_owned(),
                        _ => Token::IDENT(i),
                    };
                    return tok;
                } else if is_numeric(self.ch) {
                    let i = self.read_number();
                    tok = Token::INT(i);
                    return tok;
                } else {
                    tok = Token::ILLEGAL;
                }
            }
        };

        &self.read_char();
        tok
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

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            0 as char
        } else {
            self.input.chars().nth(self.read_position).unwrap()
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
