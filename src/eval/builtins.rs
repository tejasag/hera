use super::object::*;
use std::collections::HashMap;

pub fn new_builtins() -> HashMap<String, Object> {
    let mut builtins = HashMap::new();
    builtins.insert(String::from("len"), Object::Builtin(monkey_len));
    builtins.insert(String::from("tail"), Object::Builtin(monkey_tail));
    builtins.insert(String::from("push"), Object::Builtin(monkey_push));
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
        Object::Array(a) => Object::Int(a.len() as i32),
        o => Object::Error(format!("argument to `len` not supported, got {}", o)),
    }
}

fn monkey_tail(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "expected arguments: 1\ngiven arguments: {}",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(a) => Object::Array(a[1..].to_vec()),
        o => Object::Error(format!("argument to `tail` not supported, got {}", o)),
    }
}

fn monkey_push(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "expected arguments: 1\ngiven arguments: {}",
            args.len()
        ));
    }

    match &args[0] {
        Object::Array(a) => {
            let mut array = a.clone();
            array.push(args[1].clone());
            Object::Array(array)
        }
        o => Object::Error(format!("argument to `push` not supported, got {}", o)),
    }
}
