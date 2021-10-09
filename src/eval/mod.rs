pub mod object;

use crate::ast::*;
use object::Object;

#[cfg(test)]
pub mod test;

pub struct Eval;

impl Eval {
    pub fn new() -> Self {
        Eval {}
    }

    pub fn eval(&mut self, program: Program) -> Option<Object> {
        let mut result = None;

        for statement in program.statements {
            /*match self.eval_statement(statement) {
                e => result = e,
            }
            */
            let e = self.eval_statement(statement);
            result = e;
        }

        result
    }

    fn eval_statement(&mut self, statement: Statement) -> Option<Object> {
        let result;

        match statement {
            Statement::Expression(e) => result = self.eval_expr(e),
            _ => return None,
        };

        result
    }

    fn eval_expr(&mut self, expr: Expression) -> Option<Object> {
        let result;

        match expr {
            Expression::Ident(ident) => result = Some(self.eval_ident(ident)),
            Expression::Literal(lit) => result = Some(self.eval_literal(lit)),
            _ => result = None,
        }

        result
    }

    fn eval_ident(&mut self, ident: Ident) -> Object {
        let Ident(i) = ident;
        Object::String(i)
    }

    fn eval_literal(&mut self, lit: Literal) -> Object {
        match lit {
            Literal::String(s) => Object::String(s),
            Literal::Int(i) => Object::Int(i),
            _ => Object::String(String::from(".keep")),
        }
    }
}

impl Default for Eval {
    fn default() -> Self {
        Self::new()
    }
}
