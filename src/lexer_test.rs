use super::{
    lexer::New,
    token::Token::{self, *},
};

#[test]
pub fn test_next_token() {
    let input = "let five = 55;
let ten = 10;

let add = fn(x, y) {
    x  + y;
};

!-/*5;
5 < 10 > 5;

let result = add(five, ten);
";

    let tests: Vec<Token> = vec![
        LET,
        IDENT("five".to_string()),
        ASSIGN,
        INT("55".to_string()),
        SEMICOLON,
        LET,
        IDENT("ten".to_string()),
        ASSIGN,
        INT("10".to_string()),
        SEMICOLON,
        LET,
        IDENT("add".to_string()),
        ASSIGN,
        FUNCTION,
        LPAREN,
        IDENT("x".to_string()),
        COMMA,
        IDENT("y".to_string()),
        RPAREN,
        LBRACE,
        IDENT("x".to_string()),
        PLUS,
        IDENT("y".to_string()),
        SEMICOLON,
        RBRACE,
        SEMICOLON,
        BANG,
        MINUS,
        SLASH,
        ASTERISK,
        INT("5".to_string()),
        SEMICOLON,
        INT("5".to_string()),
        LT,
        INT("10".to_string()),
        GT,
        INT("5".to_string()),
        SEMICOLON,
        LET,
        IDENT("result".to_string()),
        ASSIGN,
        IDENT("add".to_string()),
        LPAREN,
        IDENT("five".to_string()),
        COMMA,
        IDENT("ten".to_string()),
        RPAREN,
        SEMICOLON,
    ];

    let mut l = New(input.to_string());

    for expect in tests {
        let tok: Token = l.next_token();
        assert_eq!(expect, tok);
    }
}
