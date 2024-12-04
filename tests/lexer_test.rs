use lisp::Lexer;
use lisp::Token;
use std::collections::VecDeque;

#[test]
fn tokenize_test() {
    assert_eq!(
        Lexer::tokenize("(define sqr (* x x))").unwrap(),
        vec![
            Token::LParen,
            Token::Define,
            Token::Symbol("sqr".into()),
            Token::LParen,
            Token::Symbol("*".into()),
            Token::Symbol("x".into()),
            Token::Symbol("x".into()),
            Token::RParen,
            Token::RParen,
        ]
        .into_iter()
        .collect::<VecDeque<Token>>(),
    );
}
