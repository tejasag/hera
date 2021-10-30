use super::object::Object;
use std::collections::HashMap;

#[derive(PartialEq, Clone, Debug)]
pub struct Env {
    store: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&mut self, name: &str) -> Option<Object> {
        println!("{:#?}", self.store);
        println!("name is: {}", &name);
        match self.store.get(name) {
            Some(e) => {
                println!("found! {:?}", e);
                Some(e).cloned()
            }
            None => {
                println!("not found :/");
                None
            }
        }
    }

    pub fn set(&mut self, name: String, val: Object) -> Option<Object> {
        println!("{:#?}", self.store);
        println!("val: {:?}\nname: {:?}", val, name);
        match self.store.insert(name.clone(), val) {
            Some(e) => Some(e),
            None => {
                println!("bruh");
                println!("updated store: {:#?}", self.store);
                println!("get value: {:#?}", self.get(&name));
                None
            }
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Self::new()
    }
}
