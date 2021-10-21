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

    fn is_truthy(&mut self, object: Object) -> bool {
        !matches!(object, Object::Null | Object::Bool(false))
    }

    /*    fn is_error(&mut self, object: Object) -> bool {
            matches!(object, Object::Error(_))
        }
    */

    pub fn eval(&mut self, program: Program) -> Option<Object> {
        let mut result = None;

        for statement in program.statements {
            match self.eval_statement(statement) {
                Some(Object::Return(val)) => return Some(*val),
                e => result = e,
            }
        }

        result
    }

    fn eval_statement(&mut self, statement: Statement) -> Option<Object> {
        match statement {
            Statement::Expression(e) => self.eval_expr(e),
            Statement::Return(e) => {
                let val = match self.eval_expr(e) {
                    Some(v) => v,
                    None => return None,
                };

                Some(Object::Return(Box::new(val)))
            }
            _ => None,
        }
    }

    fn eval_block_statement(&mut self, statements: BlockStatement) -> Option<Object> {
        let mut result = None;

        for statement in statements {
            match self.eval_statement(statement) {
                Some(Object::Return(e)) => return Some(Object::Return(e)),
                e => result = e,
            }
        }

        result
    }

    fn eval_expr(&mut self, expr: Expression) -> Option<Object> {
        match expr {
            Expression::Ident(ident) => Some(self.eval_ident(ident)),
            Expression::Literal(lit) => Some(self.eval_literal(lit)),
            Expression::Prefix(prefix, right) => self
                .eval_expr(*right)
                .map(|expr| self.eval_prefix_expr(prefix, expr)),
            Expression::Infix(infix, left, right) => {
                let left_expr = self.eval_expr(*left);
                let right_expr = self.eval_expr(*right);
                if let Some(l) = left_expr {
                    right_expr.map(|r| self.eval_infix_expr(infix, l, r))
                } else {
                    None
                }
            }
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                let cond_expr = match self.eval_expr(*condition) {
                    Some(e) => e,
                    None => return None,
                };

                if self.is_truthy(cond_expr) {
                    self.eval_block_statement(consequence)
                } else if let Some(a) = alternative {
                    self.eval_block_statement(a)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn eval_prefix_expr(&mut self, prefix: Prefix, expr: Object) -> Object {
        match prefix {
            Prefix::Not => self.eval_not_prefix_expr(expr),
            Prefix::Minus => self.eval_minus_prefix_expr(expr),
            Prefix::Plus => self.eval_plus_prefix_expr(expr),
        }
    }

    fn eval_not_prefix_expr(&mut self, expr: Object) -> Object {
        match expr {
            Object::Bool(true) => Object::Bool(false),
            Object::Bool(false) => Object::Bool(true),
            Object::Null => Object::Bool(true),
            _ => Object::Bool(false),
        }
    }

    fn eval_minus_prefix_expr(&mut self, expr: Object) -> Object {
        match expr {
            Object::Int(i) => Object::Int(-i),
            _ => Object::Error(format!("unknown operator: -{}", expr)),
        }
    }

    fn eval_plus_prefix_expr(&mut self, expr: Object) -> Object {
        match expr {
            Object::Int(i) => Object::Int(i),
            _ => Object::Error(format!("unknown operator: {}", expr)),
        }
    }

    fn eval_infix_expr(&mut self, infix: Infix, left: Object, right: Object) -> Object {
        match left {
            Object::Int(left_expr) => {
                if let Object::Int(right_expr) = right {
                    self.eval_int_infix_expr(infix, left_expr, right_expr)
                } else {
                    Object::Error(format!("type mismatch: {} {} {}", left, infix, right))
                }
            }
            _ => Object::Error(format!("unknown operator: {} {} {}", left, infix, right)),
        }
    }

    fn eval_int_infix_expr(&mut self, infix: Infix, left: i32, right: i32) -> Object {
        match infix {
            Infix::Plus => Object::Int(left + right),
            Infix::Minus => Object::Int(left - right),
            Infix::Multiply => Object::Int(left * right),
            Infix::Divide => Object::Int(left / right),
            Infix::LessThan => Object::Bool(left < right),
            Infix::LessThanEqual => Object::Bool(left <= right),
            Infix::GreaterThan => Object::Bool(left > right),
            Infix::GreaterThanEqual => Object::Bool(left >= right),
            Infix::Equal => Object::Bool(left == right),
            Infix::NotEqual => Object::Bool(left != right),
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
