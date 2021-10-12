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
        match expr {
            Expression::Ident(ident) => Some(self.eval_ident(ident)),
            Expression::Literal(lit) => Some(self.eval_literal(lit)),
            Expression::Prefix(prefix, right) => {
                if let Some(expr) = self.eval_expr(*right) {
                    Some(self.eval_prefix_expr(prefix, expr))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn eval_prefix_expr(&mut self, prefix: Prefix, expr: Object) -> Object {
        match prefix {
            Prefix::Not => self.parse_not_prefix_expr(expr),
            Prefix::Minus => self.parse_minus_prefix_expr(expr),
            Prefix::Plus => self.parse_minus_prefix_expr(expr), // TODO: fix this
        }
    }

    fn parse_not_prefix_expr(&mut self, expr: Object) -> Object {
        match expr {
            Object::Bool(true) => Object::Bool(false),
            Object::Bool(false) => Object::Bool(true),
            Object::Null => Object::Bool(true),
            _ => Object::Bool(false),
        }
    }

    fn parse_minus_prefix_expr(&mut self, expr: Object) -> Object {
        match expr {
            Object::Int(ref i) => Object::Int(-1 * i),
            _ => Object::Null,
        }
    }

    fn eval_ident(&mut self, ident: Ident) -> Object {
        let Ident(i) = ident;
        Object::String(i)
    }

    fn eval_literal(&mut self, lit: Literal) -> Object {
        match lit {
            Literal::String(s) => Object::String(s),
            Literal::Int(i) => Object::Int(i),
            Literal::Bool(b) => Object::Bool(b),
        }
    }
}

impl Default for Eval {
    fn default() -> Self {
        Self::new()
    }
}
