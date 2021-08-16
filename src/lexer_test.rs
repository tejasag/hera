use super::{
    lexer::Lexer,
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

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
9 != 10;

let result = add(five, ten);
";

    let tests: Vec<Token> = vec![
        Let,
        Ident("five".to_string()),
        Assign,
        Int("55".to_string()),
        SemiColon,
        Let,
        Ident("ten".to_string()),
        Assign,
        Int("10".to_string()),
        SemiColon,
        Let,
        Ident("add".to_string()),
        Assign,
        Function,
        LParen,
        Ident("x".to_string()),
        Comma,
        Ident("y".to_string()),
        RParen,
        LBrace,
        Ident("x".to_string()),
        Plus,
        Ident("y".to_string()),
        SemiColon,
        RBrace,
        SemiColon,
        Bang,
        Minus,
        Slash,
        Asterisk,
        Int("5".to_string()),
        SemiColon,
        Int("5".to_string()),
        Lt,
        Int("10".to_string()),
        Gt,
        Int("5".to_string()),
        SemiColon,
        If,
        LParen,
        Int("5".to_string()),
        Lt,
        Int("10".to_string()),
        RParen,
        LBrace,
        Return,
        True,
        SemiColon,
        RBrace,
        Else,
        LBrace,
        Return,
        False,
        SemiColon,
        RBrace,
        Int("10".to_string()),
        Eq,
        Int("10".to_string()),
        SemiColon,
        Int("9".to_string()),
        NotEq,
        Int("10".to_string()),
        SemiColon,
        Let,
        Ident("result".to_string()),
        Assign,
        Ident("add".to_string()),
        LParen,
        Ident("five".to_string()),
        Comma,
        Ident("ten".to_string()),
        RParen,
        SemiColon,
    ];

    let mut l = Lexer::new(input.to_string());

    for expect in tests {
        let tok: Token = l.next_token();
        assert_eq!(expect, tok);
    }
}
