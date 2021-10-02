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
            Statement::Return(Expression::Literal(Literal::Int(1))),
            Statement::Return(Expression::Literal(Literal::Int(1))),
            Statement::Return(Expression::Literal(Literal::Int(1))),
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
