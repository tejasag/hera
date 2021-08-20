use super::ast::*;
use super::lexer::Lexer;
use super::parser::Parser;

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
            Statement::LetExpression(
                Ident(String::from("x")),
                Expression::Literal(Literal::Int(5))
            ),
            Statement::LetExpression(
                Ident(String::from("y")),
                Expression::Literal(Literal::Int(5))
            ),
            Statement::LetExpression(
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
            Statement::ReturnExpression(Expression::Literal(Literal::Int(1))),
            Statement::ReturnExpression(Expression::Literal(Literal::Int(1))),
            Statement::ReturnExpression(Expression::Literal(Literal::Int(1))),
        ],
        program.statements
    )
}

fn check_parse_errors(p: Parser) {
    let errors = p.errors;

    if errors.len() == 0 {
        return;
    }

    panic!("Parser has {} errors!\n{:?}\n", errors.len(), errors);
}
