use jsonata_error::Result;
use std::collections::HashMap;

pub struct Function<T> {
    // arity: usize,
    pub implementation: Box<dyn Fn(Vec<T>) -> Result<T>>, 
}
    

pub enum Binding<T> {
    Value(T),
    Function(Function<T>)
}


pub struct Environment<T> {
    bindings: HashMap<String, Binding<T>>
}

impl<T> Environment<T>
{
    pub fn new() -> Self {
        Environment {
            bindings: HashMap::new()
        }
    }

    pub fn bind(&mut self, name: String, binding: Binding<T>) {
        self.bindings.insert(name, binding);
    }

    pub fn lookup(&self, name: &String) -> Option<&Binding<T>> {
        self.bindings.get(name)
    }
}
