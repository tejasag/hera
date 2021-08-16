use super::{
    ast::{Program, Statement},
    lexer::Lexer,
    token::Token,
};

pub struct Parser {
    pub l: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            l: lexer,
            current_token: Token::Eof,
            peek_token: Token::Eof,
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&self) -> Program {
        let statements: Vec<Statement> = vec![];
        while self.current_token != Token::Eof {
            let stmt = self.parse_statement();
            if stmt != None {
                statements.push(stmt);
            };
            self.next_token();
        }
        Program { statements }
    }

    pub fn parse_statement(&self) -> Option<Statement> {
        match self.current_token {
            Token::Let => Some(self.parse_let_statement()),
            _ => None,
        }
    }

    pub fn parse_let_statement(&self) -> Option<Statement> {
        match *self {
           self.expect_peek(Token::Ident) =>
        }
    }

    fn peek_token_is(&self, t: Token) -> bool {
        self.peek_token == t
    }

    fn current_token_is(&self, t: Token) -> bool {
        self.current_token == t
    }

    fn expect_peek(&self, t: Token) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }
}
