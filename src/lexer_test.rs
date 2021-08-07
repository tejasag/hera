use super::{
    lexer::New,
    token::{Token, TokenType},
};

macro_rules! lex_test {
    ($token_type:ident, $literal:expr) => {
        Token {
            token_type: TokenType::$token_type,
            literal: $literal,
        }
    };
}

#[test]
pub fn test_next_token() {
    let input = "let five = 55;
let ten = 10;
let add = fn(x, y) {x  + y;}
let result = add(five, ten);";

    let tests: Vec<Token<'_>> = vec![
        lex_test!(LET, "let"),
        lex_test!(IDENT, "five"),
        lex_test!(ASSIGN, "="),
        lex_test!(INT, "55"),
        lex_test!(SEMICOLON, ";"),
        lex_test!(LET, "let"),
        lex_test!(IDENT, "ten"),
        lex_test!(ASSIGN, "="),
        lex_test!(INT, "10"),
        lex_test!(SEMICOLON, ";"),
        lex_test!(LET, "let"),
        lex_test!(IDENT, "add"),
        lex_test!(ASSIGN, "="),
        lex_test!(FUNCTION, "fn"),
        lex_test!(LPAREN, "("),
        lex_test!(IDENT, "x"),
        lex_test!(COMMA, ","),
        lex_test!(IDENT, "y"),
        lex_test!(RPAREN, ")"),
        lex_test!(LBRACE, "{"),
        lex_test!(IDENT, "x"),
        lex_test!(PLUS, "+"),
        lex_test!(IDENT, "y"),
        lex_test!(SEMICOLON, ";"),
        lex_test!(RBRACE, "}"),
        lex_test!(LET, "let"),
        lex_test!(IDENT, "result"),
        lex_test!(ASSIGN, "="),
        lex_test!(IDENT, "add"),
        lex_test!(LPAREN, "("),
        lex_test!(IDENT, "five"),
        lex_test!(COMMA, ","),
        lex_test!(IDENT, "ten"),
        lex_test!(RPAREN, ")"),
        lex_test!(SEMICOLON, ";"),
        lex_test!(EOF, ""),
    ];

    let mut l = New(input.to_string());

    for expect in tests {
        let tok: Token = l.next_token();
        assert_eq!(expect, tok);
    }
}
