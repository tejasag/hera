use super::{env::Env, object::*, Eval};
use crate::{lexer::Lexer, parser::Parser};
use std::{cell::RefCell, collections::HashMap, fs, rc::Rc};

pub fn load_lib(lib: String) -> Option<HashMap<String, Object>> {
    let libs = vec!["std"];
    let mut methods = HashMap::new();
    methods.insert("std", vec!["map", "first", "last", "while"]);

    if !libs.contains(&lib.as_str()) {
        return None;
    }
    let file = fs::read_to_string(format!("./libraries/{}.hera", lib)).expect("Library not found.");
    let mut parser = Parser::new(Lexer::new(file));
    let program = parser.parse_program();
    if !parser.errors.is_empty() {
        for e in parser.errors.iter() {
            println!("\t{}", e);
        }
        return None;
    };
    let mut eval = Eval::new(Rc::new(RefCell::new(Env::new())));
    eval.eval(program);

    let store = (&*eval.env.borrow()).to_owned().store;
    let mut final_env = HashMap::new();
    match methods.get(lib.as_str()) {
        Some(m) => m
            .to_owned()
            .iter()
            .map(|&method| {
                final_env.insert(method.to_string(), store.get(method).unwrap().to_owned())
            })
            .collect::<Vec<_>>(),
        None => vec![],
    };
    Some(final_env)
}
