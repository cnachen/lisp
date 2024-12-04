use crate::lexer::Token;

type ExprField = Box<Expr>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Atom(Token),
    Composed { car: ExprField, cdr: ExprField },
}

impl Expr {
    pub const fn new_atom(atom: Token) -> Self {
        Self::Atom(atom)
    }

    pub fn new_composed(car: Expr, cdr: Expr) -> Self {
        Self::Composed {
            car: Box::new(car),
            cdr: Box::new(cdr),
        }
    }
}

pub mod consts {
    use super::{Expr, Token};

    /// = Expr::Atom(Token::Nil)
    pub const NIL: Expr = Expr::new_atom(Token::Nil);
    /// = Expr::Atom(Token::True)
    pub const TRUE: Expr = Expr::new_atom(Token::True);
    /// = Expr::Atom(Token::False)
    pub const FALSE: Expr = Expr::new_atom(Token::False);

    /// = Expr::Atom(Token::Lambda)
    pub const LAMBDA: Expr = Expr::new_atom(Token::Lambda);
    /// = Expr::Atom(Token::Apply)
    pub const APPLY: Expr = Expr::new_atom(Token::Apply);
    /// = Expr::Atom(Token::Define)
    pub const DEFINE: Expr = Expr::new_atom(Token::Define);
    /// = Expr::Atom(Token::Cond)
    pub const COND: Expr = Expr::new_atom(Token::Cond);
}

pub mod builtins {
    use super::consts::*;
    use super::{Expr, Token};

    pub fn cons(lhs: Expr, rhs: Expr) -> Expr {
        Expr::new_composed(lhs, rhs)
    }

    pub fn eq(lhs: Expr, rhs: Expr) -> Expr {
        match (lhs, rhs) {
            (Expr::Atom(lhs), Expr::Atom(rhs)) if lhs == rhs => TRUE,
            _ => FALSE,
        }
    }

    pub fn car(expr: Expr) -> Expr {
        match expr {
            Expr::Composed { car, .. } => *car,
            _ => NIL,
        }
    }

    pub fn cdr(expr: Expr) -> Expr {
        match expr {
            Expr::Composed { cdr, .. } => *cdr,
            _ => NIL,
        }
    }

    pub fn atom(expr: Expr) -> Expr {
        match expr {
            Expr::Atom(_) => TRUE,
            _ => FALSE,
        }
    }

    pub fn null(expr: Expr) -> Expr {
        match expr {
            Expr::Atom(Token::Nil) => TRUE,
            _ => FALSE,
        }
    }

    /// Simply return expr itself
    ///
    /// Processed later in eval phase
    #[inline]
    pub fn quote(expr: Expr) -> Expr {
        expr
    }

    /// Simply return expr itself
    ///
    /// Processed later in eval phase
    #[inline]
    pub fn eval(expr: Expr) -> Expr {
        expr
    }
}

pub mod math {
    use super::consts::*;
    use super::{Expr, Token};

    pub fn add(lhs: Expr, rhs: Expr) -> Expr {
        match (lhs, rhs) {
            (Expr::Atom(Token::Integer(lhs)), Expr::Atom(Token::Integer(rhs))) => {
                Expr::Atom(Token::Integer(lhs + rhs))
            }
            _ => NIL,
        }
    }

    pub fn sub(lhs: Expr, rhs: Expr) -> Expr {
        match (lhs, rhs) {
            (Expr::Atom(Token::Integer(lhs)), Expr::Atom(Token::Integer(rhs))) => {
                Expr::Atom(Token::Integer(lhs - rhs))
            }
            _ => NIL,
        }
    }

    pub fn mul(lhs: Expr, rhs: Expr) -> Expr {
        match (lhs, rhs) {
            (Expr::Atom(Token::Integer(lhs)), Expr::Atom(Token::Integer(rhs))) => {
                Expr::Atom(Token::Integer(lhs * rhs))
            }
            _ => NIL,
        }
    }

    pub fn div(lhs: Expr, rhs: Expr) -> Expr {
        match (lhs, rhs) {
            (Expr::Atom(Token::Integer(lhs)), Expr::Atom(Token::Integer(rhs))) => {
                Expr::Atom(Token::Integer(lhs / rhs))
            }
            _ => NIL,
        }
    }
}

pub mod intrinsics {
    use std::collections::HashSet;
    use std::sync::LazyLock;

    use super::builtins::*;
    use super::{Expr, Token};

    /// Append to a S-Expression if it's not empty
    pub fn append(lhs: Expr, rhs: Expr) -> Expr {
        if let Expr::Atom(Token::Nil) = lhs {
            rhs
        } else {
            cons(lhs, rhs)
        }
    }

    /// Get first atom in a S-Expression
    pub fn left_most(expr: Expr) -> Expr {
        match expr {
            Expr::Atom(_) => expr,
            _ => left_most(car(expr)),
        }
    }

    /// Flatten a S-Expression to atoms vector
    pub fn flatten(expr: Expr) -> Vec<Expr> {
        let mut exprs = Vec::new();

        match expr {
            Expr::Atom(_) => exprs.push(expr),
            _ => {
                exprs.extend(flatten(car(expr.clone())));
                exprs.extend(flatten(cdr(expr)));
            }
        }

        exprs
    }

    /// Gather parameters of Token::Cond | Token::Apply
    pub fn collect(expr: Expr) -> Vec<Expr> {
        let mut bottom = false;
        collect_(expr, &mut bottom)
    }

    fn collect_(expr: Expr, bottom: &mut bool) -> Vec<Expr> {
        let mut exprs = Vec::new();

        match expr {
            Expr::Atom(Token::Cond) | Expr::Atom(Token::Apply) => {
                *bottom = true;
            }
            _ if *bottom => exprs.push(expr),
            _ => {
                exprs.extend(collect_(car(expr.clone()), bottom));
                exprs.extend(collect_(cdr(expr), bottom));
            }
        }

        exprs
    }

    /// Check if symbol is a unary operator
    pub fn is_unary(expr: &Expr) -> bool {
        match expr {
            Expr::Atom(ref sym) if UNARIES.contains(sym) => true,
            _ => false,
        }
    }

    /// Check if symbol is a binary operator
    pub fn is_binary(expr: &Expr) -> bool {
        match expr {
            Expr::Atom(ref sym) if BINARIES.contains(sym) => true,
            _ => false,
        }
    }

    static UNARIES: LazyLock<HashSet<Token>> = LazyLock::new(|| {
        let mut set = HashSet::new();
        set.insert(Token::Symbol("car".into()));
        set.insert(Token::Symbol("cdr".into()));
        set.insert(Token::Symbol("atom".into()));
        set.insert(Token::Symbol("null".into()));
        set.insert(Token::Symbol("quote".into()));
        set.insert(Token::Symbol("eval".into()));
        set
    });

    static BINARIES: LazyLock<HashSet<Token>> = LazyLock::new(|| {
        let mut set = HashSet::new();
        set.insert(Token::Symbol("cons".into()));
        set.insert(Token::Symbol("eq".into()));
        set.insert(Token::Symbol("add".into()));
        set.insert(Token::Symbol("sub".into()));
        set.insert(Token::Symbol("mul".into()));
        set.insert(Token::Symbol("div".into()));
        set.insert(Token::Symbol("+".into()));
        set.insert(Token::Symbol("-".into()));
        set.insert(Token::Symbol("*".into()));
        set.insert(Token::Symbol("/".into()));
        set
    });
}
