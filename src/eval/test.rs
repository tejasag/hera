use super::{object::Object, Eval};
use crate::{ast::*, lexer::Lexer, parser::Parser};

#[test]
fn test_int_eval() {
    let tests = vec![("5", Some(Object::Int(5))), ("10", Some(Object::Int(10)))];

    for (input, expect) in tests {
        let mut parser = Parser::new(Lexer::new(input.to_string()));
        let program = parser.parse_program();
        let mut evaluator = Eval::new();
        let result = evaluator.eval(program);

        assert_eq!(result, expect);
    }
}
