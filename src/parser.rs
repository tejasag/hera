use super::{ast::*, lexer::Lexer, token::Token};

pub struct Parser {
    pub l: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut p: Parser = Parser {
            l: lexer,
            current_token: Token::Eof,
            peek_token: Token::Eof,
            errors: vec![],
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = vec![];
        while self.current_token != Token::Eof {
            let stmt: Option<Statement> = self.parse_statement();
            if stmt != None {
                statements.push(stmt.unwrap());
            };
            self.next_token();
        }
        Program { statements }
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            // _ => panic!("Illegal token found."),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_expression_statement(&mut self) -> Option<Statement> {
        match self.parse_expression(Precedence::Lowest) {
            Some(expression) => {
                if self.peek_token_is(&Token::SemiColon) {
                    self.next_token();
                }
                Some(Statement::Expression(expression))
            }
            None => None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        match &self.peek_token {
            Token::Ident(_) => self.next_token(),
            _ => {
                self.peek_error(Token::Ident(String::new()));
                return None;
            }
        }

        let name: Ident = match self.parse_ident() {
            Some(Expression::Ident(ref mut s)) => s.clone(),
            _ => return None,
        };

        if !self.expect_peek(Token::Assign) {
            return None;
        }

        self.next_token();

        let lit: Expression = match self.parse_expression(Precedence::Lowest) {
            Some(e) => e,
            None => return None,
        };

        while !self.current_token_is(Token::SemiColon) {
            self.next_token();
        }

        Some(Statement::Let(name, lit))
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        // let cur = &self.current_token;
        self.next_token();
        while !self.current_token_is(Token::SemiColon) {
            self.next_token();
        }

        Some(Statement::Return(Expression::Literal(Literal::Int(1))))
    }

    fn parse_expression(&mut self, _precedence: Precedence) -> Option<Expression> {
        // prefix
        let left: Option<Expression> = match self.current_token {
            Token::Ident(_) => self.parse_ident(),
            Token::Int(_) => self.parse_int_literal(),
            Token::Bang | Token::Minus | Token::Plus => self.parse_prefix_expression(),
            Token::LParen => None,
            Token::If => None,
            _ => {
                // TODO: add function call here
                None
            }
        };

        left

        // infix
        /*match self.current_token {
            Token::Int(_) => self.parse_int_token(),
            _ => None,
        }
        */
    }

    fn parse_int_literal(&mut self) -> Option<Expression> {
        match self.current_token {
            Token::Int(ref mut int) => Some(Expression::Literal(Literal::Int(*int))),
            _ => None,
        }
    }

    fn parse_ident(&mut self) -> Option<Expression> {
        match self.current_token {
            Token::Ident(ref mut ident) => Some(Expression::Ident(Ident(ident.clone()))),
            _ => None,
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let prefix = match self.current_token {
            Token::Bang => Prefix::Not,
            Token::Minus => Prefix::Minus,
            Token::Plus => Prefix::Plus,
            _ => return None,
        };

        self.next_token();
        match self.parse_expression(Precedence::Prefix) {
            Some(expr) => Some(Expression::Prefix(prefix, Box::new(expr))),
            None => None,
        }
    }

    fn peek_token_is(&self, t: &Token) -> bool {
        self.peek_token == *t
    }

    fn current_token_is(&self, t: Token) -> bool {
        self.current_token == t
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if let Token::Ident(..) = t {
            self.next_token();
            return true;
        }

        if self.peek_token_is(&t) {
            self.next_token();
            true
        } else {
            self.peek_error(t);
            false
        }
    }

    fn peek_error(&mut self, t: Token) {
        let msg = format!(
            "Expected next token to be {expected}, got {found}",
            expected = t,
            found = self.peek_token
        );
        self.errors.push(msg);
    }

    /*
    fn parse_prefix(&mut self) -> Option<Expression> {
        None
    }

    fn parse_infix(&mut self, expr: ast::Expression) -> Option<Expression> {
        None
    }
    */
}
