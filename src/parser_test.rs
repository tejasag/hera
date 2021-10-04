use super::ast::*;
use super::lexer::Lexer;
use super::parser::Parser;

fn check_parse_errors(p: Parser) {
    let errors = p.errors;

    if errors.len() == 0 {
        return;
    }

    panic!("Parser has {} errors!\n{:?}\n", errors.len(), errors);
}

#[test]
pub fn test_let_statement() {
    let input = r#"
                    let x = 5;
                    let y = 5;
                    let foobar = 5;
                "#;
    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);

    let program = p.parse_program();
    check_parse_errors(p);

    assert_eq!(
        vec![
            Statement::Let(
                Ident(String::from("x")),
                Expression::Literal(Literal::Int(5))
            ),
            Statement::Let(
                Ident(String::from("y")),
                Expression::Literal(Literal::Int(5))
            ),
            Statement::Let(
                Ident(String::from("foobar")),
                Expression::Literal(Literal::Int(5)),
            ),
        ],
        program.statements,
    );
}

#[test]
pub fn test_return_statement() {
    let input = r#"
                  return 5;
                  return 10;
                  return 7894687;
              "#;

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);

    let program = p.parse_program();
    check_parse_errors(p);

    assert_eq!(
        vec![
            Statement::Return(Expression::Literal(Literal::Int(5))),
            Statement::Return(Expression::Literal(Literal::Int(10))),
            Statement::Return(Expression::Literal(Literal::Int(7894687))),
        ],
        program.statements
    )
}

#[test]
pub fn test_ident_expression() {
    let input: String = String::from("foo;");

    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    let program = p.parse_program();
    check_parse_errors(p);
    assert_eq!(
        vec![Statement::Expression(Expression::Ident(Ident(
            String::from("foo")
        )))],
        program.statements
    );
}

#[test]
pub fn test_int_literal_expression() {
    let input: String = String::from("5;");

    let l = Lexer::new(input);
    let mut p = Parser::new(l);

    let program = p.parse_program();
    check_parse_errors(p);
    assert_eq!(
        vec![Statement::Expression(Expression::Literal(Literal::Int(5)))],
        program.statements
    );
}

#[test]
pub fn test_bool_literal_expression() {
    let input = r#"
            true
            false
        "#;

    let l = Lexer::new(input.to_string());
    let mut p = Parser::new(l);

    let program = p.parse_program();
    check_parse_errors(p);
    assert_eq!(
        vec![
            Statement::Expression(Expression::Literal(Literal::Bool(true))),
            Statement::Expression(Expression::Literal(Literal::Bool(false)))
        ],
        program.statements
    );
}

#[test]
pub fn test_prefix_expression() {
    let tests: Vec<Statement> = vec![
        Statement::Expression(Expression::Prefix(
            Prefix::Not,
            Box::new(Expression::Literal(Literal::Int(5))),
        )),
        Statement::Expression(Expression::Prefix(
            Prefix::Minus,
            Box::new(Expression::Literal(Literal::Int(5))),
        )),
        Statement::Expression(Expression::Prefix(
            Prefix::Plus,
            Box::new(Expression::Literal(Literal::Int(5))),
        )),
    ];

    let input = r#"
        !5;
        -5;
        +5;
    "#;

    let mut parser = Parser::new(Lexer::new(input.to_string()));
    let program = parser.parse_program();

    check_parse_errors(parser);
    assert_eq!(tests, program.statements);
}

