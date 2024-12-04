use crate::consts::*;
use crate::expr::Expr;
use crate::intrinsics::*;
use crate::lexer::Lexer;
use crate::lexer::Token;
use std::collections::VecDeque;
use std::convert::AsRef;
use std::path::Path;

pub struct Parser;

impl Parser {
    pub fn parse_file(path: impl AsRef<Path>) -> anyhow::Result<Expr> {
        let mut tokens = Lexer::tokenize_file(path)?;
        Self::parse_tokens(&mut tokens)
    }

    pub fn parse(source: impl AsRef<str>) -> anyhow::Result<Expr> {
        let mut tokens = Lexer::tokenize(source)?;
        Self::parse_tokens(&mut tokens)
    }

    fn parse_tokens(tokens: &mut VecDeque<Token>) -> anyhow::Result<Expr> {
        let token = tokens.pop_front();

        if token != Some(Token::LParen) {
            panic!("Expect Token::LParen");
        }

        let mut expr = NIL;

        while !tokens.is_empty() {
            let token = tokens.pop_front().unwrap();

            match token {
                Token::LParen => {
                    tokens.push_front(Token::LParen);
                    let sub_expr = Self::parse_tokens(tokens)?;
                    expr = append(expr, sub_expr);
                }
                Token::RParen => return Ok(expr),
                Token::Lambda => expr = append(expr, LAMBDA),
                Token::Apply => expr = append(expr, APPLY),
                Token::Define => expr = append(expr, DEFINE),
                Token::Cond => expr = append(expr, COND),
                Token::True => expr = append(expr, TRUE),
                Token::False => expr = append(expr, FALSE),
                Token::Nil => expr = append(expr, NIL),
                Token::Integer(n) => expr = append(expr, Expr::new_atom(Token::Integer(n))),
                Token::Symbol(ref sym) => {
                    expr = append(expr, Expr::new_atom(Token::Symbol(sym.into())))
                }
                #[allow(unreachable_patterns)]
                _ => unimplemented!("Unimplemented Token::{:?}", token),
            }
        }
        panic!("Expect Token::RParen");
    }
}
