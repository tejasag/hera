#[cfg(test)]
pub mod test;

use crate::{ast::*, lexer::Lexer, token::Token};

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

    fn token_to_precedence(tok: &Token) -> Precedence {
        match tok {
            Token::Equal | Token::NotEq => Precedence::Equals,
            Token::Lt | Token::LtEq => Precedence::LessGreater,
            Token::Gt | Token::GtEq => Precedence::LessGreater,
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Slash | Token::Asterisk => Precedence::Product,
            Token::LBrace => Precedence::Index,
            Token::LParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
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
        self.next_token();

        let exp = match self.parse_expression(Precedence::Lowest) {
            Some(e) => e,
            None => return None,
        };

        while !self.current_token_is(Token::SemiColon) {
            self.next_token();
        }

        Some(Statement::Return(exp))
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        self.next_token();

        let mut statements = vec![];
        while !self.current_token_is(Token::RBrace) && !self.current_token_is(Token::Eof) {
            if let Some(s) = self.parse_statement() {
                statements.push(s);
            }
            self.next_token();
        }

        statements
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // prefix
        let mut left: Option<Expression> = match self.current_token {
            Token::Ident(_) => self.parse_ident(),
            Token::Int(_) => self.parse_int_literal(),
            Token::Bool(_) => self.parse_bool_literal(),
            Token::Bang | Token::Minus | Token::Plus => self.parse_prefix_expression(),
            Token::LParen => self.parse_grouped_expression(),
            Token::If => self.parse_if_expression(),
            Token::Function => self.parse_fn_expression(),
            _ => {
                // TODO: add function call here
                None
            }
        };

        // infix
        while !self.peek_token_is(&Token::SemiColon) && precedence < self.next_token_precedence() {
            match self.peek_token {
                Token::Plus
                | Token::Minus
                | Token::Asterisk
                | Token::Equal
                | Token::Slash
                | Token::NotEq
                | Token::Lt
                | Token::LtEq
                | Token::Gt
                | Token::GtEq => {
                    self.next_token();
                    left = self.parse_infix_expression(left.unwrap());
                }
                Token::LParen => {
                    self.next_token();
                    left = self.parse_call_expression(left.unwrap());
                }
                _ => return left,
            }
        }

        left
    }

    fn parse_int_literal(&mut self) -> Option<Expression> {
        match self.current_token {
            Token::Int(ref mut int) => Some(Expression::Literal(Literal::Int(*int))),
            _ => None,
        }
    }

    fn parse_bool_literal(&mut self) -> Option<Expression> {
        match self.current_token {
            Token::Bool(boolean) => Some(Expression::Literal(Literal::Bool(boolean))),
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

        self.parse_expression(Precedence::Prefix)
            .map(|expr| Expression::Prefix(prefix, Box::new(expr)))
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let infix = match self.current_token {
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Slash => Infix::Divide,
            Token::Asterisk => Infix::Multiply,
            Token::Equal => Infix::Equal,
            Token::NotEq => Infix::NotEqual,
            Token::Lt => Infix::LessThan,
            Token::Gt => Infix::GreaterThan,
            Token::LtEq => Infix::LessThanEqual,
            Token::GtEq => Infix::GreaterThanEqual,
            _ => return None,
        };

        let precedence = self.current_token_precedence();
        self.next_token();

        self.parse_expression(precedence)
            .map(|e| Expression::Infix(infix, Box::new(left), Box::new(e)))
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let exp = self.parse_expression(Precedence::Lowest);
        if !self.expect_peek(Token::RParen) {
            return None;
        }
        exp
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(Token::LParen) {
            return None;
        }

        self.next_token();

        let expr: Expression = match self.parse_expression(Precedence::Lowest) {
            Some(e) => e,
            None => return None,
        };

        if !self.expect_peek(Token::RParen) || !self.expect_peek(Token::LBrace) {
            return None;
        }

        let cons: Vec<Statement> = self.parse_block_statement();
        let mut alternative: Option<Vec<Statement>> = None;
        if self.peek_token_is(&Token::Else) {
            self.next_token();

            if self.peek_token_is(&Token::If) {
                self.next_token();
                let else_if = self.parse_if_expression();
                alternative = Some(vec![Statement::Expression(else_if.unwrap())]);
            } else if !self.expect_peek(Token::LBrace) {
                return None;
            } else {
                alternative = Some(self.parse_block_statement())
            };
        }

        Some(Expression::If {
            condition: Box::new(expr),
            consequence: cons,
            alternative,
        })
    }

    fn parse_fn_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(Token::LParen) {
            return None;
        }
        let params = match self.parse_fn_params() {
            Some(s) => s,
            None => return None,
        };
        let body = self.parse_block_statement();

        Some(Expression::Fn { params, body })
    }

    fn parse_fn_params(&mut self) -> Option<Vec<Ident>> {
        let mut idents: Vec<Ident> = vec![];
        if self.peek_token_is(&Token::RParen) {
            self.next_token();
            return Some(idents);
        }

        self.next_token();
        match self.current_token {
            Token::Ident(ref mut ident) => idents.push(Ident(ident.clone())),
            _ => {
                self.errors.push(String::from(
                    "Expected function parameter to be an identifier.",
                ));
                return None;
            }
        };

        match self.peek_token {
            Token::Comma | Token::RParen => (),
            _ => {
                self.errors.push(String::from(
                    "Expected function parameters to be seperated by a comma.`",
                ));
                return None;
            }
        }

        while self.peek_token_is(&Token::Comma) {
            self.next_token();
            self.next_token();
            match self.current_token {
                Token::Ident(ref mut ident) => idents.push(Ident(ident.clone())),
                _ => return None,
            };
        }

        Some(idents)
    }

    fn parse_call_expression(&mut self, left: Expression) -> Option<Expression> {
        let args = match self.parse_call_arguments() {
            Some(e) => e,
            None => return None,
        };

        Some(Expression::Call {
            function: Box::new(left),
            args,
        })
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Expression>> {
        let mut args: Vec<Expression> = vec![];

        if self.peek_token_is(&Token::RParen) {
            self.next_token();
            return Some(args);
        }

        self.next_token();
        match self.parse_expression(Precedence::Lowest) {
            Some(e) => args.push(e),
            None => return None,
        };

        while self.peek_token_is(&Token::Comma) {
            self.next_token();
            self.next_token();

            match self.parse_expression(Precedence::Lowest) {
                Some(e) => args.push(e),
                None => return None,
            };
        }

        if !self.expect_peek(Token::RParen) {
            return None;
        }

        Some(args)
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

    fn current_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.current_token)
    }

    fn next_token_precedence(&mut self) -> Precedence {
        Self::token_to_precedence(&self.peek_token)
    }
}
