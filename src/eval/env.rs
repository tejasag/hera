use super::object::Object;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(PartialEq, Clone, Debug)]
pub struct Env {
    pub store: HashMap<String, Object>,
    pub outer: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn from(store: HashMap<String, Object>) -> Self {
        Self { store, outer: None }
    }

    pub fn new_enclosed(outer: Rc<RefCell<Env>>) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn get(&mut self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(e) => Some(e).cloned(),
            None => {
                if let Some(ref o) = self.outer {
                    return o.borrow_mut().get(name);
                } else {
                    None
                }
            }
        }
    }

    pub fn set(&mut self, name: String, val: Object) -> Option<Object> {
        self.store.insert(name, val)
    }

    pub fn update(&mut self, name: String, val: Object) -> Option<Object> {
        match self.store.get(&name) {
            Some(_) => self.store.insert(name, val),
            None => {
                if let Some(ref o) = self.outer {
                    let mut outer = o.borrow_mut();
                    match outer.get(&name) {
                        Some(_) => outer.update(name, val),
                        None => None,
                    }
                } else {
                    None
                }
            }
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
