use crate::{
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
\"foobar\"
\"foo bar\"
[1,2];
";

    let tests: Vec<Token> = vec![
        Let,
        Ident("five".to_string()),
        Assign,
        Int(55),
        SemiColon,
        Let,
        Ident("ten".to_string()),
        Assign,
        Int(10),
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
        Int(5),
        SemiColon,
        Int(5),
        Lt,
        Int(10),
        Gt,
        Int(5),
        SemiColon,
        If,
        LParen,
        Int(5),
        Lt,
        Int(10),
        RParen,
        LBrace,
        Return,
        Bool(true),
        SemiColon,
        RBrace,
        Else,
        LBrace,
        Return,
        Bool(false),
        SemiColon,
        RBrace,
        Int(10),
        Equal,
        Int(10),
        SemiColon,
        Int(9),
        NotEq,
        Int(10),
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
        Str(String::from("foobar")),
        Str(String::from("foo bar")),
        LBracket,
        Int(1),
        Comma,
        Int(2),
        RBracket,
        SemiColon,
    ];

    let mut l = Lexer::new(input.to_string());

    for expect in tests {
        let tok: Token = l.next_token();
        assert_eq!(expect, tok);
    }
}
