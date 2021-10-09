pub mod object;

use crate::ast::*;
use object::Object;

#[cfg(test)]
pub mod test;

pub struct Eval;

impl Eval {
    pub fn eval(&mut self, program: Program) -> Option<Object> {
        let mut result = None;

        for statement in program.statements {
            match self.ev
        }
        
        result
    }

    fn eval_statement(&mut self, statement: Statement) -> Option<Object> {
        match statement {
            Statement::Let(ident, expr) => {
                let val = match self.eval_expr(expr) {
                    Some(val) => val,
                    None => return None,
                }
            },
            _ => return None,
        };
    }

    fn eval_expr(&mut self, expr: Expression) -> Option<Object> {


        None
    }
}
