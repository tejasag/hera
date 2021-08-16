use super::ast::*;
use super::lexer::Lexer;
use super::parser::Parser;

#[test]
pub fn test_let_statement() {
    let input = "let x = 5;
                let y = 10;
                let foobar = 4345678";
    let l = Lexer::new(input.to_string());
    let p = Parser::new(l);

    let program = p.parse_program();
    assert_eq!(
        vec![
            Statement::LetExpression(
                Ident(String::from("x")),
                Expression::Literal(Literal::Int(5))
            ),
            Statement::LetExpression(
                Ident(String::from("y")),
                Expression::Literal(Literal::Int(10))
            ),
            Statement::LetExpression(
                Ident(String::from("foobar")),
                Expression::Literal(Literal::Int(4345678)),
            ),
        ],
        program,
    );
}
