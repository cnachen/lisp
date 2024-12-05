use std::collections::VecDeque;
use std::convert::AsRef;
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Integer(i32),
    Symbol(String),
    LParen,
    RParen,
    Nil,
    True,
    False,
    Lambda,
    Apply,
    Define,
    Cond,
}

pub struct Lexer;

impl Lexer {
    pub fn tokenize_file(path: impl AsRef<Path>) -> anyhow::Result<VecDeque<Token>> {
        let mut file = fs::OpenOptions::new().read(true).open(path.as_ref())?;

        let mut source = String::new();
        file.read_to_string(&mut source)?;

        Self::tokenize(source)
    }

    pub fn tokenize(source: impl AsRef<str>) -> anyhow::Result<VecDeque<Token>> {
        Ok(source
            .as_ref()
            .replace("(", " ( ")
            .replace(")", " ) ")
            .split_whitespace()
            .map(|x| match x.to_ascii_lowercase().as_str() {
                "(" => Token::LParen,
                ")" => Token::RParen,
                "lambda" => Token::Lambda,
                "apply" => Token::Apply,
                "define" => Token::Define,
                "cond" => Token::Cond,
                "t" => Token::True,
                "f" => Token::False,
                "nil" => Token::Nil,
                _ => {
                    if let Ok(i) = x.parse::<i32>() {
                        Token::Integer(i)
                    } else {
                        Token::Symbol(x.into())
                    }
                }
            })
            .collect())
    }
}
