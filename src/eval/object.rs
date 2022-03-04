use super::env::Env;
use crate::ast::{BlockStatement, Ident};
use std::{cell::RefCell, fmt, rc::Rc};

pub type BuiltlnFn = fn(Vec<Object>) -> Object;

#[derive(PartialEq, Clone, Debug)]
pub enum Object {
    Int(i32),
    String(String),
    Bool(bool),
    Null,
    Return(Box<Object>),
    Error(String),
    Fn(Vec<Ident>, BlockStatement, Rc<RefCell<Env>>),
    Builtin(BuiltlnFn),
    Array(Vec<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Int(ref value) => write!(f, "{}", value),
            Object::String(ref value) => write!(f, "{}", value),
            Object::Bool(ref value) => write!(f, "{}", value),
            Object::Null => write!(f, "null"),
            Object::Return(ref value) => write!(f, "{}", value),
            Object::Error(ref value) => write!(f, "{}", value),
            Object::Fn(ref params, _, _) => {
                let mut result = String::new();
                for (i, Ident(ref s)) in params.iter().enumerate() {
                    if i < 1 {
                        result.push_str(s);
                    } else {
                        result.push_str(&format!(", {}", s));
                    }
                }
                write!(f, "fn({}) {{ ... }}", result)
            }
            Object::Builtin(_) => write!(f, "[builtin fn]"),
            Object::Array(ref val) => {
                let mut result = String::new();
                for (i, obj) in val.iter().enumerate() {
                    if i < 1 {
                        result.push_str(&format!("{}", obj));
                    } else {
                        result.push_str(&format!(", {}", obj));
                    }
                }
                write!(f, "[{}]", result)
            }
        }
    }
}
