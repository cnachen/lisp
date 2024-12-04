use lisp::{consts::*, Env, Evaluator, Expr, Token};
use std::{cell::RefCell, rc::Rc};

#[test]
fn arithmatic_test1() {
    let mut env = Rc::new(RefCell::new(Env::new()));
    assert_eq!(
        Evaluator::eval("(add 1 (sub 2 (mul 3 4)))", &mut env).unwrap(),
        Expr::new_atom(Token::Integer(-9))
    );
}

#[test]
fn arthmatic_test2() {
    let mut env = Rc::new(RefCell::new(Env::new()));
    assert_eq!(Evaluator::eval("(eq 1 (+ -2 3))", &mut env).unwrap(), TRUE);
}

#[test]
fn cond_test() {
    let mut env = Rc::new(RefCell::new(Env::new()));
    assert_eq!(
        Evaluator::eval("(cond ((eq 3 (+ 1 2)) t) (f f))", &mut env).unwrap(),
        TRUE
    );
    assert_eq!(
        Evaluator::eval("(cond ((eq 4 (+ 1 2)) t) (t f))", &mut env).unwrap(),
        FALSE
    );
}

#[test]
fn function_test() {
    let mut env = Rc::new(RefCell::new(Env::new()));
    assert_eq!(
        Evaluator::eval(
            "(cons (define ZERO (lambda (x) (cond ((eq x 0) t) (t f)))) (apply ZERO 0))",
            &mut env
        )
        .unwrap(),
        Expr::new_composed(NIL, TRUE)
    );
    assert_eq!(
        Evaluator::eval(
            "(cons (define ZERO (lambda (x) (cond ((eq x 0) t) (t f)))) (apply ZERO 1))",
            &mut env
        )
        .unwrap(),
        Expr::new_composed(NIL, FALSE)
    );
}

#[test]
fn rec_test() {
    let mut env = Rc::new(RefCell::new(Env::new()));
    assert_eq!(
        Evaluator::eval(
            "(cons (define SUM (lambda (x) (cond ((eq x 0) 0) (t (+ x (apply SUM (- x 1))))))) (apply SUM 23))",
            &mut env
        )
        .unwrap(),
        Expr::new_composed(NIL, Expr::Atom(Token::Integer(276)))
    );
}
