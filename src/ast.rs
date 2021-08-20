use std::fmt::{self, Formatter};

#[derive(PartialEq, Clone, Debug)]
pub struct Ident(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    String(String),
    Int(i32),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Ident(Ident),
    Literal(Literal),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    Let(Ident, Expression),
    Return(Expression),
    Expression(Expression),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
