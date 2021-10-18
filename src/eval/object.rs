use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Object {
    Int(i32),
    String(String),
    Bool(bool),
    Null,
    Return(Box<Object>),
    Error(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Object::Int(ref value) => write!(f, "{}", value),
            Object::String(ref value) => write!(f, "{}", value),
            Object::Bool(ref value) => write!(f, "{}", value),
            Object::Null => write!(f, "null"),
            Object::Return(ref value) => write!(f, "{}", value),
            Object::Error(ref value) => write!(f, "ERROR: {}", value),
        }
    }
}
