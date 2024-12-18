use std::{collections::HashMap, default};

pub enum Value {
    Number(f64)
}


pub enum Binding {
    Value(Value)
}


pub struct Environment {
    bindings: HashMap<String, Binding>
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            bindings: HashMap::new()
        }
    }

    pub fn bind(&mut self, name: String, binding: Binding) {
        self.bindings.insert(name, binding);
    }

    pub fn lookup(&self, name: &String) -> Option<&Binding> {
        self.bindings.get(name)
    }
}