#[test]
pub fn test_infix_expression() {
    let tests: Vec<Statement> = vec![
        Statement::Expression(Expression::Infix(
            Infix::Plus,
            Box::new(Expression::Literal(Literal::Int(5))),
            Box::new(Expression::Literal(Literal::Int(5))),
        )),
        Statement::Expression(Expression::Infix(
            Infix::Minus,
            Box::new(Expression::Literal(Literal::Int(5))),
            Box::new(Expression::Literal(Literal::Int(5))),
        )),
        Statement::Expression(Expression::Infix(
            Infix::Multiply,
            Box::new(Expression::Literal(Literal::Int(5))),
            Box::new(Expression::Literal(Literal::Int(5))),
        )),
        Statement::Expression(Expression::Infix(
            Infix::Divide,
            Box::new(Expression::Literal(Literal::Int(5))),
            Box::new(Expression::Literal(Literal::Int(5))),
        )),
        Statement::Expression(Expression::Infix(
            Infix::LessThan,
            Box::new(Expression::Literal(Literal::Int(5))),
            Box::new(Expression::Literal(Literal::Int(5))),
        )),
        Statement::Expression(Expression::Infix(
            Infix::GreaterThan,
            Box::new(Expression::Literal(Literal::Int(5))),
            Box::new(Expression::Literal(Literal::Int(5))),
        )),
        Statement::Expression(Expression::Infix(
            Infix::Equal,
            Box::new(Expression::Literal(Literal::Int(5))),
            Box::new(Expression::Literal(Literal::Int(5))),
        )),
        Statement::Expression(Expression::Infix(
            Infix::NotEqual,
            Box::new(Expression::Literal(Literal::Int(5))),
            Box::new(Expression::Literal(Literal::Int(5))),
        )),
    ];

    let input = r#"
        5+5;
        5-5;
        5*5;
        5/5;
        5<5;
        5>5;
        5==5;
        5!=5;
    "#;

    let mut parser = Parser::new(Lexer::new(input.to_string()));
    let program = parser.parse_program();

    check_parse_errors(parser);
    assert_eq!(tests, program.statements);
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = vec![
        (
            "-a * b",
            Statement::Expression(Expression::Infix(
                Infix::Multiply,
                Box::new(Expression::Prefix(
                    Prefix::Minus,
                    Box::new(Expression::Ident(Ident(String::from("a")))),
                )),
                Box::new(Expression::Ident(Ident(String::from("b")))),
            )),
        ),
        (
            "!-a",
            Statement::Expression(Expression::Prefix(
                Prefix::Not,
                Box::new(Expression::Prefix(
                    Prefix::Minus,
                    Box::new(Expression::Ident(Ident(String::from("a")))),
                )),
            )),
        ),
        (
            "a+b+c",
            Statement::Expression(Expression::Infix(
                Infix::Plus,
                Box::new(Expression::Infix(
                    Infix::Plus,
                    Box::new(Expression::Ident(Ident(String::from("a")))),
                    Box::new(Expression::Ident(Ident(String::from("b")))),
                )),
                Box::new(Expression::Ident(Ident(String::from("c")))),
            )),
        ),
        (
            "a+b-c",
            Statement::Expression(Expression::Infix(
                Infix::Minus,
                Box::new(Expression::Infix(
                    Infix::Plus,
                    Box::new(Expression::Ident(Ident(String::from("a")))),
                    Box::new(Expression::Ident(Ident(String::from("b")))),
                )),
                Box::new(Expression::Ident(Ident(String::from("c")))),
            )),
        ),
        (
            "a*b*c",
            Statement::Expression(Expression::Infix(
                Infix::Multiply,
                Box::new(Expression::Infix(
                    Infix::Multiply,
                    Box::new(Expression::Ident(Ident(String::from("a")))),
                    Box::new(Expression::Ident(Ident(String::from("b")))),
                )),
                Box::new(Expression::Ident(Ident(String::from("c")))),
            )),
        ),
        (
            "a*b/c",
            Statement::Expression(Expression::Infix(
                Infix::Divide,
                Box::new(Expression::Infix(
                    Infix::Multiply,
                    Box::new(Expression::Ident(Ident(String::from("a")))),
                    Box::new(Expression::Ident(Ident(String::from("b")))),
                )),
                Box::new(Expression::Ident(Ident(String::from("c")))),
            )),
        ),
        (
            "a/b+c",
            Statement::Expression(Expression::Infix(
                Infix::Plus,
                Box::new(Expression::Infix(
                    Infix::Divide,
                    Box::new(Expression::Ident(Ident(String::from("a")))),
                    Box::new(Expression::Ident(Ident(String::from("b")))),
                )),
                Box::new(Expression::Ident(Ident(String::from("c")))),
            )),
        ),
        (
            "a+b/c",
            Statement::Expression(Expression::Infix(
                Infix::Plus,
                Box::new(Expression::Ident(Ident(String::from("a")))),
                Box::new(Expression::Infix(
                    Infix::Divide,
                    Box::new(Expression::Ident(Ident(String::from("b")))),
                    Box::new(Expression::Ident(Ident(String::from("c")))),
                )),
            )),
        ),
        (
            // "(((a + (b * c)) + (d / e)) - f)",
            "a + b * c + d / e - f",
            Statement::Expression(Expression::Infix(
                Infix::Minus,
                Box::new(Expression::Infix(
                    Infix::Plus,
                    Box::new(Expression::Infix(
                        Infix::Plus,
                        Box::new(Expression::Ident(Ident(String::from("a")))),
                        Box::new(Expression::Infix(
                            Infix::Multiply,
                            Box::new(Expression::Ident(Ident(String::from("b")))),
                            Box::new(Expression::Ident(Ident(String::from("c")))),
                        )),
                    )),
                    Box::new(Expression::Infix(
                        Infix::Divide,
                        Box::new(Expression::Ident(Ident(String::from("d")))),
                        Box::new(Expression::Ident(Ident(String::from("e")))),
                    )),
                )),
                Box::new(Expression::Ident(Ident(String::from("f")))),
            )),
        ),
        (
            "5 > 4 == 3 < 4",
            Statement::Expression(Expression::Infix(
                Infix::Equal,
                Box::new(Expression::Infix(
                    Infix::GreaterThan,
                    Box::new(Expression::Literal(Literal::Int(5))),
                    Box::new(Expression::Literal(Literal::Int(4))),
                )),
                Box::new(Expression::Infix(
                    Infix::LessThan,
                    Box::new(Expression::Literal(Literal::Int(3))),
                    Box::new(Expression::Literal(Literal::Int(4))),
                )),
            )),
        ),
        (
            "5 < 4 != 3 > 4",
            Statement::Expression(Expression::Infix(
                Infix::NotEqual,
                Box::new(Expression::Infix(
                    Infix::LessThan,
                    Box::new(Expression::Literal(Literal::Int(5))),
                    Box::new(Expression::Literal(Literal::Int(4))),
                )),
                Box::new(Expression::Infix(
                    Infix::GreaterThan,
                    Box::new(Expression::Literal(Literal::Int(3))),
                    Box::new(Expression::Literal(Literal::Int(4))),
                )),
            )),
        ),
        (
            "5 >= 4 == 3 <= 4",
            Statement::Expression(Expression::Infix(
                Infix::Equal,
                Box::new(Expression::Infix(
                    Infix::GreaterThanEqual,
                    Box::new(Expression::Literal(Literal::Int(5))),
                    Box::new(Expression::Literal(Literal::Int(4))),
                )),
                Box::new(Expression::Infix(
                    Infix::LessThanEqual,
                    Box::new(Expression::Literal(Literal::Int(3))),
                    Box::new(Expression::Literal(Literal::Int(4))),
                )),
            )),
        ),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            Statement::Expression(Expression::Infix(
                Infix::Equal,
                Box::new(Expression::Infix(
                    Infix::Plus,
                    Box::new(Expression::Literal(Literal::Int(3))),
                    Box::new(Expression::Infix(
                        Infix::Multiply,
                        Box::new(Expression::Literal(Literal::Int(4))),
                        Box::new(Expression::Literal(Literal::Int(5))),
                    )),
                )),
                Box::new(Expression::Infix(
                    Infix::Plus,
                    Box::new(Expression::Infix(
                        Infix::Multiply,
                        Box::new(Expression::Literal(Literal::Int(3))),
                        Box::new(Expression::Literal(Literal::Int(1))),
                    )),
                    Box::new(Expression::Infix(
                        Infix::Multiply,
                        Box::new(Expression::Literal(Literal::Int(4))),
                        Box::new(Expression::Literal(Literal::Int(5))),
                    )),
                )),
            )),
        ),
        (
            "true",
            Statement::Expression(Expression::Literal(Literal::Bool(true))),
        ),
        (
            "false",
            Statement::Expression(Expression::Literal(Literal::Bool(false))),
        ),
        (
            "3 > 5 == false",
            Statement::Expression(Expression::Infix(
                Infix::Equal,
                Box::new(Expression::Infix(
                    Infix::GreaterThan,
                    Box::new(Expression::Literal(Literal::Int(3))),
                    Box::new(Expression::Literal(Literal::Int(5))),
                )),
                Box::new(Expression::Literal(Literal::Bool(false))),
            )),
        ),
        (
            "3 < 5 == true",
            Statement::Expression(Expression::Infix(
                Infix::Equal,
                Box::new(Expression::Infix(
                    Infix::LessThan,
                    Box::new(Expression::Literal(Literal::Int(3))),
                    Box::new(Expression::Literal(Literal::Int(5))),
                )),
                Box::new(Expression::Literal(Literal::Bool(true))),
            )),
        ),
        (
            "a + (b + c)",
            Statement::Expression(Expression::Infix(
                Infix::Plus,
                Box::new(Expression::Ident(Ident(String::from("a")))),
                Box::new(Expression::Infix(
                    Infix::Plus,
                    Box::new(Expression::Ident(Ident(String::from("b")))),
                    Box::new(Expression::Ident(Ident(String::from("c")))),
                )),
            )),
        ),
    ];

    for (input, expect) in tests {
        let mut parser = Parser::new(Lexer::new(input.to_string()));
        let program = parser.parse_program();

        check_parse_errors(parser);
        assert_eq!(vec![expect], program.statements);
    }
}

