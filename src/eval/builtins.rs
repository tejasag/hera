use super::object::*;
use std::collections::HashMap;

pub fn new_builtins() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("len"), Object::Builtin(monkey_len));
    builtins
}

fn monkey_len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "expected arguments: 1\ngiven arguments: {}",
            args.len()
        ));
    }
    match &args[0] {
        Object::String(s) => Object::Int(s.len() as i32),
        o => Object::Error(format!("argument to `len` not supported, got {}", o)),
    }
}
