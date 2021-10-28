use super::{object::Object, Eval};
use crate::{lexer::Lexer, parser::Parser};

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

#[test]
fn test_bool_eval() {
    let tests = vec![
        ("true", Some(Object::Bool(true))),
        ("false", Some(Object::Bool(false))),
    ];

    for (input, expect) in tests {
        let mut parser = Parser::new(Lexer::new(input.to_string()));
        let program = parser.parse_program();
        let mut evaluator = Eval::new();
        let result = evaluator.eval(program);

        assert_eq!(result, expect);
    }
}

#[test]
fn test_not_prefix_eval() {
    let tests = vec![
        ("!true", Some(Object::Bool(false))),
        ("!!true", Some(Object::Bool(true))),
        ("!1", Some(Object::Bool(false))),
    ];

    for (input, expect) in tests {
        let mut parser = Parser::new(Lexer::new(input.to_string()));
        let program = parser.parse_program();
        let mut evaluator = Eval::new();
        let result = evaluator.eval(program);

        assert_eq!(result, expect);
    }
}
#[test]
fn test_minus_prefix_eval() {
    let tests = vec![
        ("-5", Some(Object::Int(-5))),
        ("-3498", Some(Object::Int(-3498))),
    ];

    for (input, expect) in tests {
        let mut parser = Parser::new(Lexer::new(input.to_string()));
        let program = parser.parse_program();
        let mut evaluator = Eval::new();
        let result = evaluator.eval(program);

        assert_eq!(result, expect);
    }
}

#[test]
fn test_int_infix_eval() {
    let tests = vec![
        ("5 + 5 + 5 + 5 - 10", Some(Object::Int(10))),
        ("2 * 2 * 2 * 2 * 2", Some(Object::Int(32))),
        ("-50 + 100 + -50", Some(Object::Int(0))),
        ("5 * 2 + 10", Some(Object::Int(20))),
        ("5 + 2 * 10", Some(Object::Int(25))),
        ("20 + 2 * -10", Some(Object::Int(0))),
        ("50 / 2 * 2 + 10", Some(Object::Int(60))),
        ("2 * (5 + 10)", Some(Object::Int(30))),
        ("3 * 3 * 3 + 10", Some(Object::Int(37))),
        ("3 * (3 * 3) + 10", Some(Object::Int(37))),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", Some(Object::Int(50))),
    ];

    for (input, expect) in tests {
        let mut parser = Parser::new(Lexer::new(input.to_string()));
        let program = parser.parse_program();
        let mut evaluator = Eval::new();
        let result = evaluator.eval(program);

        assert_eq!(result, expect);
    }
}

#[test]
fn test_if_eval() {
    let tests = vec![
        ("if (true) { 10 }", Some(Object::Int(10))),
        ("if (false) { 10 }", None),
        ("if (1) { 10 }", Some(Object::Int(10))),
        ("if (1 < 2) { 10 }", Some(Object::Int(10))),
        ("if (1 > 2) { 10 }", None),
        ("if (1 > 2) { 10 } else { 20 }", Some(Object::Int(20))),
        ("if (1 < 2) { 10 } else { 20 }", Some(Object::Int(10))),
        ("if (1 <= 2) { 10 }", Some(Object::Int(10))),
        ("if (1 >= 2) { 10 }", None),
        ("if (1 >= 2) { 10 } else { 20 }", Some(Object::Int(20))),
        ("if (1 <= 2) { 10 } else { 20 }", Some(Object::Int(10))),
    ];

    for (input, expect) in tests {
        let mut parser = Parser::new(Lexer::new(input.to_string()));
        let program = parser.parse_program();
        let mut evaluator = Eval::new();
        let result = evaluator.eval(program);

        assert_eq!(result, expect);
    }
}

#[test]
fn test_return_eval() {
    let tests = vec![
        ("return 10;", Some(Object::Int(10))),
        ("return 10; 9;", Some(Object::Int(10))),
        ("return 2 * 5; 9;", Some(Object::Int(10))),
        ("9; return 2 * 5; 9;", Some(Object::Int(10))),
        (
            r#"
if (10 > 1) {
  if (10 > 1) {
    return 10;
  }
  return 1;
}"#,
            Some(Object::Int(10)),
        ),
    ];

    for (input, expect) in tests {
        let mut parser = Parser::new(Lexer::new(input.to_string()));
        let program = parser.parse_program();
        let mut evaluator = Eval::new();
        let result = evaluator.eval(program);

        assert_eq!(result, expect);
    }
}

#[test]
fn test_error_handling() {
    let tests = vec![
        (
            "5 + true",
            Some(Object::Error(String::from("type mismatch: 5 + true"))),
        ),
        (
            "5 + true; 5; ",
            Some(Object::Error(String::from("type mismatch: 5 + true"))),
        ),
        (
            "-true",
            Some(Object::Error(String::from("unknown operator: -true"))),
        ),
        (
            "true + false",
            Some(Object::Error(String::from(
                "unknown operator: true + false",
            ))),
        ),
        (
            "5; true + false; 5",
            Some(Object::Error(String::from(
                "unknown operator: true + false",
            ))),
        ),
        (
            "if (10 > 1) { true + false; }",
            Some(Object::Error(String::from(
                "unknown operator: true + false",
            ))),
        ),
        (
            "if (10 > 1) {
                if (10 > 1) {
                    return true + false;
                }
                return 1;
             }",
            Some(Object::Error(String::from(
                "unknown operator: true + false",
            ))),
        ),
        (
            "1+(true+false)",
            Some(Object::Error(String::from(
                "unknown operator: true + false",
            ))),
        ),
        (
            "(true+false)+1",
            Some(Object::Error(String::from(
                "unknown operator: true + false",
            ))),
        ),
        (
            "(true+false)+(true+false)",
            Some(Object::Error(String::from(
                "unknown operator: true + false",
            ))),
        ),
    ];

    for (input, expect) in tests {
        let mut parser = Parser::new(Lexer::new(input.to_string()));
        let program = parser.parse_program();
        let mut evaluator = Eval::new();
        let result = evaluator.eval(program);

        assert_eq!(result, expect);
    }
}