#[test]
pub fn test_if_expression() {
    let tests: Vec<(&str, Statement)> = vec![
        (
            "if ( x > y ) { x }",
            Statement::Expression(Expression::If {
                condition: Box::new(Expression::Infix(
                    Infix::GreaterThan,
                    Box::new(Expression::Ident(Ident(String::from("x")))),
                    Box::new(Expression::Ident(Ident(String::from("y")))),
                )),
                consequence: vec![Statement::Expression(Expression::Ident(Ident(
                    String::from("x"),
                )))],
                alternative: None,
            }),
        ),
        (
            "if ((5 * 5) + 5 >= 30) { true }",
            Statement::Expression(Expression::If {
                condition: Box::new(Expression::Infix(
                    Infix::GreaterThanEqual,
                    Box::new(Expression::Infix(
                        Infix::Plus,
                        Box::new(Expression::Infix(
                            Infix::Multiply,
                            Box::new(Expression::Literal(Literal::Int(5))),
                            Box::new(Expression::Literal(Literal::Int(5))),
                        )),
                        Box::new(Expression::Literal(Literal::Int(5))),
                    )),
                    Box::new(Expression::Literal(Literal::Int(30))),
                )),
                consequence: vec![Statement::Expression(Expression::Literal(Literal::Bool(
                    true,
                )))],
                alternative: None,
            }),
        ),
    ];

    for (input, expect) in tests {
        let mut parser = Parser::new(Lexer::new(input.to_string()));
        let program = parser.parse_program();

        check_parse_errors(parser);
        assert_eq!(vec![expect], program.statements);
    }
}

