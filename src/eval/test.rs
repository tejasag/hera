use std::{cell::RefCell, rc::Rc};

use super::{env::Env, object::Object, Eval};
use crate::{
    ast::{Expression, Ident, Infix, Literal, Statement},
    lexer::Lexer,
    parser::Parser,
};

#[macro_export]
macro_rules! test {
    ($x:expr) => {
        for (input, expect) in $x {
            let result = Eval::new(Rc::new(RefCell::new(Env::new())))
                .eval(Parser::new(Lexer::new(input.to_string())).parse_program());
            assert_eq!(result, expect)
        }
    };
}

#[test]
fn test_int_eval() {
    let tests = vec![("5", Some(Object::Int(5))), ("10", Some(Object::Int(10)))];
    test!(tests);
}

#[test]
fn test_bool_eval() {
    let tests = vec![
        ("true", Some(Object::Bool(true))),
        ("false", Some(Object::Bool(false))),
    ];

    test!(tests);
}

#[test]
fn test_string_eval() {
    let tests = vec![
        (
            "\"hello world\"",
            Some(Object::String(String::from("hello world"))),
        ),
        ("\"foobar\"", Some(Object::String(String::from("foobar")))),
    ];
    test!(tests);
}

#[test]
fn test_string_concatenation_eval() {
    let tests = vec![
        (
            "\"Hello \" + \"World\"",
            Some(Object::String(String::from("Hello World"))),
        ),
        (
            "\"Foo \" + \"Bar\"",
            Some(Object::String(String::from("Foo Bar"))),
        ),
    ];
    test!(tests);
}

#[test]
fn test_not_prefix_eval() {
    let tests = vec![
        ("!true", Some(Object::Bool(false))),
        ("!!true", Some(Object::Bool(true))),
        ("!1", Some(Object::Bool(false))),
    ];

    test!(tests);
}
#[test]
fn test_minus_prefix_eval() {
    let tests = vec![
        ("-5", Some(Object::Int(-5))),
        ("-3498", Some(Object::Int(-3498))),
    ];

    test!(tests);
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

    test!(tests);
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

    test!(tests);
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

    test!(tests);
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
        (
            "foobar",
            Some(Object::Error(String::from("identifier not found: foobar"))),
        ),
    ];

    test!(tests);
}

#[test]
fn test_let_statements() {
    let tests = vec![
        ("let a = 5; a", Some(Object::Int(5))),
        ("let a = 5 * 5; a", Some(Object::Int(25))),
        ("let a = 5; let b = a; b", Some(Object::Int(5))),
        (
            "let a = 5; let b = a; let c = a + b + 5; c;",
            Some(Object::Int(15)),
        ),
    ];

    test!(tests);
}

#[test]
fn test_fn_object() {
    let tests = vec![(
        "fn (x) {x+2}",
        Some(Object::Fn(
            vec![Ident(String::from("x"))],
            vec![Statement::Expression(Expression::Infix(
                Infix::Plus,
                Box::new(Expression::Ident(Ident(String::from("x")))),
                Box::new(Expression::Literal(Literal::Int(2))),
            ))],
            Rc::new(RefCell::new(Env::new())),
        )),
    )];

    test!(tests);
}

#[test]
fn test_fn_application() {
    let tests = vec![
        (
            "let identity = fn(x) { x; }; identity(5);",
            Some(Object::Int(5)),
        ),
        (
            "let identity = fn(x) { return x; }; identity(5);",
            Some(Object::Int(5)),
        ),
        (
            "let double = fn(x) { x * 2; }; double(5);",
            Some(Object::Int(10)),
        ),
        (
            "let add = fn(x, y) { x + y; }; add(5, 5);",
            Some(Object::Int(10)),
        ),
        (
            "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
            Some(Object::Int(20)),
        ),
        ("fn(x) { x; }(5)", Some(Object::Int(5))),
        (
            "fn(a) { let f = fn(b) { a + b }; f(a); }(5);",
            Some(Object::Int(10)),
        ),
    ];

    test!(tests);
}

#[test]
fn test_builtin_functions() {
    let tests = vec![
        ("len(\"\")", Some(Object::Int(0))),
        ("len(\"four\")", Some(Object::Int(4))),
        ("len(\"hello world\")", Some(Object::Int(11))),
        (
            "len(1)",
            Some(Object::Error(
                "argument to `len` not supported, got: 1".to_string(),
            )),
        ),
        (
            "len(\"one\", \"two\")",
            Some(Object::Error(String::from(
                "expected arguments: 1\ngiven arguments: 2",
            ))),
        ),
    ];
    test!(tests);
}

#[test]
fn test_array_eval() {
    let tests = vec![(
        "[1, 2 * 2, 3 + 3]",
        Some(Object::Array(vec![
            Object::Int(1),
            Object::Int(4),
            Object::Int(6),
        ])),
    )];
    test!(tests);
}

#[test]
fn test_array_index_eval() {
    let tests = vec![
        ("[1,2,3][0]", Some(Object::Int(1))),
        ("[1,2,3][1]", Some(Object::Int(2))),
        ("[1,2,3][2]", Some(Object::Int(3))),
        ("let i = 0; [1][i]", Some(Object::Int(1))),
        ("let arr = [1,2,3]; arr[2];", Some(Object::Int(3))),
        ("[1,2,3][1+1]", Some(Object::Int(3))),
        (
            "let arr = [1,2,3]; arr[1] + arr[0] + arr[2]",
            Some(Object::Int(6)),
        ),
        (
            "let arr = [1,2,3]; let i = arr[0]; arr[i]",
            Some(Object::Int(2)),
        ),
        ("[1,2,3][3]", Some(Object::Null)),
        ("[1,2,3][-1]", Some(Object::Int(3))),
        (
            "let arr = [1,2,3]; arr[-1 * (arr[1] - arr[0])]",
            Some(Object::Int(3)),
        ),
    ];
    test!(tests);
}
