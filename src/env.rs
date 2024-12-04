use crate::expr::Expr;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Default, PartialEq)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, Expr>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Self {
        Self {
            vars: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get(&self, name: impl AsRef<str>) -> Option<Expr> {
        match self.vars.get(name.as_ref()) {
            Some(value) => Some(value.clone()),
            None => self
                .parent
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone()),
        }
    }

    pub fn set(&mut self, name: impl AsRef<str>, val: Expr) {
        self.vars.insert(name.as_ref().into(), val);
    }

    pub fn update(&mut self, data: Rc<RefCell<Self>>) {
        self.vars.extend(
            data.borrow()
                .vars
                .iter()
                .map(|(k, v)| (k.clone(), v.clone())),
        );
    }
}