#[test]
pub fn test_if_else_expression() {
    let tests: Vec<(&str, Statement)> = vec![
        (
            "if ( x > y ) { x } else { y }",
            Statement::Expression(Expression::If {
                condition: Box::new(Expression::Infix(
                    Infix::GreaterThan,
                    Box::new(Expression::Ident(Ident(String::from("x")))),
                    Box::new(Expression::Ident(Ident(String::from("y")))),
                )),
                consequence: vec![Statement::Expression(Expression::Ident(Ident(
                    String::from("x"),
                )))],
                alternative: Some(vec![Statement::Expression(Expression::Ident(Ident(
                    String::from("y"),
                )))]),
            }),
        ),
        (
            "if ( x > y ) { x } else if (y == x) { 1 } else { y }",
            Statement::Expression(Expression::If {
                condition: Box::new(Expression::Infix(
                    Infix::GreaterThan,
                    Box::new(Expression::Ident(Ident(String::from("x")))),
                    Box::new(Expression::Ident(Ident(String::from("y")))),
                )),
                consequence: vec![Statement::Expression(Expression::Ident(Ident(
                    String::from("x"),
                )))],
                alternative: Some(vec![Statement::Expression(Expression::If {
                    condition: Box::new(Expression::Infix(
                        Infix::Equal,
                        Box::new(Expression::Ident(Ident(String::from("y")))),
                        Box::new(Expression::Ident(Ident(String::from("x")))),
                    )),
                    consequence: vec![Statement::Expression(Expression::Literal(Literal::Int(1)))],
                    alternative: Some(vec![Statement::Expression(Expression::Ident(Ident(
                        String::from("y"),
                    )))]),
                })]),
            }),
        ),
        (
            r#"
             if ((5 * 5) + 5 >= 30) { 
                true
             } else {
                return 5 + (89 * 64 / 10);
             }
            "#,
            Statement::Expression(Expression::If {
                condition: Box::new(Expression::Infix(
                    Infix::GreaterThanEqual,
                    Box::new(Expression::Infix(
                        Infix::Plus,
                        Box::new(Expression::Infix(
                            Infix::Multiply,
                            Box::new(Expression::Literal(Literal::Int(5))),
                            Box::new(Expression::Literal(Literal::Int(5))),
                        )),
                        Box::new(Expression::Literal(Literal::Int(5))),
                    )),
                    Box::new(Expression::Literal(Literal::Int(30))),
                )),
                consequence: vec![Statement::Expression(Expression::Literal(Literal::Bool(
                    true,
                )))],
                alternative: Some(vec![Statement::Return(Expression::Infix(
                    Infix::Plus,
                    Box::new(Expression::Literal(Literal::Int(5))),
                    Box::new(Expression::Infix(
                        Infix::Divide,
                        Box::new(Expression::Infix(
                            Infix::Multiply,
                            Box::new(Expression::Literal(Literal::Int(89))),
                            Box::new(Expression::Literal(Literal::Int(64))),
                        )),
                        Box::new(Expression::Literal(Literal::Int(10))),
                    )),
                ))]),
            }),
        ),
    ];

    for (input, expect) in tests {
        let mut parser = Parser::new(Lexer::new(input.to_string()));
        let program = parser.parse_program();

        check_parse_errors(parser);
        assert_eq!(vec![expect], program.statements);
    }
}
