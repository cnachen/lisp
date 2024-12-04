use std::cell::RefCell;
use std::rc::Rc;

use lisp::Env;
use lisp::Evaluator;

fn main() -> anyhow::Result<()> {
    let mut env = Rc::new(RefCell::new(Env::new()));

    println!(
        "{:#?}",
        Evaluator::eval("(cond ((eq (+ 0 1) 1) 0) (t 1))", &mut env)
    );

    Ok(())
}
