use crate::env::Env;
use crate::expr::Expr;
use crate::parser::Parser;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

pub struct Evaluator;

impl Evaluator {
    pub fn eval_file(path: impl AsRef<Path>, env: &mut Rc<RefCell<Env>>) -> anyhow::Result<Expr> {
        let expr = Parser::parse_file(path)?;
        eval_state::eval_expr(expr, env)
    }

    pub fn eval(source: impl AsRef<str>, env: &mut Rc<RefCell<Env>>) -> anyhow::Result<Expr> {
        let expr = Parser::parse(source)?;
        eval_state::eval_expr(expr, env)
    }
}

mod eval_state {
    use super::{Env, Expr, Rc, RefCell};
    use crate::{builtins::*, consts::*, intrinsics::*, math::*, Token};

    pub fn eval_expr(expr: Expr, env: &mut Rc<RefCell<Env>>) -> anyhow::Result<Expr> {
        match expr {
            Expr::Atom(Token::Symbol(_)) => eval_symbol(expr, env),
            Expr::Atom(_) => Ok(expr),
            _ => match left_most(expr.clone()) {
                atom @ Expr::Atom(_) if is_unary(&atom) => eval_unary(atom, cdr(expr), env),
                atom @ Expr::Atom(_) if is_binary(&atom) => {
                    eval_binary(atom, cdr(car(expr.clone())), cdr(expr), env)
                }
                Expr::Atom(Token::Apply) => eval_apply(expr, env),
                Expr::Atom(Token::Define) => eval_define(cdr(car(expr.clone())), cdr(expr), env),
                Expr::Atom(Token::Cond) => eval_cond(expr, env),
                _ => Ok(NIL),
            },
        }
    }

    pub fn eval_symbol(expr: Expr, env: &mut Rc<RefCell<Env>>) -> anyhow::Result<Expr> {
        match expr {
            Expr::Atom(Token::Symbol(ref sym)) => match env.borrow().get(sym) {
                Some(val) => return Ok(val),
                _ => panic!("Symbol `{}` not defined", sym),
            },
            _ => panic!("Internal error"),
        }
    }

    pub fn eval_unary(op: Expr, expr: Expr, env: &mut Rc<RefCell<Env>>) -> anyhow::Result<Expr> {
        match op {
            Expr::Atom(Token::Symbol(ref sym)) => match sym.as_str() {
                "car" => return Ok(car(eval_expr(expr, env)?)),
                "cdr" => return Ok(cdr(eval_expr(expr, env)?)),
                "atom" => return Ok(atom(eval_expr(expr, env)?)),
                "null" => return Ok(null(eval_expr(expr, env)?)),
                "quote" => return Ok(quote(expr)),
                "eval" => return Ok(eval(eval_expr(eval_expr(expr, env)?, env)?)),
                _ => panic!("Bad Token::Symbol({})", sym),
            },
            _ => panic!("Expect Token::Symbol"),
        }
    }

    pub fn eval_binary(
        op: Expr,
        lhs: Expr,
        rhs: Expr,
        env: &mut Rc<RefCell<Env>>,
    ) -> anyhow::Result<Expr> {
        let lhs = eval_expr(lhs, env)?;
        let rhs = eval_expr(rhs, env)?;

        match op {
            Expr::Atom(Token::Symbol(ref sym)) => match sym.as_str() {
                "cons" => return Ok(cons(lhs, rhs)),
                "eq" => return Ok(eq(lhs, rhs)),
                "add" | "+" => return Ok(add(lhs, rhs)),
                "sub" | "-" => return Ok(sub(lhs, rhs)),
                "mul" | "*" => return Ok(mul(lhs, rhs)),
                "div" | "/" => return Ok(div(lhs, rhs)),
                _ => panic!("Bad Token::Symbol({})", sym),
            },
            Expr::Atom(Token::Lambda) => Ok(NIL),
            _ => panic!("Expect Token::Symbol | Token::Lambda"),
        }
    }

    pub fn eval_apply(expr: Expr, env: &mut Rc<RefCell<Env>>) -> anyhow::Result<Expr> {
        let exprs = collect(expr);

        match exprs[0] {
            Expr::Atom(Token::Symbol(ref sym)) => match env.clone().borrow().get(sym) {
                Some(lambda) => {
                    let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
                    let args = flatten(cdr(car(lambda.clone())));
                    for iarg in 0..args.len() {
                        match args[iarg] {
                            Expr::Atom(Token::Symbol(ref arg_sym)) => new_env
                                .borrow_mut()
                                .set(arg_sym, eval_expr(exprs[iarg + 1].clone(), env)?),
                            _ => panic!("Lambda argument is not Token::Symbol"),
                        }
                    }
                    return eval_expr(cdr(lambda), &mut new_env);
                }
                _ => panic!("Callable symbol `{}` not defined", sym),
            },
            _ => panic!("Expect Token::Symbol"),
        }
    }

    pub fn eval_define(name: Expr, expr: Expr, env: &mut Rc<RefCell<Env>>) -> anyhow::Result<Expr> {
        match name {
            Expr::Atom(Token::Symbol(ref sym)) => {
                env.borrow_mut().set(sym, expr);
                return Ok(NIL);
            }
            _ => panic!("Expect Token::Symbol, found {:?}", name),
        }
    }

    pub fn eval_cond(expr: Expr, env: &mut Rc<RefCell<Env>>) -> anyhow::Result<Expr> {
        Ok(collect(expr)
            .into_iter()
            .find_map(|expr| match eval_expr(car(expr.clone()), env).ok()? {
                Expr::Atom(Token::True) => eval_expr(cdr(expr), env).ok(),
                _ => None,
            })
            .unwrap_or(NIL))
    }
}
